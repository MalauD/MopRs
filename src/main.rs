use actix::Actor;
use actix_files::{Files, NamedFile};
use actix_identity::IdentityMiddleware;
use actix_session::{config::PersistentSession, storage::RedisSessionStore, SessionMiddleware};
use actix_web::{
    cookie::Key,
    middleware,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Result,
};
use actix_web_prom::PrometheusMetricsBuilder;
use dotenv::dotenv;
use log::{info, warn};
use routes::config_music;
use std::io;
use url::Url;

use crate::{
    actors::{ArtistScraperActor, DownloaderActor},
    app_settings::AppSettings,
    db::MongoClient,
    deezer::get_dz_downloader,
    s3::S3Client,
    search::{get_meilisearch, MeilisearchConfig},
};

mod actors;
mod app_settings;
mod db;
mod deezer;
mod handlers;
mod models;
mod routes;
mod s3;
mod search;
mod suggestions;
mod tools;

async fn index(_req: HttpRequest) -> Result<NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
}

async fn health(_req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("It's alive!"))
}

fn build_redis_connection_string(config: &AppSettings) -> io::Result<String> {
    let mut url = Url::parse(&format!(
        "redis://{}:{}/",
        config.redis_service_host, config.redis_service_port,
    ))
    .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid Redis URL: {e}")))?;

    if let Some(username) = &config.redis_username {
        url.set_username(username).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Redis username contains unsupported characters",
            )
        })?;
    }

    if let Some(password) = &config.redis_password {
        url.set_password(Some(password)).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Redis password contains unsupported characters",
            )
        })?;
    }

    Ok(url.into())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init();
    info!(target:"mop-rs::main","Starting MopRs");
    const PORT: i32 = 8080;

    let config: AppSettings = envy::from_env().unwrap();
    let app_config = Data::new(config.clone());

    let secret_key = if let Some(key) = config.session_key.clone() {
        Key::from(key.as_bytes())
    } else {
        Key::generate()
    };

    let mut d = get_dz_downloader(Some(config.arl.clone())).write().unwrap();
    if let Err(e) = d.authenticate().await {
        warn!("Failed to authenticate to Deezer: {}", e);
    } else {
        info!(target:"mop-rs::deezer_downloader","Deezer downloader initialized");
    }

    drop(d);

    let redis_connection_string = build_redis_connection_string(&config)?;

    let redis_store = RedisSessionStore::new(redis_connection_string)
        .await
        .unwrap();

    info!(target:"mop-rs::redis","Connected to redis");

    let _ = get_meilisearch(Some(MeilisearchConfig::new(
        config.meilisearch_host.clone(),
        config.meilisearch_api_key.clone(),
    )))
    .await;

    let db = Data::new(
        MongoClient::connect(&config.mongo_url)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to connect to MongoDB: {e}")))?,
    );
    db.initialize_indexes().await;

    let s3 = Data::new(S3Client::connect(&s3::S3Config {
        s3_url: config.s3_url.clone(),
        s3_region: config.s3_region.clone(),
        s3_bucket: config.s3_bucket.clone(),
    })
    .await?);

    let c = db.get_musics_count().await.unwrap();
    info!(target:"mop-rs::mongo","{} musics in database", c);

    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap();

    let addr = ArtistScraperActor::new(db.clone(), config.clone()).start();
    let downloader_addr = DownloaderActor::new(s3.clone(), config.clone()).start();
    let session_ttl = config.get_session_duration();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(prometheus.clone())
            .app_data(app_config.clone())
            .app_data(db.clone())
            .app_data(s3.clone())
            .app_data(Data::new(addr.clone()))
            .app_data(Data::new(downloader_addr.clone()))
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(redis_store.clone(), secret_key.clone())
                    .cookie_secure(false)
                    .cookie_name("mop-id".to_string())
                    .session_lifecycle(PersistentSession::default().session_ttl(session_ttl))
                    .build(),
            )
            .route("/", web::get().to(index)) // Redirect all react routes to index
            .route("/Login", web::get().to(index))
            .route("/Register", web::get().to(index))
            .route("/Favorites", web::get().to(index))
            .route("/History", web::get().to(index))
            .route("/Search", web::get().to(index))
            .route("/Player", web::get().to(index))
            .route("/Music", web::get().to(index))
            .route("/Music/{tail:.*}", web::get().to(index))
            .route("/Album", web::get().to(index))
            .route("/Album/{tail:.*}", web::get().to(index))
            .route("/Artist", web::get().to(index))
            .route("/Artist/{tail:.*}", web::get().to(index))
            .route("/Playlist", web::get().to(index))
            .route("/Playlist/{tail:.*}", web::get().to(index))
            .route("/User", web::get().to(index))
            .route("/User/{tail:.*}", web::get().to(index))
            .route("/Settings", web::get().to(index))
            .route("/health", web::get().to(health))
            .configure(config_music)
            .service(Files::new("/", "./static"))
    })
    .bind(format!("0.0.0.0:{}", PORT))?
    .run()
    .await
}
