use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::json;

use crate::{db::get_mongo, models::{User, DeezerId}};

use super::UserResponse;

pub async fn get_current_playlist(user: User) -> UserResponse {
    let db = get_mongo(None).await;
    let u = db.get_user(&user.id().unwrap()).await.unwrap().unwrap();

    let res = db.get_musics(&u.current_playlist().to_vec()).await.unwrap();
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
) -> UserResponse {
    let db = get_mongo(None).await;

    let _ = db
        .set_current_playlist_musics(&user, &playlist.current_playlist)
        .await;
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
) -> UserResponse {
    let db = get_mongo(None).await;
    let _ = db
        .set_current_playlist_index(&user, &playlist.current_playlist_playing)
        .await;
    Ok(HttpResponse::Ok().finish())
}
