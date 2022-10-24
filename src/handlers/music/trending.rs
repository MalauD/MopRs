use actix_web::{web, HttpResponse};

use crate::{
    db::{get_mongo, PaginationOptions},
    deezer::{get_dz_client, SearchMusicsResult},
    models::Chart,
};

use super::{index_search_musics_result, MusicResponse};

pub async fn trending_musics(pagination: web::Query<PaginationOptions>) -> MusicResponse {
    let db = get_mongo(None).await;
    let dz = get_dz_client(None).await.read().await;

    let charts = db.get_chart_today().await?;
    let charts = match charts {
        Some(c) => c,
        None => {
            let chart = dz.get_most_popular().await?;
            let _ = index_search_musics_result(&SearchMusicsResult {
                data: chart.clone().tracks.data,
                next: None,
            })
            .await;
            let ch = Chart::from(chart);
            let _ = db.insert_chart(&ch).await?;
            ch
        }
    };
    let vec: Vec<i32> = pagination.trim_vec(&charts.musics);
    let musics = db.get_musics(&vec).await?;

    Ok(HttpResponse::Ok().json(musics))
}
