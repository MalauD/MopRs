use crate::{
    db::{get_mongo, PaginationOptions},
    deezer::DeezerClient,
    models::{Album, Artist, Music},
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
            ),
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
