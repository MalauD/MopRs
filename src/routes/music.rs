use crate::{
    db::{get_mongo, PaginationOptions},
    deezer::DeezerClient,
    models::{Album, Artist, Music, PopulatedAlbum},
    tools::MusicError,
};
use actix_web::{web, HttpResponse};
use itertools::Itertools;

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
            .route("/Album/id/{id}", web::get().to(get_album)),
    );
}

pub async fn search_music(
    req: web::Path<String>,
    deezer_api: web::Data<DeezerClient>,
    pagination: web::Query<PaginationOptions>,
) -> MusicResponse {
    let db = get_mongo().await;
    let res = deezer_api.search_music(req.clone()).await.unwrap();
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
            let _ = db.append_to_album(val.1.id, val.0).await;
        }
        for val in albums.clone().iter() {
            let _ = db.append_to_artist(val.1.id, val.0).await;
        }
    };

    actix_rt::spawn(lazy_update);
    //musics.group_by()
    let searched_musics = db.search_music(req.into_inner(), &pagination).await;
    Ok(HttpResponse::Ok().json(searched_musics.unwrap().unwrap()))
}

pub async fn search_album(
    req: web::Path<String>,
    deezer_api: web::Data<DeezerClient>,
    pagination: web::Query<PaginationOptions>,
) -> MusicResponse {
    let db = get_mongo().await;
    let res = deezer_api.search_music(req.clone()).await.unwrap();
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

    let _ = db
        .bulk_insert_albums(albums.clone().into_iter().map(|x| x.1).collect())
        .await;
    let _ = db.bulk_insert_artists(artists).await;

    let lazy_update = async move {
        for val in albums.clone().iter() {
            let _ = db.append_to_artist(val.1.id, val.0).await;
        }
    };

    actix_rt::spawn(lazy_update);
    //musics.group_by()
    let searched_albums = db.search_album(req.into_inner(), &pagination).await;
    Ok(HttpResponse::Ok().json(searched_albums.unwrap().unwrap()))
}

pub async fn search_artist(
    req: web::Path<String>,
    deezer_api: web::Data<DeezerClient>,
    pagination: web::Query<PaginationOptions>,
) -> MusicResponse {
    let db = get_mongo().await;
    let res = deezer_api.search_music(req.clone()).await.unwrap();
    let artists: Vec<Artist> = res
        .data
        .clone()
        .into_iter()
        .map(|x| Artist::from(x))
        .unique_by(|x| x.id)
        .collect_vec();

    let _ = db.bulk_insert_artists(artists).await;

    //musics.group_by()
    let searched_artists = db.search_artist(req.into_inner(), &pagination).await;
    Ok(HttpResponse::Ok().json(searched_artists.unwrap().unwrap()))
}

pub async fn get_album(req: web::Path<i32>, deezer_api: web::Data<DeezerClient>) -> MusicResponse {
    let db = get_mongo().await;
    let res = deezer_api.get_album_musics(req.clone()).await.unwrap();
    let album = db.get_album(req.clone()).await.unwrap().unwrap();
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
    let _ = db.append_multiple_to_an_album(music_ids, req.clone()).await;
    //musics.group_by()
    let compl_album = db.get_album(req.clone()).await.unwrap().unwrap();
    let musics_of_album = db
        .get_musics(&compl_album.musics.as_ref().unwrap())
        .await
        .unwrap()
        .unwrap();
    let mut pop_album = PopulatedAlbum::from(compl_album);
    pop_album.musics = Some(musics_of_album);
    Ok(HttpResponse::Ok().json(pop_album))
}
