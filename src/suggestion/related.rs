use crate::db::get_mongo;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub async fn get_related_to(base_music_ids: &Vec<i32>, limit: i32) -> Vec<i32> {
    let db = get_mongo(None).await;
    let base_musics = db.get_musics(base_music_ids).await.unwrap().unwrap();
    let base_musics_artists = db
        .get_artists_by_name(&base_musics.iter().map(|m| m.artist_name.clone()).collect())
        .await
        .unwrap()
        .unwrap();

    let mut related_musics = Vec::new();
    for artist in base_musics_artists {
        related_musics.extend(artist.top_tracks.unwrap_or_default());
    }
    related_musics.sort();
    related_musics.dedup();
    related_musics.shuffle(&mut thread_rng());
    related_musics.truncate(limit as usize);
    related_musics
}
