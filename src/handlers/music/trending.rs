use actix::Addr;
use actix_web::{web, HttpResponse};

use crate::{
    actors::ArtistScraperActor,
    db::{MongoClient, PaginationOptions},
    deezer::{get_dz_client, SearchMusicsResult},
    models::{Chart, DeezerId},
};

use super::{index_search_musics_result, IndexType, MusicResponse};

pub async fn trending_musics(
    pagination: web::Query<PaginationOptions>,
    db: web::Data<MongoClient>,
    scraper: web::Data<Addr<ArtistScraperActor>>,
) -> MusicResponse {
    let dz = get_dz_client();

    let charts = db.get_chart_today().await?;
    let charts = match charts {
        Some(c) => c,
        None => {
            let chart = dz.get_most_popular().await?;
            let _ = index_search_musics_result(
                db.get_ref(),
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
