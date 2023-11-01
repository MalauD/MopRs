use crate::db::get_mongo;
use crate::models::DeezerId;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub async fn get_related_to(
    base_music_ids: &Vec<DeezerId>,
    exclude: Option<&Vec<DeezerId>>,
    limit: i32,
) -> Vec<DeezerId> {
    let db = get_mongo(None).await;
    let base_musics = db.get_musics(base_music_ids).await.unwrap().unwrap();
    let base_musics_artists = db
        .get_artists_by_name(&base_musics.iter().map(|m| m.artist_name.clone()).collect())
        .await
        .unwrap()
        .unwrap();

    let related_artists = db
        .get_artists(
            &base_musics_artists
                .iter()
                .flat_map(|a| a.related_artists.clone().unwrap_or_default())
                .collect(),
        )
        .await
        .unwrap_or_default()
        .unwrap();

    let mut related_musics = Vec::new();
    for artist in base_musics_artists {
        related_musics.extend(artist.top_tracks.unwrap_or_default());
    }
    let mut related_musics_ext = Vec::new();
    for artist in related_artists {
        related_musics_ext.extend(artist.top_tracks.unwrap_or_default());
    }
    related_musics_ext.shuffle(&mut thread_rng());
    related_musics_ext.truncate(related_musics.len());

    related_musics.extend(related_musics_ext);
    related_musics.sort();
    if let Some(exclude) = exclude {
        remove_from_sorted_vec(&mut related_musics, exclude);
    }
    related_musics.dedup();
    related_musics.shuffle(&mut thread_rng());
    related_musics.truncate(limit as usize);
    related_musics
}

//O(nlog(n)) n: size of vec
fn remove_from_sorted_vec(sorted_vec: &mut Vec<DeezerId>, vec: &Vec<DeezerId>) {
    for i in vec {
        if let Ok(index) = sorted_vec.binary_search(i) {
            sorted_vec.swap_remove(index);
        }
    }
}
