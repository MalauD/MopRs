use crate::{
    actors::ArtistScraperActor,
    db::{get_mongo, PaginationOptions},
    deezer::get_dz_client,
    models::{PopulatedPlaylist, User},
    search::{self, get_meilisearch},
};
use actix::Addr;
use actix_web::{web, HttpResponse};
use serde::Deserialize;

use super::{index_search_musics_result, IndexType, MusicResponse};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchOption {
    pub no_index: Option<bool>,
}

pub async fn search_music(
    req: web::Path<String>,
    pagination: web::Query<PaginationOptions>,
    search_opt: web::Query<SearchOption>,
    scraper: web::Data<Addr<ArtistScraperActor>>,
) -> MusicResponse {
    let db = get_mongo(None).await;
    let search = get_meilisearch(None).await;
    let dz = get_dz_client(None).await.read().await;

    if let Some(true) = search_opt.no_index {
        return Ok(HttpResponse::Ok().json(
            search
                .search_musics(req.into_inner(), pagination.into_inner())
                .await?,
        ));
    }

    if pagination.get_page() == 0 {
        let res = dz.search_music(req.clone()).await?;
        let _ = index_search_musics_result(&res, scraper.get_ref(), IndexType::Music).await;
        //musics.group_by()
    }
    let search_res = search
        .search_musics(req.into_inner(), pagination.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(search_res))
}

pub async fn search_album(
    req: web::Path<String>,
    pagination: web::Query<PaginationOptions>,
) -> MusicResponse {
    let db = get_mongo(None).await;
    let search = get_meilisearch(None).await;
    let _dz = get_dz_client(None).await.read().await;
    /*
    if pagination.get_page() == 0 {}
        let res = dz.search_music(req.clone()).await?;
        let _ = index_search_musics_result(&res, scraper.get_ref(), IndexType::Album).await;
    }
    //musics.group_by() */
    let search_res = search
        .search_albums(req.into_inner(), pagination.into_inner())
        .await?;
    let searched_albums = db.get_albums(&search_res).await?;
    Ok(HttpResponse::Ok().json(searched_albums.unwrap_or_default()))
}

pub async fn search_artist(
    req: web::Path<String>,
    pagination: web::Query<PaginationOptions>,
) -> MusicResponse {
    let db = get_mongo(None).await;
    let search = get_meilisearch(None).await;
    let _dz = get_dz_client(None).await.read().await;

    /*
    if pagination.get_page() == 0 {
        let res = dz.search_music(req.clone()).await?;
        let _ = index_search_musics_result(&res, scraper.get_ref(), IndexType::Artist).await;
    }
    //musics.group_by() */
    let search_res = search
        .search_artists(req.into_inner(), pagination.into_inner())
        .await?;
    let searched_artists = db.get_artists(&search_res).await?;
    Ok(HttpResponse::Ok().json(searched_artists.unwrap()))
}

pub async fn search_playlist(
    req: web::Path<String>,
    user: User,
    pagination: web::Query<PaginationOptions>,
) -> MusicResponse {
    let db = get_mongo(None).await;

    let playlists = db
        .search_playlist(req.into_inner(), &pagination)
        .await?
        .unwrap_or_default();

    let mut pop_playlists: Vec<PopulatedPlaylist> = Vec::with_capacity(playlists.len());
    for playlist in playlists.iter().cloned() {
        if playlist.is_authorized_read(&user.clone().id().unwrap()) {
            //Something else might be faster
            let musics = db.get_musics(&playlist.musics.as_ref().unwrap()).await?;
            //TODO add correct user..
            let mut playlist_pop = PopulatedPlaylist::from_playlist(playlist, user.clone());
            playlist_pop.musics = musics;
            pop_playlists.push(playlist_pop);
        }
    }

    Ok(HttpResponse::Ok().json(pop_playlists))
}
