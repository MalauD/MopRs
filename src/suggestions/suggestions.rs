use rand::{seq::SliceRandom, thread_rng};
use std::collections::HashSet;

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
    let mut base_pool: HashSet<DeezerId> = HashSet::new();

    if let Ok(likes) = db.get_liked_musics(&user, memory).await {
        base_pool.extend(likes);
    }

    let hist_limit = (memory as f32 * like_hist_ratio) as i64;
    if let Ok(hist) = db.get_history(&user, hist_limit).await {
        base_pool.extend(hist);
    }

    let base_ids: Vec<DeezerId> = base_pool.into_iter().collect();
    let related_limit = (novelty * limit as f32) as i32;

    let related = get_related_to(db, &base_ids, None, novelty / 5.0, related_limit).await;

    let mut final_pool: HashSet<DeezerId> = base_ids.into_iter().collect();
    final_pool.extend(related);

    let final_pool_vec: Vec<DeezerId> = final_pool.into_iter().collect();

    let mut rng = thread_rng();
    final_pool_vec
        .choose_multiple(&mut rng, limit as usize)
        .cloned()
        .collect()
}
