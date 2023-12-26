use crate::{
    actors::{DownloadSongMessage, DownloaderActor},
    db::get_mongo,
    deezer::{get_dz_client, get_dz_downloader, DeezerMusicFormats},
    models::{DeezerId, User},
    s3::get_s3,
    tools::MusicError,
};
use actix::Addr;
use actix_web::{
    http::header::{ByteRangeSpec, Range},
    web, HttpRequest, HttpResponse,
};

use id3::{
    frame::{Picture, PictureType},
    Tag, TagLike,
};
use log::debug;
use serde::Deserialize;
use std::{convert::TryInto, iter::FromIterator, str::FromStr};

#[derive(Debug, Deserialize)]
pub struct MusicFormat {
    pub format: Option<DeezerMusicFormats>,
}

pub async fn get_music_audio(
    req: web::Path<DeezerId>,
    user: User,
    httpreq: HttpRequest,
    query: web::Query<MusicFormat>,
    downloader_actor: web::Data<Addr<DownloaderActor>>,
) -> actix_web::Result<HttpResponse> {
    let db = get_mongo(None).await;
    let s3 = get_s3(None).await;
    let downloader = get_dz_downloader(None).read().unwrap();
    let id = req.into_inner();

    let formats = query
        .format
        .unwrap_or(user.prefered_format())
        .get_formats_below();

    let range = httpreq
        .headers()
        .get("range")
        .map(|v| Range::from_str(v.to_str().unwrap()).unwrap());

    let start_at = get_range_start(range.unwrap_or(Range::Bytes(vec![])));
    if start_at.unwrap_or(0) == 0 {
        db.add_to_history(&user, &id).await.unwrap();
    }

    let res = s3.get_music(id, &formats, start_at).await;
    if res.is_err() {
        let (stream, format, song_length, range) =
            if let Ok(d) = downloader.stream_music(id, &formats, start_at).await {
                d
            } else {
                drop(downloader);
                let mut downloader = get_dz_downloader(None).write().unwrap();
                downloader.authenticate().await.unwrap();
                downloader
                    .stream_music(id, &formats, start_at)
                    .await
                    .unwrap()
            };
        debug!(target : "mop-rs::cdn", "Streaming music {} from Deezer (format {:?})", id, format);

        if start_at.unwrap_or(0) == 0 {
            let _ = downloader_actor
                .send(DownloadSongMessage::new(id, formats))
                .await
                .unwrap();
        }
        let range = range.to_satisfiable_range(song_length).unwrap();
        stream_seek(
            range.0,
            range.1,
            song_length,
            format.get_mime_type(),
            stream,
        )
    } else {
        let (res, format, song_size) = res.unwrap();
        let start_at = start_at.unwrap_or(0);
        let stream_size = res.len() as u64;
        let stream = futures::stream::once(async move { Ok(bytes::Bytes::from_iter(res)) });
        stream_seek(
            start_at,
            start_at + stream_size - 1,
            song_size,
            format.get_mime_type(),
            stream,
        )
    }
}

fn get_range_start(range: Range) -> Option<u64> {
    if let Range::Bytes(ranges) = range {
        if let Some(a) = ranges.first() {
            return match a {
                ByteRangeSpec::From(start) => Some(*start),
                ByteRangeSpec::Last(_) => None,
                ByteRangeSpec::FromTo(start, _) => Some(*start),
            };
        }
    }
    return None;
}

fn stream_seek<T>(
    from: u64,
    to: u64,
    song_size: u64,
    mime_type: String,
    stream: T,
) -> Result<HttpResponse, actix_web::Error>
where
    T: futures::Stream<Item = Result<bytes::Bytes, MusicError>> + 'static,
{
    let range = format!("bytes {}-{}/{}", from, to, song_size);
    debug!(target : "mop-rs::cdn", "Streaming music with range {}", range);
    return Ok(HttpResponse::PartialContent()
        .append_header(("Content-Range", range))
        .append_header(("Accept-Ranges", "bytes"))
        .append_header(("Content-Type", mime_type))
        .streaming(stream));
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

    let formats = query
        .format
        .unwrap_or(user.prefered_format())
        .get_formats_below();

    let res = s3.get_music(id, &formats, None).await;
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
        (res.0, res.1)
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
