use crate::{app_settings::AppSettings, s3::S3Client};
use crate::deezer::{get_dz_downloader, DeezerMusicFormats};
use crate::models::DeezerId;
use crate::tools::MusicError;
use actix::{Actor, AsyncContext, Context, Handler, Message, WrapFuture};
use actix_web::web::Data;
use log::{debug, info};
use std::time::Duration;

pub struct DownloaderActor {
    s3: Data<S3Client>,
    config: AppSettings,
}

impl DownloaderActor {
    pub fn new(s3: Data<S3Client>, config: AppSettings) -> Self {
        Self { s3, config }
    }

    async fn download_song(
        s3: Data<S3Client>,
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
            s3.upload_music(id, format, &song).await.unwrap();
            info!(target : "mop-rs::DownloaderActor", "Uploaded music {} to S3 (format {:?})", id, format);
        });
    }
}

impl Actor for DownloaderActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
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
        let s3 = self.s3.clone();
        let predownload_wait_time = self.config.predownload_wait_time;
        let fut = Box::pin(async move {
            DownloaderActor::download_song(
                s3,
                msg.id,
                msg.allowed_formats,
                predownload_wait_time,
            )
            .await;
        });

        let actor_fut = fut.into_actor(self);
        ctx.spawn(actor_fut);
        Ok(())
    }
}
