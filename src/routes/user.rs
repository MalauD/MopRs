use crate::{
    db::get_mongo,
    models::{Sessions, User, UserReq},
    tools::UserError,
};
use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use std::sync::RwLock;

type UserResponse = Result<HttpResponse, UserError>;

pub fn config_user(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/User")
            .route("/Login", web::post().to(login))
            .route("/Register", web::post().to(register))
            .route("/Logout", web::post().to(logout))
            .route("/Me", web::get().to(get_account)),
    );
}

pub async fn login(
    id: Identity,
    user: web::Json<UserReq>,
    sessions: web::Data<RwLock<Sessions>>,
) -> UserResponse {
    let db = get_mongo().await;
    if let Some(user_mod) = db.get_user(&user).await? {
        user_mod.login(&user)?;
        id.remember(user_mod.get_username());
        sessions
            .write()
            .unwrap()
            .map
            .insert(user_mod.get_username(), user_mod);
        Ok(HttpResponse::Ok().json(json!({"success": true})))
    } else {
        Ok(HttpResponse::Forbidden().finish())
    }
}

pub async fn register(
    id: Identity,
    user: web::Json<UserReq>,
    sessions: web::Data<RwLock<Sessions>>,
) -> UserResponse {
    let db = get_mongo().await;
    let user_mod = User::new(&user.0);

    if db.has_user_by_name(&user_mod).await? {
        return Ok(HttpResponse::Ok().json(json!({"success": false})));
    }
    let user_saved = user_mod.clone();
    db.save_user(user_mod).await?;
    id.remember(user.get_username());
    sessions
        .write()
        .unwrap()
        .map
        .insert(user.get_username(), user_saved.clone());
    Ok(HttpResponse::Ok().json(json!({"success": true})))
}

pub async fn logout(id: Identity) -> UserResponse {
    id.forget();
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_account(user: User) -> impl Responder {
    web::Json(json!({ "Account": user }))
}
