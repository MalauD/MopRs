use crate::{
    actors::{ArtistScraperActor, ArtistScraperMessage},
    app_settings::AppSettings,
    db::MongoClient,
    deezer::DeezerClient,
    models::{Album, Artist, DeezerId, PopulatedArtist},
    search::MeilisearchClient,
};
use actix::Addr;
use actix_web::{web, HttpResponse};
use itertools::Itertools;

use super::{index_artist_top_tracks, MusicResponse};

pub async fn get_artist(
    req: web::Path<DeezerId>,
    db: web::Data<MongoClient>,
    dz: web::Data<DeezerClient>,
    search: web::Data<MeilisearchClient>,
    config: web::Data<AppSettings>,
    scraper: web::Data<Addr<ArtistScraperActor>>,
) -> MusicResponse {
    let config = config.get_ref();

    let Some(mut compl_artist) = db.get_artist(&req).await? else {
        return Ok(HttpResponse::NotFound().finish());
    };
    let mut pop_artist = PopulatedArtist::from(compl_artist.clone());

    if compl_artist.should_update(config.get_artist_update_interval()) {
        let res = dz.get_artist_albums(&req).await?;
        let albums: Vec<Album> = res
            .data
            .clone()
            .into_iter()
            .map(Album::from)
            .unique_by(|x| x.id)
            .collect_vec();
        let albums_id = albums.clone().into_iter().map(|x| x.id).collect_vec();
        let _ = db.bulk_insert_albums(albums.clone()).await;
        let _ = search.index_albums(albums).await?;
        let _ = db.append_multiple_to_an_artist(albums_id, &req).await;
        //musics.group_by()

        let related = dz.get_related_artists(&req).await?;
        let top_tracks = dz.get_artist_top_tracks(&req).await?;

        let rel_artists: Vec<Artist> = related
            .data
            .clone()
            .into_iter()
            .map(Artist::from)
            .unique_by(|x| x.id)
            .collect_vec();

        let _ = db.bulk_insert_artists(&rel_artists).await;
        let _ = search.index_artists(rel_artists.clone()).await?;

        let _ = db
            .set_related_artists(
                &req,
                rel_artists.clone().into_iter().map(|x| x.id).collect_vec(),
            )
            .await;

        let tracks = match index_artist_top_tracks(
            db.get_ref(),
            search.get_ref(),
            &top_tracks,
            &req,
        )
        .await
        {
            Ok(tracks) => tracks,
            Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
        };
        db.set_top_tracks(
            &req,
            &tracks.clone().into_iter().map(|x| x.id).collect_vec(),
        )
        .await?;
        compl_artist.top_tracks = Some(tracks.into_iter().map(|x| x.id).collect_vec());
        compl_artist.related_artists = Some(rel_artists.into_iter().map(|x| x.id).collect_vec());
    };

    let albums_of_artist = match compl_artist.albums.as_ref() {
        Some(album_ids) => db.get_albums(album_ids).await?.unwrap_or_default(),
        None => Vec::new(),
    };

    pop_artist.albums = Some(albums_of_artist);

    let top_tracks = db
        .get_musics(&compl_artist.top_tracks.unwrap_or_default())
        .await?
        .unwrap_or_default();
    let related = db
        .get_artists(&compl_artist.related_artists.unwrap_or_default())
        .await?
        .unwrap_or_default();

    for rel in related.clone() {
        scraper.do_send(ArtistScraperMessage(rel));
    }

    pop_artist.related_artists = Some(related);
    pop_artist.top_tracks = Some(top_tracks);

    Ok(HttpResponse::Ok().json(pop_artist))
}
