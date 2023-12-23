use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::{
    db::{get_mongo, PaginationOptions},
    deezer::DeezerMusicFormats,
    models::{PublicUser, User},
    suggestions::get_suggestions_for,
};

use super::UserResponse;

pub async fn get_account(user: User) -> impl Responder {
    let db = get_mongo(None).await;
    let u = db.get_user(&user.id().unwrap()).await.unwrap().unwrap();
    web::Json(json!({ "Account": PublicUser::from(u) }))
}

pub async fn get_liked(pagination: web::Query<PaginationOptions>, user: User) -> UserResponse {
    let db = get_mongo(None).await;
    let u = db.get_user(&user.id().unwrap()).await.unwrap().unwrap();
    let mut musics = u.liked_musics().to_vec();
    musics.reverse();
    let res = db.get_musics(&pagination.trim_vec(&musics)).await.unwrap();
    Ok(HttpResponse::Ok().json(res))
}

pub async fn get_viewed(pagination: web::Query<PaginationOptions>, user: User) -> UserResponse {
    let db = get_mongo(None).await;
    let u = db.get_user(&user.id().unwrap()).await.unwrap().unwrap();
    let mut musics = u.viewed_musics().to_vec();
    musics.reverse();
    let res = db.get_musics(&pagination.trim_vec(&musics)).await.unwrap();
    Ok(HttpResponse::Ok().json(res))
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuggestionSettings {
    memory: i64,
    like_hist_ratio: f32,
    novelty: f32,
    limit: i32,
}
pub async fn get_suggestions(user: User, settings: web::Query<SuggestionSettings>) -> UserResponse {
    let db = get_mongo(None).await;

    let sugg = get_suggestions_for(
        user,
        settings.memory.min(300).max(10),
        settings.like_hist_ratio.min(1.0).max(0.0),
        settings.novelty.max(10.0).max(0.0),
        settings.limit.min(100).max(1),
    )
    .await;

    let res = db.get_musics(&sugg).await.unwrap();

    Ok(HttpResponse::Ok().json(res))
}

#[derive(serde::Deserialize)]
pub struct PreferedFormat {
    pub format: DeezerMusicFormats,
}

pub async fn set_prefered_format(
    user: User,
    format: web::Json<PreferedFormat>,
) -> actix_web::Result<HttpResponse> {
    let db = get_mongo(None).await;
    db.set_prefered_format(&user, &format.format).await.unwrap();
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_prefered_format(user: User) -> actix_web::Result<HttpResponse> {
    let db = get_mongo(None).await;
    let user = db.get_user(&user.id().unwrap()).await.unwrap().unwrap();
    Ok(HttpResponse::Ok().json(user.prefered_format().to_string()))
}
