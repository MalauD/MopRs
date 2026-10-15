use actix_identity::Identity;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use serde_json::json;

use crate::{
    db::MongoClient,
    models::{User, UserReq},
    tools::UserError,
};

use super::UserResponse;

pub async fn login(
    request: HttpRequest,
    user: web::Json<UserReq>,
    db: web::Data<MongoClient>,
) -> UserResponse {
    if let Some(user_mod) = db.get_user_req(&user).await? {
        user_mod.login(&user)?;
        let user_id = user_mod.id().ok_or(UserError::AuthenticationError)?;
        Identity::login(&request.extensions(), user_id.to_string())
            .map_err(|_| UserError::AuthenticationError)?;

        Ok(HttpResponse::Ok().json(json!({"success": true})))
    } else {
        Ok(HttpResponse::Forbidden().finish())
    }
}

pub async fn register(
    request: HttpRequest,
    user: web::Json<UserReq>,
    db: web::Data<MongoClient>,
) -> UserResponse {
    let user_mod = User::new(&user.0);

    if db.has_user_by_name(&user_mod).await? {
        return Ok(HttpResponse::Ok().json(json!({"success": false})));
    }
    let uid = db.save_user(user_mod).await?;
    Identity::login(&request.extensions(), uid.to_string())
        .map_err(|_| UserError::AuthenticationError)?;
    Ok(HttpResponse::Ok().json(json!({"success": true})))
}

pub async fn logout(id: Identity) -> UserResponse {
    Identity::logout(id);
    Ok(HttpResponse::Ok().finish())
}
