use actix_web::{web, HttpResponse};
use serde_json::json;

use crate::{
    db::{MongoClient, PaginationOptions},
    deezer::DeezerMusicFormats,
    models::{PublicUser, User},
    suggestions::get_suggestions_for,
};

use super::UserResponse;

fn current_user_id(user: &User) -> Option<mongodb::bson::oid::ObjectId> {
    user.id()
}

pub async fn get_account(user: User, db: web::Data<MongoClient>) -> UserResponse {
    let Some(user_id) = current_user_id(&user) else {
        return Ok(HttpResponse::Unauthorized().finish());
    };
    let Some(u) = db.get_user(&user_id).await? else {
        return Ok(HttpResponse::Unauthorized().finish());
    };
    Ok(HttpResponse::Ok().json(json!({ "Account": PublicUser::from(u) })))
}

pub async fn get_liked(
    pagination: web::Query<PaginationOptions>,
    user: User,
    db: web::Data<MongoClient>,
) -> UserResponse {
    let Some(user_id) = current_user_id(&user) else {
        return Ok(HttpResponse::Unauthorized().finish());
    };
    let Some(u) = db.get_user(&user_id).await? else {
        return Ok(HttpResponse::Unauthorized().finish());
    };
    let mut musics = u.liked_musics().to_vec();
    musics.reverse();
    let res = db.get_musics(&pagination.trim_vec(&musics)).await?;
    Ok(HttpResponse::Ok().json(res))
}

pub async fn get_viewed(
    pagination: web::Query<PaginationOptions>,
    user: User,
    db: web::Data<MongoClient>,
) -> UserResponse {
    let Some(user_id) = current_user_id(&user) else {
        return Ok(HttpResponse::Unauthorized().finish());
    };
    let Some(u) = db.get_user(&user_id).await? else {
        return Ok(HttpResponse::Unauthorized().finish());
    };
    let mut musics = u.viewed_musics().to_vec();
    musics.reverse();
    let res = db.get_musics(&pagination.trim_vec(&musics)).await?;
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
pub async fn get_suggestions(
    user: User,
    settings: web::Query<SuggestionSettings>,
    db: web::Data<MongoClient>,
) -> UserResponse {
    let sugg = get_suggestions_for(
        db.get_ref(),
        user,
        settings.memory.min(300).max(10),
        settings.like_hist_ratio.min(1.0).max(0.0),
        settings.novelty.max(10.0).max(0.0),
        settings.limit.min(100).max(1),
    )
    .await;

    let res = db.get_musics(&sugg).await?;

    Ok(HttpResponse::Ok().json(res))
}

#[derive(serde::Deserialize)]
pub struct PreferedFormat {
    pub format: DeezerMusicFormats,
}

pub async fn set_prefered_format(
    user: User,
    format: web::Json<PreferedFormat>,
    db: web::Data<MongoClient>,
) -> actix_web::Result<HttpResponse> {
    db.set_prefered_format(&user, &format.format)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_prefered_format(
    user: User,
    db: web::Data<MongoClient>,
) -> actix_web::Result<HttpResponse> {
    let user_id = current_user_id(&user)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("unauthorized"))?;
    let user = db
        .get_user(&user_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("unauthorized"))?;
    Ok(HttpResponse::Ok().json(user.prefered_format().to_string()))
}
