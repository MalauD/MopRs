use actix_files::{Files, NamedFile};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{
    web::{self, Data},
    App, HttpRequest, HttpServer, Result,
};
use dotenv::dotenv;
use log::info;
use routes::{config_music, config_user};
use std::fs;

use crate::{app_settings::AppSettings, db::get_mongo, deezer::get_dz_client};

mod app_settings;
mod db;
mod deezer;
mod models;
mod routes;
mod tools;

async fn index(_req: HttpRequest) -> Result<NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init();
    info!(target:"mop-rs::main","Starting MopRs");
    const PORT: i32 = 8080;

    let config: AppSettings = envy::from_env().unwrap();

    let _ = fs::create_dir_all(config.music_path.clone());

    let _ = get_dz_client(Some(config.arl.clone())).await;

    let db = get_mongo(Some(config.mongo_url.clone())).await;
    let c = db.get_musics_count().await.unwrap();
    info!(target:"mop-rs::mongo","{} musics in database", c);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(config.clone()))
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("mop-id")
                    .secure(false),
            ))
            .route("/", web::get().to(index))
            .configure(config_user)
            .configure(config_music)
            .service(Files::new("/", "./static"))
    })
    .bind(format!("0.0.0.0:{}", PORT))?
    .run()
    .await
}
