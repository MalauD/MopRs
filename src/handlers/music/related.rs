use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::json;

use crate::{
    db::get_mongo,
    models::{DeezerId, User},
    suggestions::get_related_to,
};

use super::MusicResponse;

fn get_default_related_limit() -> i32 {
    20
}

#[derive(Deserialize)]
pub struct RelMusicsReq {
    #[serde(rename = "MusicIds")]
    pub music_ids: Vec<DeezerId>,
    #[serde(rename = "Exclude", default = "Vec::default")]
    pub exclude: Vec<DeezerId>,
    #[serde(rename = "Limit", default = "get_default_related_limit")]
    pub limit: i32,
}

pub async fn get_related_musics(_user: User, pl: web::Json<RelMusicsReq>) -> MusicResponse {
    let db = get_mongo(None).await;
    let rel = get_related_to(&pl.music_ids, Some(&pl.exclude), 0.2, pl.limit).await;
    let pop_rel = db.get_musics(&rel).await?.unwrap_or_default();
    Ok(HttpResponse::Ok().json(&json!({ "RelatedMusics": pop_rel })))
}
