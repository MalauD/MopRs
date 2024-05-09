use actix_web::{web, HttpResponse};
use bson::oid::ObjectId;
use serde_json::json;

use crate::{
    db::{get_mongo, PaginationOptions},
    models::{PopulatedPlaylist, User},
};

use super::UserResponse;

pub async fn get_user_playlists(
    req: web::Path<String>,
    user: User,
    pagination: web::Query<PaginationOptions>,
) -> UserResponse {
    let db = get_mongo(None).await;
    let user_id = &ObjectId::parse_str(&*req).unwrap();
    let playlists = db.get_user_playlists(user_id, &pagination).await.unwrap();

    let creator = db.get_user_public(user_id).await.unwrap().unwrap();

    let mut pop_playlists: Vec<PopulatedPlaylist> = Vec::with_capacity(playlists.len());
    for playlist in playlists.iter().cloned() {
        if playlist.is_authorized_read(&user.id().unwrap()) {
            //Something else might be faster
            let musics = db
                .get_musics(playlist.musics.as_ref().unwrap())
                .await
                .unwrap();
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
