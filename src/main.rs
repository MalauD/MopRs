use actix_files::{Files, NamedFile};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{
    web::{self, Data},
    App, HttpRequest, HttpServer, Result,
};
use routes::{config_music, config_user};
use std::{path::Path, sync::RwLock};

use crate::{deezer::DeezerClient, models::Sessions};

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
    const PORT: i32 = 8080;
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Settings")).unwrap();

    let arl = settings.get_str("arl").unwrap();

    let sessions: Data<RwLock<Sessions>> = Data::new(RwLock::new(Default::default()));
    let deezer_client = Data::new(RwLock::new(DeezerClient::new(
        "https://api.deezer.com/".to_string(),
        arl,
    )));
    {
        let mut cl = deezer_client.write().unwrap();
        let _ = cl.init_session().await;
        println!("Sid: {}", cl.cred.sid);
        let _ = cl.init_user().await;
        println!("Token: {}", cl.cred.token);
        let m = cl.get_music_by_id_unofficial(350171311).await.unwrap();
        println!("Music: {}", m.get_url());
        println!("Bf key {}", m.get_bf_key());
        cl.download_music(350171311, Path::new("./Musics")).await;
    }

    HttpServer::new(move || {
        App::new()
            .app_data(sessions.clone())
            .app_data(deezer_client.clone())
            .wrap(actix_web::middleware::Compress::new(
                actix_web::http::ContentEncoding::Auto,
            ))
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
