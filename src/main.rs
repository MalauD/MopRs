use actix_files::{Files, NamedFile};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{
    web::{self, Data},
    App, HttpRequest, HttpServer, Result,
};
use log::info;
use routes::{config_music, config_user};
use std::{fs, sync::RwLock};

use crate::{db::get_mongo, deezer::DeezerClient, models::Sessions};

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
    env_logger::init();
    info!(target:"mop-rs::main","Starting MopRs");
    const PORT: i32 = 8080;
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Settings")).unwrap();

    let _ = fs::create_dir_all(settings.get_str("music_path").unwrap());

    let arl = settings.get_str("arl").unwrap();

    let sessions: Data<RwLock<Sessions>> = Data::new(RwLock::new(Default::default()));
    let deezer_client = Data::new(RwLock::new(DeezerClient::new(
        "https://api.deezer.com/".to_string(),
        arl,
    )));
    {
        info!(target:"mop-rs::deezer","Initializing deezer client");
        let mut cl = deezer_client.write().unwrap();
        let _ = cl.init_session().await;
        let _ = cl.init_user().await;
    }

    let db = get_mongo().await;
    let c = db.get_musics_count().await.unwrap();
    info!(target:"mop-rs::mongo","{} musics in database", c);

    HttpServer::new(move || {
        App::new()
            .app_data(sessions.clone())
            .app_data(deezer_client.clone())
            .app_data(Data::new(app_settings::AppSettings::new(
                settings.get_str("music_path").unwrap(),
            )))
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
