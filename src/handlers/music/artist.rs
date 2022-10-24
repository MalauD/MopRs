use crate::{
    db::get_mongo,
    deezer::get_dz_client,
    models::{Album, Artist, PopulatedArtist},
};
use actix_web::{web, HttpResponse};
use chrono::{Duration, Utc};
use itertools::Itertools;

use super::{index_artist_top_tracks, MusicResponse};

pub async fn get_artist(req: web::Path<i32>) -> MusicResponse {
    let db = get_mongo(None).await;
    let dz = get_dz_client(None).await.read().await;
    let res = dz.get_artist_albums(&req).await?;
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
    let mut compl_artist = db.get_artist(&req).await?.unwrap();
    let albums_of_artist = db
        .get_albums(&compl_artist.albums.as_ref().unwrap())
        .await?
        .unwrap();
    let mut pop_artist = PopulatedArtist::from(compl_artist.clone());
    pop_artist.albums = Some(albums_of_artist);

    if Utc::now() - compl_artist.last_update > Duration::hours(1)
        || compl_artist.top_tracks.is_none()
        || compl_artist.related_artists.is_none()
    {
        let related = dz.get_related_artists(&req).await?;
        let top_tracks = dz.get_artist_top_tracks(&req).await?;

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
            .await?;
        compl_artist.top_tracks = Some(tracks.into_iter().map(|x| x.id).collect_vec());
        compl_artist.related_artists = Some(rel_artists.into_iter().map(|x| x.id).collect_vec());
    };
    let top_tracks = db
        .get_musics(&compl_artist.top_tracks.unwrap_or_default())
        .await?
        .unwrap();
    let related = db
        .get_artists(&compl_artist.related_artists.unwrap_or_default())
        .await?
        .unwrap();

    pop_artist.related_artists = Some(related);
    pop_artist.top_tracks = Some(top_tracks);

    Ok(HttpResponse::Ok().json(pop_artist))
}