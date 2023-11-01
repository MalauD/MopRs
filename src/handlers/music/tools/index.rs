use actix::Addr;
use itertools::Itertools;

use crate::{
    actors::{ArtistScraperActor, ArtistScraperMessage},
    db::get_mongo,
    deezer,
    models::{Album, Artist, DeezerId, Music},
    search::get_meilisearch,
};

pub enum IndexType {
    Music,
    Album,
    Artist,
    None,
}

pub async fn index_search_musics_result(
    res: &deezer::SearchMusicsResult,
    artist_scraper_addr: &Addr<ArtistScraperActor>,
    wait_for_search_index: IndexType,
) -> Result<Vec<Music>, String> {
    let db = get_mongo(None).await;
    let search = get_meilisearch(None).await;

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

    let task_music_search = search.index_musics(music_data.clone()).await.unwrap();
    let task_album_search = search.index_albums(album_data.clone()).await.unwrap();
    let task_artist_search = search.index_artists(artists.clone()).await.unwrap();

    let _ = db.bulk_insert_musics(music_data).await;
    let _ = db.bulk_insert_albums(album_data).await;
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
    res: &deezer::ArtistTopTracksResult,
    artist_id: &DeezerId,
) -> Result<Vec<Music>, String> {
    let db = get_mongo(None).await;
    let search = get_meilisearch(None).await;

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
