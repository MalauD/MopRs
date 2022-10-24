use actix_files::{Files, NamedFile};
use actix_identity::IdentityMiddleware;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{
    cookie::Key,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Result,
};
use dotenv::dotenv;
use log::info;
use routes::{config_music, config_user};

use crate::{app_settings::AppSettings, db::get_mongo, deezer::get_dz_client, s3::get_s3};

mod app_settings;
mod db;
mod deezer;
mod handlers;
mod models;
mod routes;
mod s3;
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

    let secret_key = Key::generate();

    let redis_store = RedisSessionStore::new(
        format!(
            "redis://{}:{}",
            config.redis_service_host, config.redis_service_port,
        )
        .as_str(),
    )
    .await
    .unwrap();

    info!(target:"mop-rs::redis","Connected to redis");

    let _ = get_dz_client(Some(config.arl.clone())).await;

    let db = get_mongo(Some(config.mongo_url.clone())).await;

    let _ = get_s3(Some(config.s3_url.clone())).await;

    let c = db.get_musics_count().await.unwrap();
    info!(target:"mop-rs::mongo","{} musics in database", c);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(config.clone()))
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
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
