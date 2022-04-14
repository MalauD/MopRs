use std::path::PathBuf;

use crate::{
    app_settings::AppSettings,
    db::{get_mongo, PaginationOptions},
    deezer::{self, get_dz_client, refresh_dz_client, SearchMusicsResult},
    models::{
        Album, Artist, Chart, Music, PopulatedAlbum, PopulatedArtist, PopulatedPlaylist, User,
    },
    tools::MusicError,
};
use actix_files::NamedFile;
use actix_web::{web, HttpResponse};
use bson::oid::ObjectId;
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
    let db = get_mongo().await;
    let dz = get_dz_client(None).await.read().await;
    let res = dz.search_music(req.clone()).await.unwrap();
    index_search_musics_result(&res).await;
    //musics.group_by()
    let searched_musics = db.search_music(req.into_inner(), &pagination).await;
    Ok(HttpResponse::Ok().json(searched_musics.unwrap().unwrap()))
}

pub async fn search_album(
    req: web::Path<String>,
    pagination: web::Query<PaginationOptions>,
) -> MusicResponse {
    let db = get_mongo().await;
    let dz = get_dz_client(None).await.read().await;
    let res = dz.search_music(req.clone()).await.unwrap();
    index_search_musics_result(&res).await;
    //musics.group_by()
    let searched_albums = db.search_album(req.into_inner(), &pagination).await;
    Ok(HttpResponse::Ok().json(searched_albums.unwrap().unwrap()))
}

pub async fn search_artist(
    req: web::Path<String>,
    pagination: web::Query<PaginationOptions>,
) -> MusicResponse {
    let db = get_mongo().await;
    let dz = get_dz_client(None).await.read().await;
    let res = dz.search_music(req.clone()).await.unwrap();
    index_search_musics_result(&res).await;
    //musics.group_by()
    let searched_artists = db.search_artist(req.into_inner(), &pagination).await;
    Ok(HttpResponse::Ok().json(searched_artists.unwrap().unwrap()))
}

pub async fn search_playlist(
    req: web::Path<String>,
    user: User,
    pagination: web::Query<PaginationOptions>,
) -> MusicResponse {
    let db = get_mongo().await;

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
    let db = get_mongo().await;
    let dz = get_dz_client(None).await.read().await;

    let charts = db.get_chart_today().await.unwrap();
    let charts = match charts {
        Some(c) => c,
        None => {
            let chart = dz.get_most_popular().await.unwrap();
            index_search_musics_result(&SearchMusicsResult {
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
    let db = get_mongo().await;
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
    let db = get_mongo().await;
    let u = db.get_user(&user.id().unwrap()).await.unwrap().unwrap();
    let res = db.like_music(&u, &req).await.unwrap();
    db.modify_like_count(&req, if res { 1 } else { -1 })
        .await
        .unwrap();
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_artist(req: web::Path<i32>) -> MusicResponse {
    let db = get_mongo().await;
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
    let mut pop_artist = PopulatedArtist::from(compl_artist);
    pop_artist.albums = Some(albums_of_artist);
    Ok(HttpResponse::Ok().json(pop_artist))
}

pub async fn get_playlist(req: web::Path<String>, user: User) -> MusicResponse {
    let db = get_mongo().await;
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
    let db = get_mongo().await;
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
    let db = get_mongo().await;
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
    let db = get_mongo().await;
    let id = db
        .create_playlist(pl.name.clone(), &pl.musics, pl.is_public, &user)
        .await?;
    Ok(HttpResponse::Ok().json(&json!({ "CreatedPlaylistId": id.to_hex() })))
}

pub async fn delete_playlist(req: web::Path<String>, user: User) -> MusicResponse {
    let db = get_mongo().await;
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
    settings: web::Data<AppSettings>,
    user: User,
) -> actix_web::Result<NamedFile> {
    let dz = get_dz_client(None).await.read().await;
    let db = get_mongo().await;
    let path = format!("./Musics/{}.mp3", &req);
    let path: PathBuf = path.parse().unwrap();
    let f = NamedFile::open(path);
    let f = match f {
        Ok(file) => file,
        Err(_) => {
            let path_dir: PathBuf = settings.music_path().parse().unwrap();
            if let Ok(p) = dz.download_music(*req, &path_dir).await {
                NamedFile::open(p).unwrap()
            } else {
                refresh_dz_client().await;
                let dz = get_dz_client(None).await.read().await;
                NamedFile::open(dz.download_music(*req, &path_dir).await.unwrap()).unwrap()
            }
        }
    };

    db.add_to_history(&user, &req).await.unwrap();

    Ok(f.use_last_modified(true)
        .set_content_type("audio/mpeg".parse().unwrap()))
}

async fn index_search_musics_result(res: &deezer::SearchMusicsResult) {
    let db = get_mongo().await;
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
    let _ = db.bulk_insert_artists(artists).await;
    let lazy_update = async move {
        for val in musics.clone().iter() {
            let _ = db.append_to_album(&val.1.id, &val.0).await;
        }
        for val in albums.clone().iter() {
            let _ = db.append_to_artist(&val.1.id, &val.0).await;
        }
    };
    actix_rt::spawn(lazy_update);
}
