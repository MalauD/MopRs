use actix_web::{web, HttpResponse};
use bson::oid::ObjectId;
use serde_json::json;

use crate::{
    db::{MongoClient, PaginationOptions},
    models::{PopulatedPlaylist, User},
};

use super::UserResponse;

pub async fn get_user_playlists(
    req: web::Path<String>,
    user: User,
    pagination: web::Query<PaginationOptions>,
    db: web::Data<MongoClient>,
) -> UserResponse {
    let Ok(user_id) = ObjectId::parse_str(&*req) else {
        return Ok(HttpResponse::BadRequest().finish());
    };
    let user_id = &user_id;
    let playlists = db.get_user_playlists(user_id, &pagination).await?;

    let Some(creator) = db.get_user_public(user_id).await? else {
        return Ok(HttpResponse::NotFound().finish());
    };
    let Some(request_user_id) = user.id() else {
        return Ok(HttpResponse::Unauthorized().finish());
    };

    let mut pop_playlists: Vec<PopulatedPlaylist> = Vec::with_capacity(playlists.len());
    for playlist in playlists.iter().cloned() {
        if playlist.is_authorized_read(&request_user_id) {
            //Something else might be faster
            let musics = match playlist.musics.as_ref() {
                Some(musics) => db.get_musics(musics).await?,
                None => None,
            };
            let mut playlist_pop = PopulatedPlaylist::from_playlist(playlist, creator.clone());
            playlist_pop.musics = musics;
            pop_playlists.push(playlist_pop);
        }
    }

    Ok(HttpResponse::Ok().json(json!({
        "Playlists": pop_playlists,
        "Creator": creator
    })))
}
