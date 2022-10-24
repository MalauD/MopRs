use actix_identity::Identity;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use serde_json::json;

use crate::{
    db::get_mongo,
    models::{User, UserReq},
};

use super::UserResponse;

pub async fn login(request: HttpRequest, user: web::Json<UserReq>) -> UserResponse {
    let db = get_mongo(None).await;
    if let Some(user_mod) = db.get_user_req(&user).await? {
        user_mod.login(&user)?;
        let _ = Identity::login(&request.extensions(), user_mod.id().unwrap().to_string());
        Ok(HttpResponse::Ok().json(json!({"success": true})))
    } else {
        Ok(HttpResponse::Forbidden().finish())
    }
}

pub async fn register(request: HttpRequest, user: web::Json<UserReq>) -> UserResponse {
    let db = get_mongo(None).await;
    let user_mod = User::new(&user.0);

    if db.has_user_by_name(&user_mod).await? {
        return Ok(HttpResponse::Ok().json(json!({"success": false})));
    }
    let uid = db.save_user(user_mod).await?;
    let _ = Identity::login(&request.extensions(), uid.to_string());
    Ok(HttpResponse::Ok().json(json!({"success": true})))
}

pub async fn logout(id: Identity) -> UserResponse {
    Identity::logout(id);
    Ok(HttpResponse::Ok().finish())
}
