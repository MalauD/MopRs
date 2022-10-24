use std::str::FromStr;

use actix_web::{http::header::Range, web, HttpRequest, HttpResponse};

use crate::{
    db::get_mongo,
    deezer::{get_dz_client, refresh_dz_client},
    models::User,
    s3::get_s3,
};

pub async fn get_music(
    req: web::Path<i32>,
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
