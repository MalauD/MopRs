use crate::{
    db::get_mongo,
    deezer::get_dz_client,
    models::{DeezerId, Music, PopulatedAlbum},
    search::get_meilisearch,
};
use actix_web::{web, HttpResponse};
use itertools::Itertools;

use super::MusicResponse;

pub async fn get_album(req: web::Path<DeezerId>) -> MusicResponse {
    let db = get_mongo(None).await;
    let dz = get_dz_client(None).await.read().await;
    let search = get_meilisearch(None).await;

    let res = dz.get_album_musics(req.clone()).await?;
    let album = db.get_album(&req).await?.unwrap();
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
    let compl_album = db.get_album(&req).await?.unwrap();
    let musics_of_album = db
        .get_musics(&compl_album.musics.as_ref().unwrap())
        .await?
        .unwrap();
    let mut pop_album = PopulatedAlbum::from(compl_album);
    pop_album.musics = Some(musics_of_album);
    Ok(HttpResponse::Ok().json(pop_album))
}
