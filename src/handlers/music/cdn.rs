use crate::{
    db::get_mongo,
    deezer::{get_dz_client, get_dz_downloader, DeezerMusicFormats},
    models::{DeezerId, User},
    s3::get_s3,
};
use actix_web::{http::header::Range, web, HttpRequest, HttpResponse};
use id3::{
    frame::{Picture, PictureType},
    Tag, TagLike,
};
use log::debug;
use serde::Deserialize;
use std::{convert::TryInto, str::FromStr};

#[derive(Debug, Deserialize)]
pub struct MusicFormat {
    #[serde(default)]
    pub format: DeezerMusicFormats,
}

pub async fn get_music_audio(
    req: web::Path<DeezerId>,
    user: User,
    httpreq: HttpRequest,
    query: web::Query<MusicFormat>,
) -> actix_web::Result<HttpResponse> {
    let db = get_mongo(None).await;
    let s3 = get_s3(None).await;
    let downloader = get_dz_downloader(None).read().unwrap();
    let id = req.into_inner();

    let formats = query.format.get_formats_below();

    let res = s3.get_music(id, &vec![query.format]).await;
    let (t, format) = if res.is_err() {
        let (track, format) = if let Ok(d) = downloader.download_music(id, &formats).await {
            d
        } else {
            drop(downloader);
            let mut downloader = get_dz_downloader(None).write().unwrap();
            downloader.authenticate().await.unwrap();
            downloader.download_music(id, &formats).await.unwrap()
        };
        let track_c = track.clone();
        actix_rt::spawn(async move {
            let _ = s3.upload_music(id, format, &track).await;
            debug!(target : "mop-rs::cdn", "Uploaded music {} to S3", id)
        });
        (track_c, format)
    } else {
        let res = res.unwrap();
        db.add_to_history(&user, &id).await.unwrap();
        res
    };

    let mime_type = format.get_mime_type();

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
                .append_header(("Content-Type", mime_type))
                .body(t[range.0 as usize..=range.1 as usize].to_vec()))
        } else {
            Ok(HttpResponse::Ok()
                .append_header(("Content-Type", mime_type))
                .body(t))
        }
    } else {
        Ok(HttpResponse::Ok()
            .append_header(("Content-Type", mime_type))
            .body(t))
    }
}

pub async fn get_music_tagged(
    req: web::Path<DeezerId>,
    user: User,
    query: web::Query<MusicFormat>,
) -> actix_web::Result<HttpResponse> {
    let db = get_mongo(None).await;
    let s3 = get_s3(None).await;
    let downloader = get_dz_downloader(None).read().unwrap();
    let id = req.into_inner();

    let formats = query.format.get_formats_below();

    let res = s3.get_music(id, &vec![query.format]).await;
    let (mut t, format) = if res.is_err() {
        let (track, format) = if let Ok(d) = downloader.download_music(id, &formats).await {
            d
        } else {
            drop(downloader);
            let mut downloader = get_dz_downloader(None).write().unwrap();
            downloader.authenticate().await.unwrap();
            downloader.download_music(id, &formats).await.unwrap()
        };
        let track_c = track.clone();
        actix_rt::spawn(async move {
            let _ = s3.upload_music(id, format, &track).await;
            debug!(target : "mop-rs::cdn", "Uploaded music {} to S3", id)
        });
        (track_c, format)
    } else {
        let res = res.unwrap();
        db.add_to_history(&user, &id).await.unwrap();
        res
    };

    if format == DeezerMusicFormats::FLAC {
        return Ok(HttpResponse::Ok()
            .append_header(("Content-Type", "audio/flac"))
            .body(t));
    }

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
        let dz = get_dz_client();
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
