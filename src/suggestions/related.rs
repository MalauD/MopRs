use crate::db::MongoClient;
use crate::models::{Artist, DeezerId};
use log::warn;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;

pub async fn get_related_to(
    db: &MongoClient,
    base_music_ids: &[DeezerId],
    exclude: Option<&[DeezerId]>,
    related_artist_music_ratio: f32,
    limit: i32,
) -> Vec<DeezerId> {
    let base_artists = fetch_base_artists(db, base_music_ids).await;
    if base_artists.is_empty() {
        return Vec::new();
    }

    let related_artists = fetch_related_artists(db, &base_artists).await;

    let mut direct_top_tracks = extract_top_tracks(&base_artists);
    let extended_top_tracks = extract_top_tracks(&related_artists);

    let extended_limit = (direct_top_tracks.len() as f32 * related_artist_music_ratio) as usize;

    let mut rng = thread_rng();
    let sampled_extended: Vec<DeezerId> = extended_top_tracks
        .into_iter()
        .collect::<Vec<_>>()
        .choose_multiple(&mut rng, extended_limit)
        .cloned()
        .collect();

    direct_top_tracks.extend(sampled_extended);

    if let Some(exclude_ids) = exclude {
        let exclude_set: HashSet<&DeezerId> = exclude_ids.iter().collect();
        direct_top_tracks.retain(|id| !exclude_set.contains(id));
    }

    let direct_top_tracks_vec: Vec<DeezerId> = direct_top_tracks.into_iter().collect();
    direct_top_tracks_vec
        .choose_multiple(&mut rng, limit as usize)
        .cloned()
        .collect()
}

async fn fetch_base_artists(db: &MongoClient, music_ids: &[DeezerId]) -> Vec<Artist> {
    let musics = match db.get_musics(music_ids).await {
        Ok(Some(m)) => m,
        _ => {
            warn!("Failed to load base musics for related suggestions");
            return Vec::new();
        }
    };

    let artist_names: Vec<String> = musics.into_iter().map(|m| m.artist_name).collect();

    match db.get_artists_by_name(&artist_names).await {
        Ok(Some(artists)) => artists,
        _ => {
            warn!("Failed to load base artists for related suggestions");
            Vec::new()
        }
    }
}

async fn fetch_related_artists(db: &MongoClient, base_artists: &[Artist]) -> Vec<Artist> {
    let related_ids: Vec<DeezerId> = base_artists
        .iter()
        .filter_map(|a| a.related_artists.clone())
        .flatten()
        .collect();

    match db.get_artists(&related_ids).await {
        Ok(Some(artists)) => artists,
        _ => {
            warn!("Failed to load related artists for suggestions");
            Vec::new()
        }
    }
}

fn extract_top_tracks(artists: &[Artist]) -> HashSet<DeezerId> {
    artists
        .iter()
        .filter_map(|a| a.top_tracks.clone())
        .flatten()
        .collect()
}
