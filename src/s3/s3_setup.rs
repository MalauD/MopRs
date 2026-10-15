use log::info;
use s3::{creds::Credentials, error::S3Error, Bucket, BucketConfiguration, Region};
use std::{io, str::FromStr};

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
    pub async fn connect(s3_config: &S3Config) -> io::Result<Self> {
        let region = Region::Custom {
            region: s3_config.s3_region.clone(),
            endpoint: s3_config.s3_url.clone(),
        };

        let credentials = Credentials::from_env_specific(None, None, None, None).map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to load S3 credentials: {e}"),
            )
        })?;

        let bucket = match Bucket::create_with_path_style(
            &s3_config.s3_bucket,
            region.clone(),
            credentials,
            BucketConfiguration::private(),
        )
        .await
        {
            Ok(b) => b.bucket,
            Err(S3Error::Http(409, _)) => Bucket::new(
                &s3_config.s3_bucket,
                region,
                Credentials::from_env_specific(None, None, None, None).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        format!("Failed to load S3 credentials: {e}"),
                    )
                })?,
            )
            .map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!("Failed to open S3 bucket: {e}"),
                )
            })?
            .with_path_style(),
            Err(e) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Failed to create S3 bucket: {e}"),
                ))
            }
        };

        info!(target: "mop-rs::s3", "Connected to s3");
        Ok(Self { bucket })
    }

    pub fn get_bucket(&self) -> &Bucket {
        &self.bucket
    }

    pub async fn get_music(
        &self,
        id: DeezerId,
        allowed_formats: &[DeezerMusicFormats],
        start: Option<u64>,
    ) -> Result<(Vec<u8>, DeezerMusicFormats, u64), S3Error> {
        let bucket = self.get_bucket();
        let (res, _) = bucket
            .list_page(id.to_string(), None, None, None, None)
            .await?;
        let mut best_allowed_format = Option::None;
        let mut best_allowed_format_obj_size = 0;
        let mut best_allowed_format_obj_key = String::new();
        for r in res.contents {
            let format = if !r.key.contains('.') {
                DeezerMusicFormats::MP3_128
            } else {
                let Some(format) = r.key.rsplit('.').next() else {
                    continue;
                };
                let Ok(format) = DeezerMusicFormats::from_str(format) else {
                    continue;
                };
                format
            };
            if !allowed_formats.contains(&format) {
                continue;
            }
            if best_allowed_format.map_or(true, |best| best < format) {
                best_allowed_format = Some(format);
                best_allowed_format_obj_key = r.key;
                best_allowed_format_obj_size = r.size;
            }
        }

        let best_allowed_format = best_allowed_format.ok_or(S3Error::HttpFail)?;

        let res = bucket
            .get_object_range(best_allowed_format_obj_key, start.unwrap_or(0), None)
            .await?;

        Ok((
            res.bytes().to_vec(),
            best_allowed_format,
            best_allowed_format_obj_size,
        ))
    }

    pub async fn get_music_stream(
        &self,
        id: DeezerId,
        allowed_formats: &[DeezerMusicFormats],
    ) -> Result<(s3::request::ResponseDataStream, DeezerMusicFormats), S3Error> {
        let (res, _) = self
            .bucket
            .list_page(format!("/{}", id), None, None, None, None)
            .await?;
        let mut best_allowed_format = Option::None;
        let mut best_allowed_format_obj_key = String::new();
        for r in res.contents {
            let format = if !r.key.contains('.') {
                DeezerMusicFormats::MP3_128
            } else {
                let Some(format) = r.key.rsplit('.').next() else {
                    continue;
                };
                let Ok(format) = DeezerMusicFormats::from_str(format) else {
                    continue;
                };
                format
            };
            if !allowed_formats.contains(&format) {
                continue;
            }
            if best_allowed_format.map_or(true, |best| best < format) {
                best_allowed_format = Some(format);
                best_allowed_format_obj_key = r.key;
            }
        }

        let best_allowed_format = best_allowed_format.ok_or(S3Error::HttpFail)?;

        let res = self
            .bucket
            .get_object_stream(best_allowed_format_obj_key)
            .await?;
        Ok((res, best_allowed_format))
    }

    pub async fn upload_music(
        &self,
        id: DeezerId,
        format: DeezerMusicFormats,
        data: &[u8],
    ) -> Result<(), S3Error> {
        let bucket = self.get_bucket();
        let obj_key = format!("/{}.{}", id, format.to_string());
        bucket.put_object(obj_key, data).await?;
        Ok(())
    }
}
