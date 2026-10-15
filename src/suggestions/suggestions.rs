use rand::{seq::SliceRandom, thread_rng};

use crate::{
    db::MongoClient,
    models::{DeezerId, User},
};

use super::get_related_to;

pub async fn get_suggestions_for(
    db: &MongoClient,
    user: User,
    memory: i64,
    like_hist_ratio: f32,
    novelty: f32,
    limit: i32,
) -> Vec<DeezerId> {
    let mut base_ids = db.get_liked_musics(&user, memory).await.unwrap_or_default();
    let mut hist = db
        .get_history(&user, (memory as f32 * like_hist_ratio) as i64)
        .await
        .unwrap_or_default();
    base_ids.append(&mut hist);

    let mut related = get_related_to(
        db,
        &base_ids,
        None,
        novelty / 5.,
        (novelty * limit as f32) as i32,
    )
    .await;

    // Mix base_ids and related*
    base_ids.shuffle(&mut thread_rng());
    base_ids.truncate(limit as usize);
    base_ids.append(&mut related);
    base_ids.dedup();
    base_ids.shuffle(&mut thread_rng());
    base_ids.truncate(limit as usize);
    base_ids
}
