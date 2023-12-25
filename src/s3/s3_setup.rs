use log::info;
use once_cell::sync::OnceCell;
use s3::{creds::Credentials, error::S3Error, Bucket, BucketConfiguration, Region};
use std::str::FromStr;
use tokio::sync::Mutex;

use crate::{deezer::DeezerMusicFormats, models::DeezerId};

pub struct S3Client {
    bucket: Bucket,
}

pub struct S3Config {
    pub s3_url: String,
    pub s3_region: String,
    pub s3_bucket: String,
}

impl S3Client {
    pub fn get_bucket(&self) -> &Bucket {
        &self.bucket
    }

    pub async fn get_music(
        &self,
        id: DeezerId,
        allowed_formats: &Vec<DeezerMusicFormats>,
    ) -> Result<(Vec<u8>, DeezerMusicFormats), S3Error> {
        let bucket = self.get_bucket();
        let (res, _) = bucket
            .list_page(format!("/{}", id), None, None, None, None)
            .await
            .unwrap();
        let mut best_allowed_format = Option::None;
        let mut best_allowed_format_obj_key = String::new();
        for r in res.contents {
            let format = if !r.key.contains(".") {
                DeezerMusicFormats::MP3_128
            } else {
                let format = r.key.split(".").last().unwrap();
                DeezerMusicFormats::from_str(&format).unwrap()
            };
            if !allowed_formats.contains(&format) {
                continue;
            }
            if best_allowed_format.is_none() || best_allowed_format.unwrap() < format {
                best_allowed_format = Some(format);
                best_allowed_format_obj_key = r.key;
            }
        }

        if best_allowed_format.is_none() {
            return Err(S3Error::HttpFail);
        }

        let res = bucket.get_object(best_allowed_format_obj_key).await?;
        Ok((res.bytes().to_vec(), best_allowed_format.unwrap()))
    }

    pub async fn get_music_stream(
        &self,
        id: DeezerId,
        allowed_formats: &Vec<DeezerMusicFormats>,
    ) -> Result<(s3::request::ResponseDataStream, DeezerMusicFormats), S3Error> {
        let (res, _) = self
            .bucket
            .list_page(format!("/{}", id), None, None, None, None)
            .await
            .unwrap();
        let mut best_allowed_format = Option::None;
        let mut best_allowed_format_obj_key = String::new();
        for r in res.contents {
            let format = if !r.key.contains(".") {
                DeezerMusicFormats::MP3_128
            } else {
                let format = r.key.split(".").last().unwrap();
                DeezerMusicFormats::from_str(&format).unwrap()
            };
            if !allowed_formats.contains(&format) {
                continue;
            }
            if best_allowed_format.is_none() || best_allowed_format.unwrap() < format {
                best_allowed_format = Some(format);
                best_allowed_format_obj_key = r.key;
            }
        }

        if best_allowed_format.is_none() {
            return Err(S3Error::HttpFail);
        }

        let res = self
            .bucket
            .get_object_stream(best_allowed_format_obj_key)
            .await?;
        Ok((res, best_allowed_format.unwrap()))
    }

    pub async fn upload_music(
        &self,
        id: DeezerId,
        format: DeezerMusicFormats,
        data: &Vec<u8>,
    ) -> Result<(), S3Error> {
        let bucket = self.get_bucket();
        let obj_key = format!("/{}.{}", id, format.to_string());
        bucket.put_object(obj_key, data).await?;
        Ok(())
    }
}

static S3: OnceCell<S3Client> = OnceCell::new();
static S3_INITIALIZED: OnceCell<Mutex<bool>> = OnceCell::new();

pub async fn get_s3(s3_config: Option<S3Config>) -> &'static S3Client {
    if let Some(c) = S3.get() {
        return c;
    }
    let initializing_mutex = S3_INITIALIZED.get_or_init(|| tokio::sync::Mutex::new(false));

    let mut initialized = initializing_mutex.lock().await;

    if !*initialized {
        let s3_config = s3_config.unwrap();
        let s3_url = s3_config.s3_url;
        let s3_region = s3_config.s3_region;

        let bucket = match Bucket::create_with_path_style(
            &s3_config.s3_bucket,
            Region::Custom {
                region: s3_region.clone(),
                endpoint: s3_url.clone(),
            },
            Credentials::from_env_specific(None, None, None, None).unwrap(),
            BucketConfiguration::private(),
        )
        .await
        {
            Ok(b) => b.bucket,
            Err(S3Error::Http(409, _)) => Bucket::new(
                "moprs",
                Region::Custom {
                    region: s3_region.clone(),
                    endpoint: s3_url.clone(),
                },
                Credentials::from_env_specific(None, None, None, None).unwrap(),
            )
            .unwrap()
            .with_path_style(),
            Err(e) => {
                panic!("Failed to create bucket: {}", e)
            }
        };
        if S3.set(S3Client { bucket }).is_ok() {
            info!(target: "mop-rs::s3", "Connected to s3");
            *initialized = true;
        }
    }
    drop(initialized);
    S3.get().unwrap()
}
