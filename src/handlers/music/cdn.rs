use actix_web::{http::header::Range, web, HttpRequest, HttpResponse};
use id3::{
    frame::{Picture, PictureType},
    Tag, TagLike,
};
use std::{convert::TryInto, str::FromStr};

use crate::{
    db::get_mongo,
    deezer::{get_dz_client, refresh_dz_client},
    models::{DeezerId, User},
    s3::get_s3,
};

pub async fn get_music(
    req: web::Path<DeezerId>,
    user: User,
    httpreq: HttpRequest,
) -> actix_web::Result<HttpResponse> {
    let db = get_mongo(None).await;
    let bucket = get_s3(None).await.get_bucket();
    let dz = get_dz_client(None).await.read().await;
    let id = req.into_inner();

    let res = bucket.get_object(format!("/{}", id)).await;
    let t = if res.is_err() {
        if let Ok(track) = dz.download_music(id).await {
            track
        } else {
            drop(dz);
            refresh_dz_client().await;
            let dz = get_dz_client(None).await.read().await;
            dz.download_music(id).await.unwrap()
        }
    } else {
        let res = res.unwrap();
        db.add_to_history(&user, &id).await.unwrap();
        res.bytes().to_vec()
    };

    if let Some(r) = httpreq.headers().get("range") {
        let range = Range::from_str(r.to_str().unwrap()).unwrap();
        if let Range::Bytes(ranges) = range {
            let range = ranges
                .first()
                .unwrap()
                .to_satisfiable_range(t.len() as u64)
                .unwrap();
            Ok(HttpResponse::PartialContent()
                .append_header((
                    "Content-Range",
                    format!("bytes {}-{}/{}", range.0, range.1, t.len()),
                ))
                .append_header(("Accept-Ranges", "bytes"))
                .append_header(("Content-Type", "audio/mpeg"))
                .body(t[range.0 as usize..=range.1 as usize].to_vec()))
        } else {
            Ok(HttpResponse::Ok()
                .append_header(("Content-Type", "audio/mpeg"))
                .body(t))
        }
    } else {
        Ok(HttpResponse::Ok()
            .append_header(("Content-Type", "audio/mpeg"))
            .body(t))
    }
}

pub async fn get_music_tagged(
    req: web::Path<DeezerId>,
    user: User,
    httpreq: HttpRequest,
) -> actix_web::Result<HttpResponse> {
    let db = get_mongo(None).await;
    let bucket = get_s3(None).await.get_bucket();
    let dz = get_dz_client(None).await.read().await;
    let id = req.into_inner();

    let res = bucket.get_object(format!("/{}", id)).await;
    let mut t = if res.is_err() {
        if let Ok(track) = dz.download_music(id).await {
            track
        } else {
            drop(dz);
            refresh_dz_client().await;
            let dz = get_dz_client(None).await.read().await;
            dz.download_music(id).await.unwrap()
        }
    } else {
        let res = res.unwrap();
        db.add_to_history(&user, &id).await.unwrap();
        res.bytes().to_vec()
    };

    let db_music = db.get_musics(&vec![id]).await.unwrap().unwrap();
    let db_music = db_music.first().unwrap();

    let db_album = db.get_album_of_music(&id).await.unwrap().unwrap();

    let mut tags = Tag::new();
    tags.set_title(db_music.title.clone());
    tags.set_artist(db_music.artist_name.clone());
    tags.set_album(db_album.name);
    if let Some(track) = db_music.track_number {
        tags.set_track(track.try_into().unwrap());
    }
    if let Some(disc) = db_music.disc_number {
        tags.set_disc(disc.try_into().unwrap());
    }
    if let Some(cover_url) = db_music.image_url.clone() {
        let dz = get_dz_client(None).await.read().await;
        let data = dz.get_cover(&cover_url).await.unwrap();
        tags.add_frame(Picture {
            mime_type: "image/jpeg".to_string(),
            picture_type: PictureType::CoverFront,
            description: cover_url,
            data,
        });
    }
    let mut tagged = Vec::new();
    tags.write_to(&mut tagged, id3::Version::Id3v22).unwrap();
    tagged.append(&mut t);
    Ok(HttpResponse::Ok()
        .append_header(("Content-Type", "audio/mpeg"))
        .body(tagged))
}
