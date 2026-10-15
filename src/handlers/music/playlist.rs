use actix::Addr;
use actix_web::{web, HttpResponse};
use bson::oid::ObjectId;
use serde::Deserialize;
use serde_json::json;

use crate::{
    actors::ArtistScraperActor,
    db::MongoClient,
    deezer::DeezerClient,
    models::{DeezerId, PopulatedPlaylist, User},
    search::MeilisearchClient,
};

use super::{index_search_musics_result, IndexType, MusicResponse};

fn parse_playlist_id(req: &web::Path<String>) -> Option<ObjectId> {
    ObjectId::parse_str(&**req).ok()
}

fn current_user_id(user: &User) -> Option<ObjectId> {
    user.id()
}

pub async fn get_playlist(
    req: web::Path<String>,
    user: User,
    db: web::Data<MongoClient>,
) -> MusicResponse {
    let Some(playlist_id) = parse_playlist_id(&req) else {
        return Ok(HttpResponse::BadRequest().finish());
    };
    let Some(request_user_id) = current_user_id(&user) else {
        return Ok(HttpResponse::Unauthorized().finish());
    };
    let Some(playlist) = db.get_playlist(&playlist_id).await? else {
        return Ok(HttpResponse::NotFound().finish());
    };
    if !playlist.is_authorized_read(&request_user_id) {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let musics = match playlist.musics.as_ref() {
        Some(musics) => db.get_musics(musics).await?,
        None => None,
    };
    let Some(user) = db.get_user_public(&playlist.creator()).await? else {
        return Ok(HttpResponse::NotFound().finish());
    };
    let mut playlist_pop = PopulatedPlaylist::from_playlist(playlist, user);
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
    db: web::Data<MongoClient>,
) -> MusicResponse {
    let Some(playlist_id) = parse_playlist_id(&req) else {
        return Ok(HttpResponse::BadRequest().finish());
    };
    let Some(request_user_id) = current_user_id(&user) else {
        return Ok(HttpResponse::Unauthorized().finish());
    };
    let Some(playlist) = db.get_playlist(&playlist_id).await? else {
        return Ok(HttpResponse::NotFound().finish());
    };
    if !playlist.is_authorized_write(&request_user_id) {
        return Ok(HttpResponse::Unauthorized().finish());
    }
    db.add_musics_playlist(playlist.id, &pl.musics).await?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn edit_music_playlist(
    user: User,
    pl: web::Json<AddRemoveMusicBody>,
    req: web::Path<String>,
    db: web::Data<MongoClient>,
) -> MusicResponse {
    let Some(playlist_id) = parse_playlist_id(&req) else {
        return Ok(HttpResponse::BadRequest().finish());
    };
    let Some(request_user_id) = current_user_id(&user) else {
        return Ok(HttpResponse::Unauthorized().finish());
    };
    let Some(playlist) = db.get_playlist(&playlist_id).await? else {
        return Ok(HttpResponse::NotFound().finish());
    };
    if !playlist.is_authorized_write(&request_user_id) {
        return Ok(HttpResponse::Unauthorized().finish());
    }
    db.edit_musics_playlist(playlist.id, &pl.musics).await?;
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
    db: web::Data<MongoClient>,
) -> MusicResponse {
    let Some(playlist_id) = parse_playlist_id(&req) else {
        return Ok(HttpResponse::BadRequest().finish());
    };
    let Some(request_user_id) = current_user_id(&user) else {
        return Ok(HttpResponse::Unauthorized().finish());
    };
    let Some(playlist) = db.get_playlist(&playlist_id).await? else {
        return Ok(HttpResponse::NotFound().finish());
    };
    if !playlist.is_authorized_write(&request_user_id) {
        return Ok(HttpResponse::Unauthorized().finish());
    }
    db.edit_playlist(
        playlist.id,
        pl.name.clone().unwrap_or(playlist.name().to_owned()),
        pl.is_public.unwrap_or(playlist.public()),
    )
    .await?;
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
    db: web::Data<MongoClient>,
) -> MusicResponse {
    let Some(playlist_id) = parse_playlist_id(&req) else {
        return Ok(HttpResponse::BadRequest().finish());
    };
    let Some(request_user_id) = current_user_id(&user) else {
        return Ok(HttpResponse::Unauthorized().finish());
    };
    let Some(playlist) = db.get_playlist(&playlist_id).await? else {
        return Ok(HttpResponse::NotFound().finish());
    };
    if !playlist.is_authorized_write(&request_user_id) {
        return Ok(HttpResponse::Unauthorized().finish());
    }
    db.remove_music_playlist(playlist.id, pl.at_index).await?;
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

pub async fn create_playlist(
    user: User,
    pl: web::Json<CreatePlaylistBody>,
    db: web::Data<MongoClient>,
) -> MusicResponse {
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
    db: web::Data<MongoClient>,
    dz: web::Data<DeezerClient>,
    search: web::Data<MeilisearchClient>,
    scraper: web::Data<Addr<ArtistScraperActor>>,
) -> MusicResponse {
    let music_dz_ids = dz.get_playlist_musics(&pl.deezer_id).await?;

    let musics: Vec<DeezerId> = match index_search_musics_result(
        db.get_ref(),
        search.get_ref(),
        &music_dz_ids,
        scraper.get_ref(),
        IndexType::None,
    )
    .await
    {
        Ok(musics) => musics.into_iter().map(|m| m.id).collect(),
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };

    let id = db
        .create_playlist(pl.name.clone(), &musics, pl.is_public, &user)
        .await?;
    Ok(HttpResponse::Ok().json(&json!({ "CreatedPlaylistId": id.to_hex() })))
}

pub async fn delete_playlist(
    req: web::Path<String>,
    user: User,
    db: web::Data<MongoClient>,
) -> MusicResponse {
    let Some(playlist_id) = parse_playlist_id(&req) else {
        return Ok(HttpResponse::BadRequest().finish());
    };
    let Some(request_user_id) = current_user_id(&user) else {
        return Ok(HttpResponse::Unauthorized().finish());
    };
    let Some(playlist) = db.get_playlist(&playlist_id).await? else {
        return Ok(HttpResponse::NotFound().finish());
    };
    if !playlist.is_authorized_write(&request_user_id) {
        return Ok(HttpResponse::Unauthorized().finish());
    }
    db.remove_playlist(&playlist.id).await?;
    Ok(HttpResponse::Ok().finish())
}
