use actix::Addr;
use actix_web::{web, HttpResponse};

use crate::{
    actors::ArtistScraperActor,
    db::{get_mongo, PaginationOptions},
    deezer::{get_dz_client, SearchMusicsResult},
    models::{Chart, DeezerId},
};

use super::{index_search_musics_result, IndexType, MusicResponse};

pub async fn trending_musics(
    pagination: web::Query<PaginationOptions>,
    scraper: web::Data<Addr<ArtistScraperActor>>,
) -> MusicResponse {
    let db = get_mongo(None).await;
    let dz = get_dz_client(None).await.read().await;

    let charts = db.get_chart_today().await?;
    let charts = match charts {
        Some(c) => c,
        None => {
            let chart = dz.get_most_popular().await?;
            let _ = index_search_musics_result(
                &SearchMusicsResult {
                    data: chart.clone().tracks.data,
                    next: None,
                },
                scraper.get_ref(),
                IndexType::None,
            )
            .await;
            let ch = Chart::from(chart);
            db.insert_chart(&ch).await?;
            ch
        }
    };
    let vec: Vec<DeezerId> = pagination.trim_vec(&charts.musics);
    let musics = db.get_musics(&vec).await?;

    Ok(HttpResponse::Ok().json(musics))
}
