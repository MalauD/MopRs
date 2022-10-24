use actix_web::{web, HttpResponse};

use crate::{
    db::{get_mongo, PaginationOptions},
    deezer::get_dz_client,
    models::{PopulatedPlaylist, User},
};

use super::{index_search_musics_result, MusicResponse};

pub async fn search_music(
    req: web::Path<String>,
    pagination: web::Query<PaginationOptions>,
) -> MusicResponse {
    let db = get_mongo(None).await;
    let dz = get_dz_client(None).await.read().await;
    let res = dz.search_music(req.clone()).await?;
    let _ = index_search_musics_result(&res).await;
    //musics.group_by()
    let searched_musics = db.search_music(req.into_inner(), &pagination).await?;
    Ok(HttpResponse::Ok().json(searched_musics.unwrap_or_default()))
}

pub async fn search_album(
    req: web::Path<String>,
    pagination: web::Query<PaginationOptions>,
) -> MusicResponse {
    let db = get_mongo(None).await;
    let dz = get_dz_client(None).await.read().await;
    let res = dz.search_music(req.clone()).await?;
    let _ = index_search_musics_result(&res).await;
    //musics.group_by()
    let searched_albums = db.search_album(req.into_inner(), &pagination).await?;
    Ok(HttpResponse::Ok().json(searched_albums.unwrap_or_default()))
}

pub async fn search_artist(
    req: web::Path<String>,
    pagination: web::Query<PaginationOptions>,
) -> MusicResponse {
    let db = get_mongo(None).await;
    let dz = get_dz_client(None).await.read().await;
    let res = dz.search_music(req.clone()).await?;
    let _ = index_search_musics_result(&res).await;
    //musics.group_by()
    let searched_artists = db.search_artist(req.into_inner(), &pagination).await?;
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
