use crate::app_settings::get_settings_sync;
use crate::deezer::{get_dz_downloader, DeezerMusicFormats};
use crate::models::DeezerId;
use crate::s3::get_s3;
use crate::tools::MusicError;
use actix::{Actor, AsyncContext, Context, Handler, Message, WrapFuture};
use bson::de;
use log::{debug, error, info};
use std::time::Duration;

pub struct DownloaderActor;

impl DownloaderActor {
    async fn download_song(
        id: DeezerId,
        allowed_formats: Vec<DeezerMusicFormats>,
        pre_download_wait: Duration,
    ) {
        actix_rt::spawn(async move {
            debug!(target : "mop-rs::DownloaderActor", "Waiting {:?} before downloading {}", pre_download_wait,id);
            tokio::time::sleep(pre_download_wait).await;
            let downloader = get_dz_downloader(None).read().unwrap();
            let (song, format) = downloader
                .download_music(id, &allowed_formats)
                .await
                .unwrap();
            debug!(target : "mop-rs::DownloaderActor", "Downloaded music {} from Deezer (format {:?})", id, format);
            let s3 = get_s3(None).await;
            s3.upload_music(id, format, &song).await;
            info!(target : "mop-rs::DownloaderActor", "Uploaded music {} to S3 (format {:?})", id, format);
        });
    }
}

impl Actor for DownloaderActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!(target : "mop-rs::DownloaderActor","Started Downloader actor");
    }
}

#[derive(Message)]
#[rtype(result = "Result<(), MusicError>")]
pub struct DownloadSongMessage {
    pub(super) id: DeezerId,
    pub(super) allowed_formats: Vec<DeezerMusicFormats>,
}

impl DownloadSongMessage {
    pub fn new(id: DeezerId, allowed_formats: Vec<DeezerMusicFormats>) -> Self {
        Self {
            id,
            allowed_formats,
        }
    }
}

impl Handler<DownloadSongMessage> for DownloaderActor {
    type Result = Result<(), MusicError>;

    fn handle(&mut self, msg: DownloadSongMessage, ctx: &mut Self::Context) -> Self::Result {
        let fut = Box::pin(async move {
            let config = get_settings_sync();
            DownloaderActor::download_song(
                msg.id,
                msg.allowed_formats,
                config.predownload_wait_time,
            )
            .await;
        });

        let actor_fut = fut.into_actor(self);
        ctx.spawn(actor_fut);
        Ok(())
    }
}
