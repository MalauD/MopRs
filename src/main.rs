use actix_files::{Files, NamedFile};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{
    web::{self, Data},
    App, HttpRequest, HttpServer, Result,
};
use routes::{config_music, config_user};
use std::sync::RwLock;

use crate::{deezer::DeezerClient, models::Sessions};

mod db;
mod deezer;
mod models;
mod routes;
mod tools;

async fn index(req: HttpRequest) -> Result<NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const PORT: i32 = 8080;

    let sessions: Data<RwLock<Sessions>> = Data::new(RwLock::new(Default::default()));
    let deezer_client = Data::new(DeezerClient::new("https://api.deezer.com/".to_string()));

    HttpServer::new(move || {
        App::new()
            .app_data(sessions.clone())
            .app_data(deezer_client.clone())
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
