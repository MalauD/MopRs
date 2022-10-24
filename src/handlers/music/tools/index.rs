use itertools::Itertools;

use crate::{
    db::get_mongo,
    deezer,
    models::{Album, Artist, Music},
};

pub async fn index_search_musics_result(
    res: &deezer::SearchMusicsResult,
) -> Result<Vec<Music>, String> {
    let db = get_mongo(None).await;
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
    Ok(musics.into_iter().map(|x| x.1).collect())
}

pub async fn index_artist_top_tracks(
    res: &deezer::ArtistTopTracksResult,
    artist_id: &i32,
) -> Result<Vec<Music>, String> {
    let db = get_mongo(None).await;
    let albums: Vec<(i32, Album)> = res
        .data
        .clone()
        .into_iter()
        .map(|x| (*artist_id, Album::from(x)))
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