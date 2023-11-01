use crate::{db::get_mongo, models::DeezerId};
use actix_web::{web, HttpResponse};

use super::MusicResponse;

pub async fn get_music(req: web::Path<DeezerId>) -> MusicResponse {
    let db = get_mongo(None).await;
    let res = db.get_musics(&vec![*req]).await?;

    match res {
        Some(x) => Ok(HttpResponse::Ok().json(x[0].clone())),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}
