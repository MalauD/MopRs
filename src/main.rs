use actix::Actor;
use actix_files::{Files, NamedFile};
use actix_identity::IdentityMiddleware;
use actix_session::{config::PersistentSession, storage::RedisSessionStore, SessionMiddleware};
use actix_web::{
    cookie::Key,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Result,
};
use dotenv::dotenv;
use log::info;
use routes::{config_music, config_user};

use crate::{
    actors::ArtistScraperActor,
    app_settings::{get_settings, AppSettings},
    db::get_mongo,
    deezer::get_dz_client,
    s3::get_s3,
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
mod suggestion;
mod tools;

async fn index(_req: HttpRequest) -> Result<NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
}

async fn health(_req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("It's alive!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init();
    info!(target:"mop-rs::main","Starting MopRs");
    const PORT: i32 = 8080;

    let config: AppSettings = envy::from_env().unwrap();
    let _ = get_settings(Some(config.clone())).await;

    let secret_key = if let Some(key) = config.session_key.clone() {
        Key::from(key.as_bytes())
    } else {
        Key::generate()
    };

    let redis_config = config.clone();
    let redis_connection_string = if let Some(redis_pasword) = redis_config.redis_password {
        format!(
            "redis://{}:{}@{}:{}",
            redis_config.redis_username.unwrap_or("default".to_string()),
            redis_pasword,
            redis_config.redis_service_host,
            redis_config.redis_service_port,
        )
    } else {
        format!(
            "redis://{}:{}",
            redis_config.redis_service_host, redis_config.redis_service_port,
        )
    };

    let redis_store = RedisSessionStore::new(redis_connection_string)
        .await
        .unwrap();

    info!(target:"mop-rs::redis","Connected to redis");

    let _ = get_dz_client(Some(config.arl.clone())).await;

    let _ = get_meilisearch(Some(MeilisearchConfig::new(
        config.meilisearch_host.clone(),
        config.meilisearch_api_key.clone(),
    )))
    .await;

    let db = get_mongo(Some(config.mongo_url.clone())).await;

    let _ = get_s3(Some(config.s3_url.clone())).await;

    let c = db.get_musics_count().await.unwrap();
    info!(target:"mop-rs::mongo","{} musics in database", c);

    let addr = ArtistScraperActor {}.start();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(config.clone()))
            .app_data(Data::new(addr.clone()))
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(redis_store.clone(), secret_key.clone())
                    .cookie_secure(false)
                    .cookie_name("mop-id".to_string())
                    .session_lifecycle(
                        PersistentSession::default().session_ttl(config.get_session_duration()),
                    )
                    .build(),
            )
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health))
            .configure(config_user)
            .configure(config_music)
            .service(Files::new("/", "./static"))
    })
    .bind(format!("0.0.0.0:{}", PORT))?
    .run()
    .await
}
