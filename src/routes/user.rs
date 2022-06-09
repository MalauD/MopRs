use crate::{
    db::{get_mongo, PaginationOptions},
    models::{PopulatedPlaylist, User, UserReq},
    tools::UserError,
};
use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};
use bson::oid::ObjectId;
use serde::Deserialize;
use serde_json::json;
use std::sync::RwLock;

type UserResponse = Result<HttpResponse, UserError>;

pub fn config_user(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/User")
            .route("/Login", web::post().to(login))
            .route("/Register", web::post().to(register))
            .route("/Logout", web::post().to(logout))
            .route("/Me", web::get().to(get_account))
            .route("/LikedMusics", web::get().to(get_liked))
            .route("/ViewedMusics", web::get().to(get_viewed))
            .route("/CurrentPlaylist", web::get().to(get_current_playlist))
            .route(
                "/CurrentPlaylist/Musics",
                web::post().to(set_current_playlist_musics),
            )
            .route(
                "/CurrentPlaylist/Playing",
                web::post().to(set_current_playlist_playing),
            )
            .route("/{id}/Playlists", web::get().to(get_user_playlists)),
    );
}

pub async fn login(id: Identity, user: web::Json<UserReq>) -> UserResponse {
    let db = get_mongo().await;
    if let Some(user_mod) = db.get_user_req(&user).await? {
        user_mod.login(&user)?;
        id.remember(user_mod.id().unwrap().to_string());
        Ok(HttpResponse::Ok().json(json!({"success": true})))
    } else {
        Ok(HttpResponse::Forbidden().finish())
    }
}

pub async fn register(id: Identity, user: web::Json<UserReq>) -> UserResponse {
    let db = get_mongo().await;
    let user_mod = User::new(&user.0);

    if db.has_user_by_name(&user_mod).await? {
        return Ok(HttpResponse::Ok().json(json!({"success": false})));
    }
    let user_saved = user_mod.clone();
    db.save_user(user_mod).await?;
    id.remember(user_saved.id().unwrap().to_string());
    Ok(HttpResponse::Ok().json(json!({"success": true})))
}

pub async fn logout(id: Identity) -> UserResponse {
    id.forget();
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_account(user: User) -> impl Responder {
    let db = get_mongo().await;
    let u = db.get_user(&user.id().unwrap()).await.unwrap().unwrap();
    web::Json(json!({ "Account": u }))
}

pub async fn get_liked(pagination: web::Query<PaginationOptions>, user: User) -> UserResponse {
    let db = get_mongo().await;
    let u = db.get_user(&user.id().unwrap()).await.unwrap().unwrap();
    let mut musics = u.liked_musics().to_vec();
    musics.reverse();
    let res = db.get_musics(&pagination.trim_vec(&musics)).await.unwrap();
    Ok(HttpResponse::Ok().json(res))
}

pub async fn get_viewed(pagination: web::Query<PaginationOptions>, user: User) -> UserResponse {
    let db = get_mongo().await;
    let u = db.get_user(&user.id().unwrap()).await.unwrap().unwrap();
    let mut musics = u.viewed_musics().to_vec();
    musics.reverse();
    let res = db.get_musics(&pagination.trim_vec(&musics)).await.unwrap();
    Ok(HttpResponse::Ok().json(res))
}

pub async fn get_current_playlist(user: User) -> UserResponse {
    let db = get_mongo().await;
    let u = db.get_user(&user.id().unwrap()).await.unwrap().unwrap();

    let res = db.get_musics(&u.current_playlist().to_vec()).await.unwrap();
    Ok(HttpResponse::Ok()
        .json(json! ({"CurrentPlaylist": res, "CurrentPlaylistPlaying": u.current_playing()})))
}

#[derive(Deserialize)]
pub struct PlaylistMusics {
    #[serde(rename = "CurrentPlaylist")]
    pub current_playlist: Vec<i32>,
}

pub async fn set_current_playlist_musics(
    user: User,
    playlist: web::Json<PlaylistMusics>,
) -> UserResponse {
    let db = get_mongo().await;

    let _ = db
        .set_current_playlist_musics(&user, &playlist.current_playlist)
        .await;
    Ok(HttpResponse::Ok().finish())
}

#[derive(Deserialize)]
pub struct CurrentPlaylistPlaying {
    #[serde(rename = "CurrentPlaylistPlaying")]
    pub current_playlist_playing: i32,
}

pub async fn set_current_playlist_playing(
    user: User,
    playlist: web::Json<CurrentPlaylistPlaying>,
) -> UserResponse {
    let db = get_mongo().await;
    let _ = db
        .set_current_playlist_index(&user, &playlist.current_playlist_playing)
        .await;
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_user_playlists(
    req: web::Path<String>,
    user: User,
    pagination: web::Query<PaginationOptions>,
) -> UserResponse {
    let db = get_mongo().await;
    let user_id = &ObjectId::parse_str(&*req).unwrap();
    let playlists = db.get_user_playlists(user_id, &pagination).await.unwrap();

    let creator = db.get_user(user_id).await.unwrap().unwrap();

    let mut pop_playlists: Vec<PopulatedPlaylist> = Vec::with_capacity(playlists.len());
    for playlist in playlists.iter().cloned() {
        if playlist.is_authorized_read(&user.id().unwrap()) {
            //Something else might be faster
            let musics = db
                .get_musics(&playlist.musics.as_ref().unwrap())
                .await
                .unwrap();
            let mut playlist_pop = PopulatedPlaylist::from_playlist(playlist, creator.clone());
            playlist_pop.musics = musics;
            pop_playlists.push(playlist_pop);
        }
    }

    Ok(HttpResponse::Ok().json(pop_playlists))
}
