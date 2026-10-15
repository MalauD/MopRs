use actix_web::{web, HttpResponse};

use crate::{
    db::MongoClient,
    models::{DeezerId, User},
};

use super::MusicResponse;

pub async fn like_music(
    req: web::Path<DeezerId>,
    user: User,
    db: web::Data<MongoClient>,
) -> MusicResponse {
    let u = db.get_user(&user.id().unwrap()).await?.unwrap();
    let res = db.like_music(&u, &req).await?;
    db.modify_like_count(&req, if res { 1 } else { -1 }).await?;
    Ok(HttpResponse::Ok().finish())
}
