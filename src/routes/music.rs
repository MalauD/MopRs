use crate::{
    db::get_mongo,
    deezer::DeezerClient,
    models::{Album, Artist, Music},
    tools::MusicError,
};
use actix_web::{get, web, HttpResponse};
use itertools::Itertools;

type MusicResponse = Result<HttpResponse, MusicError>;

pub fn config_music(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/Music").route(
        "/Search/Music/Name/{search_req}",
        web::get().to(search_music),
    ));
}

pub async fn search_music(
    req: web::Path<String>,
    deezer_api: web::Data<DeezerClient>,
) -> MusicResponse {
    let db = get_mongo().await;
    let res = deezer_api.search_music(req.into_inner()).await.unwrap();
    let artists: Vec<Artist> = res
        .data
        .clone()
        .into_iter()
        .map(|x| Artist::from(x))
        .collect();
    let albums: Vec<Album> = res
        .data
        .clone()
        .into_iter()
        .map(|x| Album::from(x))
        .collect();
    let musics: Vec<Music> = res
        .data
        .clone()
        .into_iter()
        .map(|x| Music::from(x))
        .collect();

    let _ = db.bulk_insert_musics(musics).await;
    let _ = db.bulk_insert_albums(albums).await;
    let _ = db.bulk_insert_artists(artists).await;

    //musics.group_by()

    Ok(HttpResponse::Ok().finish())
}
