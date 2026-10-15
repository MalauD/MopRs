use crate::db::MongoClient;
use crate::models::DeezerId;
use log::warn;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub async fn get_related_to(
    db: &MongoClient,
    base_music_ids: &[DeezerId],
    exclude: Option<&[DeezerId]>,
    related_artist_music_ratio: f32,
    limit: i32,
) -> Vec<DeezerId> {
    let base_musics = match db.get_musics(base_music_ids).await {
        Ok(Some(musics)) => musics,
        Ok(None) => return Vec::new(),
        Err(error) => {
            warn!(
                "Failed to load base musics for related suggestions: {}",
                error
            );
            return Vec::new();
        }
    };
    let base_musics_artists = db
        .get_artists_by_name(
            &base_musics
                .iter()
                .map(|m| m.artist_name.clone())
                .collect::<Vec<_>>(),
        )
        .await;
    let base_musics_artists = match base_musics_artists {
        Ok(Some(artists)) => artists,
        Ok(None) => return Vec::new(),
        Err(error) => {
            warn!(
                "Failed to load base artists for related suggestions: {}",
                error
            );
            return Vec::new();
        }
    };

    let related_artists = db
        .get_artists(
            &base_musics_artists
                .iter()
                .flat_map(|a| a.related_artists.clone().unwrap_or_default())
                .collect::<Vec<_>>(),
        )
        .await;
    let related_artists = match related_artists {
        Ok(Some(artists)) => artists,
        Ok(None) => Vec::new(),
        Err(error) => {
            warn!("Failed to load related artists for suggestions: {}", error);
            Vec::new()
        }
    };

    let mut related_musics = Vec::new();
    for artist in base_musics_artists {
        related_musics.extend(artist.top_tracks.unwrap_or_default());
    }
    let mut related_musics_ext = Vec::new();
    for artist in related_artists {
        related_musics_ext.extend(artist.top_tracks.unwrap_or_default());
    }
    related_musics_ext.shuffle(&mut thread_rng());
    related_musics_ext.truncate(related_musics.len() * related_artist_music_ratio as usize);

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

fn remove_from_sorted_vec(sorted_vec: &mut Vec<DeezerId>, vec: &[DeezerId]) {
    for i in vec {
        if let Ok(index) = sorted_vec.binary_search(i) {
            sorted_vec.remove(index);
        }
    }
}
