use std::str::FromStr;

use crate::{
    db::{get_mongo, PaginationOptions},
    deezer::{self, get_dz_client, refresh_dz_client, SearchMusicsResult},
    models::{
        Album, Artist, Chart, Music, PopulatedAlbum, PopulatedArtist, PopulatedPlaylist, User,
    },
    s3::get_s3,
    tools::MusicError,
};
use actix_web::{http::header::Range, web, HttpRequest, HttpResponse};
use bson::oid::ObjectId;
use chrono::{Duration, Utc};
use itertools::Itertools;
use serde::Deserialize;
use serde_json::json;

type MusicResponse = Result<HttpResponse, MusicError>;

pub fn config_music(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/Music")
            .route(
                "/Search/Music/Name/{search_req}",
                web::get().to(search_music),
            )
            .route(
                "/Search/Album/Name/{search_req}",
                web::get().to(search_album),
            )
            .route(
                "/Search/Artist/Name/{search_req}",
                web::get().to(search_artist),
            )
            .route(
                "/Search/Playlist/Name/{search_req}",
                web::get().to(search_playlist),
            )
            .route("/Trending/Musics", web::get().to(trending_musics))
            .route("/Album/id/{id}", web::get().to(get_album))
            .route("/Artist/id/{id}", web::get().to(get_artist))
            .route("/Playlist/Create", web::post().to(create_playlist))
            .route("/Playlist/id/{id}", web::get().to(get_playlist))
            .route("/Playlist/id/{id}", web::delete().to(delete_playlist))
            .route("/Playlist/id/{id}/Add", web::post().to(add_music_playlist))
            .route(
                "/Playlist/id/{id}/Remove",
                web::delete().to(remove_music_playlist),
            )
            .route("/cdn/{id}", web::get().to(get_music))
            .route("/Like/Music/{id}", web::get().to(like_music)),
    );
}

pub async fn search_music(
    req: web::Path<String>,
    pagination: web::Query<PaginationOptions>,
) -> MusicResponse {
    let db = get_mongo(None).await;
    let dz = get_dz_client(None).await.read().await;
    let res = dz.search_music(req.clone()).await.unwrap();
    let _ = index_search_musics_result(&res).await;
    //musics.group_by()
    let searched_musics = db.search_music(req.into_inner(), &pagination).await;
    Ok(HttpResponse::Ok().json(searched_musics.unwrap().unwrap()))
}

pub async fn search_album(
    req: web::Path<String>,
    pagination: web::Query<PaginationOptions>,
) -> MusicResponse {
    let db = get_mongo(None).await;
    let dz = get_dz_client(None).await.read().await;
    let res = dz.search_music(req.clone()).await.unwrap();
    let _ = index_search_musics_result(&res).await;
    //musics.group_by()
    let searched_albums = db.search_album(req.into_inner(), &pagination).await;
    Ok(HttpResponse::Ok().json(searched_albums.unwrap().unwrap()))
}

pub async fn search_artist(
    req: web::Path<String>,
    pagination: web::Query<PaginationOptions>,
) -> MusicResponse {
    let db = get_mongo(None).await;
    let dz = get_dz_client(None).await.read().await;
    let res = dz.search_music(req.clone()).await.unwrap();
    let _ = index_search_musics_result(&res).await;
    //musics.group_by()
    let searched_artists = db.search_artist(req.into_inner(), &pagination).await;
    Ok(HttpResponse::Ok().json(searched_artists.unwrap().unwrap()))
}

pub async fn search_playlist(
    req: web::Path<String>,
    user: User,
    pagination: web::Query<PaginationOptions>,
) -> MusicResponse {
    let db = get_mongo(None).await;

    let playlists = db
        .search_playlist(req.into_inner(), &pagination)
        .await
        .unwrap()
        .unwrap();

    let mut pop_playlists: Vec<PopulatedPlaylist> = Vec::with_capacity(playlists.len());
    for playlist in playlists.iter().cloned() {
        if playlist.is_authorized_read(&user.clone().id().unwrap()) {
            //Something else might be faster
            let musics = db
                .get_musics(&playlist.musics.as_ref().unwrap())
                .await
                .unwrap();
            //TODO add correct user..
            let mut playlist_pop = PopulatedPlaylist::from_playlist(playlist, user.clone());
            playlist_pop.musics = musics;
            pop_playlists.push(playlist_pop);
        }
    }

    Ok(HttpResponse::Ok().json(pop_playlists))
}

pub async fn trending_musics(pagination: web::Query<PaginationOptions>) -> MusicResponse {
    let db = get_mongo(None).await;
    let dz = get_dz_client(None).await.read().await;

    let charts = db.get_chart_today().await.unwrap();
    let charts = match charts {
        Some(c) => c,
        None => {
            let chart = dz.get_most_popular().await.unwrap();
            let _ = index_search_musics_result(&SearchMusicsResult {
                data: chart.clone().tracks.data,
            })
            .await;
            let ch = Chart::from(chart);
            let _ = db.insert_chart(&ch).await.unwrap();
            ch
        }
    };
    let vec: Vec<i32> = pagination.trim_vec(&charts.musics);
    let musics = db.get_musics(&vec).await.unwrap();

    Ok(HttpResponse::Ok().json(musics))
}

pub async fn get_album(req: web::Path<i32>) -> MusicResponse {
    let db = get_mongo(None).await;
    let dz = get_dz_client(None).await.read().await;
    let res = dz.get_album_musics(req.clone()).await.unwrap();
    let album = db.get_album(&req).await.unwrap().unwrap();
    let musics: Vec<Music> = res
        .data
        .clone()
        .into_iter()
        .map(|x| Music {
            image_url: Some(album.cover.clone()),
            ..Music::from(x)
        })
        .unique_by(|x| x.id)
        .collect_vec();
    let music_ids = musics.clone().into_iter().map(|x| x.id).collect_vec();
    let _ = db.bulk_insert_musics(musics).await;
    let _ = db.set_album_musics(music_ids, &req).await;
    //musics.group_by()
    let compl_album = db.get_album(&req).await.unwrap().unwrap();
    let musics_of_album = db
        .get_musics(&compl_album.musics.as_ref().unwrap())
        .await
        .unwrap()
        .unwrap();
    let mut pop_album = PopulatedAlbum::from(compl_album);
    pop_album.musics = Some(musics_of_album);
    Ok(HttpResponse::Ok().json(pop_album))
}

pub async fn like_music(req: web::Path<i32>, user: User) -> MusicResponse {
    let db = get_mongo(None).await;
    let u = db.get_user(&user.id().unwrap()).await.unwrap().unwrap();
    let res = db.like_music(&u, &req).await.unwrap();
    db.modify_like_count(&req, if res { 1 } else { -1 })
        .await
        .unwrap();
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_artist(req: web::Path<i32>) -> MusicResponse {
    let db = get_mongo(None).await;
    let dz = get_dz_client(None).await.read().await;
    let res = dz.get_artist_albums(&req).await.unwrap();
    let albums: Vec<Album> = res
        .data
        .clone()
        .into_iter()
        .map(|x| Album::from(x))
        .unique_by(|x| x.id)
        .collect_vec();
    let albums_id = albums.clone().into_iter().map(|x| x.id).collect_vec();
    let _ = db.bulk_insert_albums(albums).await;
    let _ = db.append_multiple_to_an_artist(albums_id, &req).await;
    //musics.group_by()
    let compl_artist = db.get_artist(&req).await.unwrap().unwrap();
    let albums_of_artist = db
        .get_albums(&compl_artist.albums.as_ref().unwrap())
        .await
        .unwrap()
        .unwrap();
    let mut pop_artist = PopulatedArtist::from(compl_artist.clone());
    pop_artist.albums = Some(albums_of_artist);

    if Utc::now() - compl_artist.last_update > Duration::hours(1)
        || compl_artist.top_tracks.is_none()
        || compl_artist.related_artists.is_none()
    {
        let related = dz.get_related_artists(&req).await.unwrap();
        let top_tracks = dz.get_artist_top_tracks(&req).await.unwrap();

        let rel_artists: Vec<Artist> = related
            .data
            .clone()
            .into_iter()
            .map(|x| Artist::from(x))
            .unique_by(|x| x.id)
            .collect_vec();
        let _ = db.bulk_insert_artists(&rel_artists).await;
        let _ = db
            .set_related_artists(
                &req,
                rel_artists.clone().into_iter().map(|x| x.id).collect_vec(),
            )
            .await;

        let tracks = index_artist_top_tracks(&top_tracks, &req).await.unwrap();
        let _ = db
            .set_top_tracks(
                &req,
                &tracks.clone().into_iter().map(|x| x.id).collect_vec(),
            )
            .await
            .unwrap();
    };
    let top_tracks = db
        .get_musics(&compl_artist.top_tracks.unwrap())
        .await
        .unwrap()
        .unwrap();
    let related = db
        .get_artists(&compl_artist.related_artists.unwrap())
        .await
        .unwrap()
        .unwrap();

    pop_artist.related_artists = Some(related);
    pop_artist.top_tracks = Some(top_tracks);

    Ok(HttpResponse::Ok().json(pop_artist))
}

pub async fn get_playlist(req: web::Path<String>, user: User) -> MusicResponse {
    let db = get_mongo(None).await;
    let playlist = db
        .get_playlist(&ObjectId::parse_str(&*req).unwrap())
        .await
        .unwrap();
    if playlist.is_none() {
        return Ok(HttpResponse::NotFound().finish());
    }
    let playlist = playlist.unwrap();
    if !playlist.is_authorized_read(&user.id().unwrap()) {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let musics = db
        .get_musics(&playlist.musics.as_ref().unwrap())
        .await
        .unwrap();
    let user = db.get_user(&playlist.creator()).await.unwrap();
    let mut playlist_pop = PopulatedPlaylist::from_playlist(playlist, user.unwrap());
    playlist_pop.musics = musics;
    Ok(HttpResponse::Ok().json(playlist_pop))
}
#[derive(Deserialize)]
pub struct AddRemoveMusicBody {
    #[serde(rename = "MusicsId")]
    pub musics: Vec<i32>,
}

pub async fn add_music_playlist(
    user: User,
    pl: web::Json<AddRemoveMusicBody>,
    req: web::Path<String>,
) -> MusicResponse {
    let db = get_mongo(None).await;
    let playlist = db
        .get_playlist(&ObjectId::parse_str(&*req).unwrap())
        .await
        .unwrap();
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

pub async fn remove_music_playlist(
    user: User,
    pl: web::Json<AddRemoveMusicBody>,
    req: web::Path<String>,
) -> MusicResponse {
    let db = get_mongo(None).await;
    let playlist = db
        .get_playlist(&ObjectId::parse_str(&*req).unwrap())
        .await
        .unwrap();
    if playlist.is_none() {
        return Ok(HttpResponse::NotFound().finish());
    }
    let playlist = playlist.unwrap();
    if !playlist.is_authorized_write(&user.id().unwrap()) {
        return Ok(HttpResponse::Unauthorized().finish());
    }
    let _ = db.remove_musics_playlist(playlist.id, &pl.musics).await;
    Ok(HttpResponse::Ok().finish())
}

#[derive(Deserialize)]
pub struct CreatePlaylistBody {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "MusicsId")]
    pub musics: Vec<i32>,
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

pub async fn delete_playlist(req: web::Path<String>, user: User) -> MusicResponse {
    let db = get_mongo(None).await;
    let playlist = db
        .get_playlist(&ObjectId::parse_str(&*req).unwrap())
        .await
        .unwrap();
    if playlist.is_none() {
        return Ok(HttpResponse::NotFound().finish());
    }
    let playlist = playlist.unwrap();
    if !playlist.is_authorized_write(&user.id().unwrap()) {
        return Ok(HttpResponse::Unauthorized().finish());
    }
    let _ = db.remove_playlist(&playlist.id).await.unwrap();
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_music(
    req: web::Path<i32>,
    user: User,
    httpreq: HttpRequest,
) -> actix_web::Result<HttpResponse> {
    let db = get_mongo(None).await;
    let bucket = get_s3(None).await.get_bucket();
    let dz = get_dz_client(None).await.read().await;
    let id = req.into_inner();

    let res = bucket.get_object(format!("/{}", id)).await;
    let t = if res.is_err() {
        if let Ok(track) = dz.download_music(id).await {
            track
        } else {
            drop(dz);
            refresh_dz_client().await;
            let dz = get_dz_client(None).await.read().await;
            dz.download_music(id).await.unwrap()
        }
    } else {
        let res = res.unwrap();
        db.add_to_history(&user, &id).await.unwrap();
        res.bytes().to_vec()
    };

    if let Some(r) = httpreq.headers().get("range") {
        let range = Range::from_str(r.to_str().unwrap()).unwrap();
        if let Range::Bytes(ranges) = range {
            let range = ranges
                .first()
                .unwrap()
                .to_satisfiable_range(t.len() as u64)
                .unwrap();
            Ok(HttpResponse::PartialContent()
                .append_header((
                    "Content-Range",
                    format!("bytes {}-{}/{}", range.0, range.1, t.len()),
                ))
                .append_header(("Accept-Ranges", "bytes"))
                .append_header(("Content-Type", "audio/mpeg"))
                .append_header(("Content-Length", t.len().to_string()))
                .body(t))
        } else {
            Ok(HttpResponse::Ok()
                .append_header(("Content-Type", "audio/mpeg"))
                .body(t))
        }
    } else {
        Ok(HttpResponse::Ok()
            .append_header(("Content-Type", "audio/mpeg"))
            .body(t))
    }
}

async fn index_search_musics_result(
    res: &deezer::SearchMusicsResult,
) -> Result<Vec<Music>, String> {
    let db = get_mongo(None).await;
    let artists: Vec<Artist> = res
        .data
        .clone()
        .into_iter()
        .map(|x| Artist::from(x))
        .unique_by(|x| x.id)
        .collect_vec();
    let albums: Vec<(i32, Album)> = res
        .data
        .clone()
        .into_iter()
        .map(|x| (x.artist.id, Album::from(x)))
        .unique_by(|x| x.1.id)
        .collect_vec();
    let musics: Vec<(i32, Music)> = res
        .data
        .clone()
        .into_iter()
        .map(|x| (x.album.id, Music::from(x)))
        .collect();
    let _ = db
        .bulk_insert_musics(musics.clone().into_iter().map(|x| x.1).collect())
        .await;
    let _ = db
        .bulk_insert_albums(albums.clone().into_iter().map(|x| x.1).collect())
        .await;
    let _ = db.bulk_insert_artists(&artists).await;
    let musics_s = musics.clone();
    let lazy_update = async move {
        for val in musics_s.clone().iter() {
            let _ = db.append_to_album(&val.1.id, &val.0).await;
        }
        for val in albums.clone().iter() {
            let _ = db.append_to_artist(&val.1.id, &val.0).await;
        }
    };
    actix_rt::spawn(lazy_update);
    Ok(musics.into_iter().map(|x| x.1).collect())
}

async fn index_artist_top_tracks(
    res: &deezer::ArtistTopTracksResult,
    artist_id: &i32,
) -> Result<Vec<Music>, String> {
    let db = get_mongo(None).await;
    let albums: Vec<(i32, Album)> = res
        .data
        .clone()
        .into_iter()
        .map(|x| (*artist_id, Album::from(x)))
        .unique_by(|x| x.1.id)
        .collect_vec();
    let musics: Vec<(i32, Music)> = res
        .data
        .clone()
        .into_iter()
        .map(|x| (x.album.id, Music::from(x)))
        .collect();
    let _ = db
        .bulk_insert_musics(musics.clone().into_iter().map(|x| x.1).collect())
        .await;
    let _ = db
        .bulk_insert_albums(albums.clone().into_iter().map(|x| x.1).collect())
        .await;
    let musics_s = musics.clone();
    let lazy_update = async move {
        for val in musics_s.clone().iter() {
            let _ = db.append_to_album(&val.1.id, &val.0).await;
        }
        for val in albums.clone().iter() {
            let _ = db.append_to_artist(&val.1.id, &val.0).await;
        }
    };
    actix_rt::spawn(lazy_update);
    Ok(musics.into_iter().map(|x| x.1).collect())
}
