use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::{
    db::{get_mongo, PaginationOptions},
    models::{PublicUser, User},
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
