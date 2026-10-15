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
        .iter()
        .cloned()
        .map(Artist::from)
        .unique_by(|x| x.id)
        .collect_vec();

    let albums: Vec<(DeezerId, Album)> = res
        .data
        .iter()
        .cloned()
        .map(|x| (x.artist.id, Album::from(x)))
        .unique_by(|x| x.1.id)
        .collect_vec();
    let musics: Vec<(DeezerId, Music)> = res
        .data
        .iter()
        .cloned()
        .map(|x| (x.album.id, Music::from(x)))
        .collect();
    let music_data: Vec<Music> = musics.iter().map(|(_, music)| music.clone()).collect();
    let album_data: Vec<Album> = albums.iter().map(|(_, album)| album.clone()).collect();

    let task_music_search = search
        .index_musics(&music_data)
        .await
        .map_err(|error| error.to_string())?;
    let task_album_search = search
        .index_albums(&album_data)
        .await
        .map_err(|error| error.to_string())?;
    let task_artist_search = search
        .index_artists(&artists)
        .await
        .map_err(|error| error.to_string())?;

    let _ = db.bulk_insert_musics(&music_data).await;
    let _ = db.bulk_insert_albums(&album_data).await;
    let _ = db.bulk_insert_artists(&artists).await;

    let album_relations: Vec<(DeezerId, DeezerId)> = musics
        .iter()
        .map(|(album_id, music)| (music.id, *album_id))
        .collect();
    let artist_relations: Vec<(DeezerId, DeezerId)> = albums
        .iter()
        .map(|(artist_id, album)| (album.id, *artist_id))
        .collect();
    let _ = db.bulk_append_to_albums(&album_relations).await;
    let _ = db.bulk_append_to_artists(&artist_relations).await;

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

    for artist in artists {
        artist_scraper_addr.do_send(ArtistScraperMessage(artist));
    }

    Ok(music_data)
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
    let musics_data: Vec<Music> = musics.iter().map(|(_, music)| music.clone()).collect();
    let _ = db.bulk_insert_musics(&musics_data).await;
    let _ = search.index_musics(&musics_data).await;
    let albums_data: Vec<Album> = albums.iter().map(|(_, album)| album.clone()).collect();
    let _ = db.bulk_insert_albums(&albums_data).await;
    let _ = search.index_albums(&albums_data).await;
    let album_relations: Vec<(DeezerId, DeezerId)> = musics
        .iter()
        .map(|(album_id, music)| (music.id, *album_id))
        .collect();
    let artist_relations: Vec<(DeezerId, DeezerId)> = albums
        .iter()
        .map(|(artist_id, album)| (album.id, *artist_id))
        .collect();
    let _ = db.bulk_append_to_albums(&album_relations).await;
    let _ = db.bulk_append_to_artists(&artist_relations).await;
    Ok(musics.into_iter().map(|x| x.1).collect())
}
