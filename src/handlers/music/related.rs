use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::json;

use crate::{db::get_mongo, models::User, suggestion::get_related_to};

use super::MusicResponse;

#[derive(Deserialize)]
pub struct RelMusicsReq {
    #[serde(rename = "MusicIds")]
    pub music_ids: Vec<i32>,
}

pub async fn get_related_musics(_user: User, pl: web::Json<RelMusicsReq>) -> MusicResponse {
    let db = get_mongo(None).await;
    let rel = get_related_to(&pl.music_ids, 20).await;
    let pop_rel = db.get_musics(&rel).await?.unwrap_or_default();
    Ok(HttpResponse::Ok().json(&json!({ "RelatedMusics": pop_rel })))
}
