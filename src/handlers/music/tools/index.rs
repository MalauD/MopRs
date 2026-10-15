use actix::Addr;
use itertools::Itertools;

use crate::{
    actors::{ArtistScraperActor, ArtistScraperMessage},
    db::MongoClient,
    deezer,
    models::{Album, Artist, DeezerId, Music},
    search::MeilisearchClient,
};

pub enum IndexType {
    Music,
    #[allow(dead_code)]
    Album,
    #[allow(dead_code)]
    Artist,
    None,
}

pub async fn index_search_musics_result(
    db: &MongoClient,
    search: &MeilisearchClient,
    res: &deezer::SearchMusicsResult,
    artist_scraper_addr: &Addr<ArtistScraperActor>,
    wait_for_search_index: IndexType,
) -> Result<Vec<Music>, String> {
    let artists: Vec<Artist> = res
        .data
        .clone()
        .into_iter()
        .map(Artist::from)
        .unique_by(|x| x.id)
        .collect_vec();

    let albums: Vec<(DeezerId, Album)> = res
        .data
        .clone()
        .into_iter()
        .map(|x| (x.artist.id, Album::from(x)))
        .unique_by(|x| x.1.id)
        .collect_vec();
    let musics: Vec<(DeezerId, Music)> = res
        .data
        .clone()
        .into_iter()
        .map(|x| (x.album.id, Music::from(x)))
        .collect();
    let music_data: Vec<Music> = musics.clone().into_iter().map(|x| x.1).collect();
    let album_data: Vec<Album> = albums.clone().into_iter().map(|x| x.1).collect();

    let task_music_search = search
        .index_musics(music_data.clone())
        .await
        .map_err(|error| error.to_string())?;
    let task_album_search = search
        .index_albums(album_data.clone())
        .await
        .map_err(|error| error.to_string())?;
    let task_artist_search = search
        .index_artists(artists.clone())
        .await
        .map_err(|error| error.to_string())?;

    let _ = db.bulk_insert_musics(music_data).await;
    let _ = db.bulk_insert_albums(album_data).await;
    let _ = db.bulk_insert_artists(&artists).await;

    for val in &musics {
        let _ = db.append_to_album(&val.1.id, &val.0).await;
    }
    for val in &albums {
        let _ = db.append_to_artist(&val.1.id, &val.0).await;
    }

    match wait_for_search_index {
        IndexType::Music => {
            let _ = search.wait_for_task(task_music_search).await;
        }
        IndexType::Album => {
            let _ = search.wait_for_task(task_album_search).await;
        }
        IndexType::Artist => {
            let _ = search.wait_for_task(task_artist_search).await;
        }
        IndexType::None => {}
    }

    for artist in artists.clone() {
        artist_scraper_addr.do_send(ArtistScraperMessage(artist));
    }

    Ok(musics.into_iter().map(|x| x.1).collect())
}

pub async fn index_artist_top_tracks(
    db: &MongoClient,
    search: &MeilisearchClient,
    res: &deezer::ArtistTopTracksResult,
    artist_id: &DeezerId,
) -> Result<Vec<Music>, String> {
    let albums: Vec<(DeezerId, Album)> = res
        .data
        .clone()
        .into_iter()
        .map(|x| (*artist_id, Album::from(x)))
        .unique_by(|x| x.1.id)
        .collect_vec();
    let musics: Vec<(DeezerId, Music)> = res
        .data
        .clone()
        .into_iter()
        .map(|x| (x.album.id, Music::from(x)))
        .collect();
    let musics_data: Vec<Music> = musics.clone().into_iter().map(|x| x.1).collect();
    let _ = db.bulk_insert_musics(musics_data.clone()).await;
    let _ = search.index_musics(musics_data).await;
    let albums_data: Vec<Album> = albums.clone().into_iter().map(|x| x.1).collect();
    let _ = db.bulk_insert_albums(albums_data.clone()).await;
    let _ = search.index_albums(albums_data).await;
    for val in &musics {
        let _ = db.append_to_album(&val.1.id, &val.0).await;
    }
    for val in &albums {
        let _ = db.append_to_artist(&val.1.id, &val.0).await;
    }
    Ok(musics.into_iter().map(|x| x.1).collect())
}
