use actix::Addr;
use actix_web::{web, HttpResponse};
use bson::oid::ObjectId;
use serde::Deserialize;
use serde_json::json;

use crate::{
    actors::ArtistScraperActor,
    db::get_mongo,
    deezer::get_dz_client,
    models::{DeezerId, PopulatedPlaylist, User},
};

use super::{index_search_musics_result, IndexType, MusicResponse};

pub async fn get_playlist(req: web::Path<String>, user: User) -> MusicResponse {
    let db = get_mongo(None).await;
    let playlist = db
        .get_playlist(&ObjectId::parse_str(&*req).unwrap())
        .await?;
    if playlist.is_none() {
        return Ok(HttpResponse::NotFound().finish());
    }
    let playlist = playlist.unwrap();
    if !playlist.is_authorized_read(&user.id().unwrap()) {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let musics = db.get_musics(playlist.musics.as_ref().unwrap()).await?;
    let user = db.get_user(&playlist.creator()).await.unwrap();
    let mut playlist_pop = PopulatedPlaylist::from_playlist(playlist, user.unwrap());
    playlist_pop.musics = musics;
    Ok(HttpResponse::Ok().json(playlist_pop))
}
#[derive(Deserialize)]
pub struct AddRemoveMusicBody {
    #[serde(rename = "MusicsId")]
    pub musics: Vec<DeezerId>,
}

pub async fn add_music_playlist(
    user: User,
    pl: web::Json<AddRemoveMusicBody>,
    req: web::Path<String>,
) -> MusicResponse {
    let db = get_mongo(None).await;
    let playlist = db
        .get_playlist(&ObjectId::parse_str(&*req).unwrap())
        .await?;
    if playlist.is_none() {
        return Ok(HttpResponse::NotFound().finish());
    }
    let playlist = playlist.unwrap();
    if !playlist.is_authorized_write(&user.id().unwrap()) {
        return Ok(HttpResponse::Unauthorized().finish());
    }
    let _ = db.add_musics_playlist(playlist.id, &pl.musics).await;
    Ok(HttpResponse::Ok().finish())
}

pub async fn edit_music_playlist(
    user: User,
    pl: web::Json<AddRemoveMusicBody>,
    req: web::Path<String>,
) -> MusicResponse {
    let db = get_mongo(None).await;
    let playlist = db
        .get_playlist(&ObjectId::parse_str(&*req).unwrap())
        .await?;
    if playlist.is_none() {
        return Ok(HttpResponse::NotFound().finish());
    }
    let playlist = playlist.unwrap();
    if !playlist.is_authorized_write(&user.id().unwrap()) {
        return Ok(HttpResponse::Unauthorized().finish());
    }
    let _ = db.edit_musics_playlist(playlist.id, &pl.musics).await;
    Ok(HttpResponse::Ok().finish())
}

#[derive(Deserialize)]
pub struct EditPlaylistBody {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "IsPublic")]
    pub is_public: Option<bool>,
}

pub async fn edit_playlist(
    user: User,
    req: web::Path<String>,
    pl: web::Json<EditPlaylistBody>,
) -> MusicResponse {
    let db = get_mongo(None).await;
    let playlist = db
        .get_playlist(&ObjectId::parse_str(&*req).unwrap())
        .await?;
    if playlist.is_none() {
        return Ok(HttpResponse::NotFound().finish());
    }
    let playlist = playlist.unwrap();
    if !playlist.is_authorized_write(&user.id().unwrap()) {
        return Ok(HttpResponse::Unauthorized().finish());
    }
    let _ = db
        .edit_playlist(
            playlist.id,
            pl.name.clone().unwrap_or(playlist.name().to_owned()),
            pl.is_public.unwrap_or(playlist.public()),
        )
        .await;
    Ok(HttpResponse::Ok().finish())
}

#[derive(Deserialize)]
pub struct RemovePlaylistBody {
    #[serde(rename = "AtIndex")]
    pub at_index: usize,
}

pub async fn remove_music_playlist(
    user: User,
    pl: web::Json<RemovePlaylistBody>,
    req: web::Path<String>,
) -> MusicResponse {
    let db = get_mongo(None).await;
    let playlist = db
        .get_playlist(&ObjectId::parse_str(&*req).unwrap())
        .await?;
    if playlist.is_none() {
        return Ok(HttpResponse::NotFound().finish());
    }
    let playlist = playlist.unwrap();
    if !playlist.is_authorized_write(&user.id().unwrap()) {
        return Ok(HttpResponse::Unauthorized().finish());
    }
    let _ = db.remove_music_playlist(playlist.id, pl.at_index).await;
    Ok(HttpResponse::Ok().finish())
}

#[derive(Deserialize)]
pub struct CreatePlaylistBody {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "MusicsId")]
    pub musics: Vec<DeezerId>,
    #[serde(rename = "IsPublic")]
    pub is_public: bool,
}

pub async fn create_playlist(user: User, pl: web::Json<CreatePlaylistBody>) -> MusicResponse {
    let db = get_mongo(None).await;
    let id = db
        .create_playlist(pl.name.clone(), &pl.musics, pl.is_public, &user)
        .await?;
    Ok(HttpResponse::Ok().json(&json!({ "CreatedPlaylistId": id.to_hex() })))
}

#[derive(Deserialize)]
pub struct CreatePlaylistDeezerBody {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "DeezerId")]
    pub deezer_id: DeezerId,
    #[serde(rename = "IsPublic")]
    pub is_public: bool,
}

pub async fn create_playlist_deezer(
    user: User,
    pl: web::Json<CreatePlaylistDeezerBody>,
    scraper: web::Data<Addr<ArtistScraperActor>>,
) -> MusicResponse {
    let db = get_mongo(None).await;
    let dz = get_dz_client();

    let music_dz_ids = dz.get_playlist_musics(&pl.deezer_id).await?;

    let musics: Vec<DeezerId> =
        index_search_musics_result(&music_dz_ids, scraper.get_ref(), IndexType::None)
            .await
            .unwrap()
            .into_iter()
            .map(|m| m.id)
            .collect();

    let id = db
        .create_playlist(pl.name.clone(), &musics, pl.is_public, &user)
        .await?;
    Ok(HttpResponse::Ok().json(&json!({ "CreatedPlaylistId": id.to_hex() })))
}

pub async fn delete_playlist(req: web::Path<String>, user: User) -> MusicResponse {
    let db = get_mongo(None).await;
    let playlist = db
        .get_playlist(&ObjectId::parse_str(&*req).unwrap())
        .await?;
    if playlist.is_none() {
        return Ok(HttpResponse::NotFound().finish());
    }
    let playlist = playlist.unwrap();
    if !playlist.is_authorized_write(&user.id().unwrap()) {
        return Ok(HttpResponse::Unauthorized().finish());
    }
    db.remove_playlist(&playlist.id).await?;
    Ok(HttpResponse::Ok().finish())
}
