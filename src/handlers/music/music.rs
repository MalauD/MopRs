use crate::{db::MongoClient, models::DeezerId};
use actix_web::{web, HttpResponse};

use super::MusicResponse;

pub async fn get_music(req: web::Path<DeezerId>, db: web::Data<MongoClient>) -> MusicResponse {
    let res = db.get_musics(&vec![*req]).await?;

    match res {
        Some(x) => Ok(HttpResponse::Ok().json(x[0].clone())),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}
