use crate::{
    db::MongoClient,
    deezer::DeezerClient,
    models::{DeezerId, Music, PopulatedAlbum},
    search::MeilisearchClient,
};
use actix_web::{web, HttpResponse};
use itertools::Itertools;

use super::MusicResponse;

pub async fn get_album(
    req: web::Path<DeezerId>,
    db: web::Data<MongoClient>,
    dz: web::Data<DeezerClient>,
    search: web::Data<MeilisearchClient>,
) -> MusicResponse {
    let res = dz.get_album_musics(*req).await?;
    let Some(album) = db.get_album(&req).await? else {
        return Ok(HttpResponse::NotFound().finish());
    };
    let musics: Vec<Music> = res
        .data
        .clone()
        .into_iter()
        .map(|x| Music {
            image_url: Some(album.cover.clone()),
            ..Music::from(x)
        })
        .unique_by(|x| x.id)
        .collect_vec();
    let music_ids = musics.clone().into_iter().map(|x| x.id).collect_vec();
    let _ = db.bulk_insert_musics(musics.clone()).await;
    let _ = search.index_musics(musics).await;
    let _ = db.set_album_musics(music_ids, &req).await;
    //musics.group_by()
    let Some(compl_album) = db.get_album(&req).await? else {
        return Ok(HttpResponse::NotFound().finish());
    };
    let musics_of_album = match compl_album.musics.as_ref() {
        Some(musics) => db.get_musics(musics).await?.unwrap_or_default(),
        None => Vec::new(),
    };
    let mut pop_album = PopulatedAlbum::from(compl_album);
    pop_album.musics = Some(musics_of_album);
    Ok(HttpResponse::Ok().json(pop_album))
}
