use crate::{
    actors::ArtistScraperActor,
    db::{get_mongo, PaginationOptions},
    deezer::get_dz_client,
    models::{PopulatedPlaylist, User},
    search::get_meilisearch,
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
    let search = get_meilisearch(None).await;
    let dz = get_dz_client();

    if let Some(true) = search_opt.no_index {
        let musics = search
            .search_musics(req.into_inner(), pagination.into_inner())
            .await?;
        return Ok(HttpResponse::Ok().json(musics));
    }

    if pagination.get_page() == 0 {
        let res = dz.search_music(req.clone()).await?;
        let _ = index_search_musics_result(&res, scraper.get_ref(), IndexType::Music).await;
        //musics.group_by()
    }
    let musics = search
        .search_musics(req.into_inner(), pagination.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(musics))
}

pub async fn search_album(
    req: web::Path<String>,
    pagination: web::Query<PaginationOptions>,
) -> MusicResponse {
    let search = get_meilisearch(None).await;
    let _dz = get_dz_client();
    /*
    if pagination.get_page() == 0 {}
        let res = dz.search_music(req.clone()).await?;
        let _ = index_search_musics_result(&res, scraper.get_ref(), IndexType::Album).await;
    }
    //musics.group_by() */
    let search_res = search
        .search_albums(req.into_inner(), pagination.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(search_res))
}

pub async fn search_artist(
    req: web::Path<String>,
    pagination: web::Query<PaginationOptions>,
) -> MusicResponse {
    let search = get_meilisearch(None).await;
    let _dz = get_dz_client();

    /*
    if pagination.get_page() == 0 {
        let res = dz.search_music(req.clone()).await?;
        let _ = index_search_musics_result(&res, scraper.get_ref(), IndexType::Artist).await;
    }
    //musics.group_by() */
    let search_res = search
        .search_artists(req.into_inner(), pagination.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(search_res))
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
            let musics = db.get_musics(playlist.musics.as_ref().unwrap()).await?;
            //TODO add correct user..
            let mut playlist_pop = PopulatedPlaylist::from_playlist(playlist, user.clone().into());
            playlist_pop.musics = musics;
            pop_playlists.push(playlist_pop);
        }
    }

    Ok(HttpResponse::Ok().json(pop_playlists))
}
