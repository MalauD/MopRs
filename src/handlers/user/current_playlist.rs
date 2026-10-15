use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::json;

use crate::{
    db::MongoClient,
    models::{DeezerId, User},
};

use super::UserResponse;

pub async fn get_current_playlist(user: User, db: web::Data<MongoClient>) -> UserResponse {
    let Some(user_id) = user.id() else {
        return Ok(HttpResponse::Unauthorized().finish());
    };
    let Some(u) = db.get_user(&user_id).await? else {
        return Ok(HttpResponse::Unauthorized().finish());
    };

    let res = db.get_musics(u.current_playlist()).await?;
    Ok(HttpResponse::Ok()
        .json(json! ({"CurrentPlaylist": res, "CurrentPlaylistPlaying": u.current_playing()})))
}

#[derive(Deserialize)]
pub struct PlaylistMusics {
    #[serde(rename = "CurrentPlaylist")]
    pub current_playlist: Vec<DeezerId>,
}

pub async fn set_current_playlist_musics(
    user: User,
    playlist: web::Json<PlaylistMusics>,
    db: web::Data<MongoClient>,
) -> UserResponse {
    db.set_current_playlist_musics(&user, &playlist.current_playlist)
        .await?;
    Ok(HttpResponse::Ok().finish())
}

#[derive(Deserialize)]
pub struct CurrentPlaylistPlaying {
    #[serde(rename = "CurrentPlaylistPlaying")]
    pub current_playlist_playing: DeezerId,
}

pub async fn set_current_playlist_playing(
    user: User,
    playlist: web::Json<CurrentPlaylistPlaying>,
    db: web::Data<MongoClient>,
) -> UserResponse {
    db.set_current_playlist_index(&user, &playlist.current_playlist_playing)
        .await?;
    Ok(HttpResponse::Ok().finish())
}
