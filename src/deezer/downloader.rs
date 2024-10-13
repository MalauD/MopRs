use actix_web::http::header::ByteRangeSpec;
use block_modes::{block_padding::NoPadding, BlockMode, Cbc};
use blowfish::Blowfish;
use futures::TryStreamExt;
use itertools::Itertools;
use log::debug;
use once_cell::sync::OnceCell;
use pin_project::pin_project;
use reqwest::{cookie::Jar, Client, Url};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::{serde_as, DisplayFromStr};
use std::sync::{Arc, Mutex, RwLock};
use thiserror::Error;

use crate::{models::DeezerId, tools::MusicError};

#[derive(Error, Debug)]
pub enum DeezerDownloaderError {
    #[error("Something went wrong with the request")]
    ApiBackendError(#[from] reqwest::Error),
    #[error("Invalid arl token")]
    InvalidArlToken,
    #[error("Session expired")]
    SessionExpired,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeezerUnofficialMusic {
    #[serde(rename = "SNG_ID")]
    id: String,
    #[serde(rename = "TRACK_TOKEN")]
    token: String,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "FILESIZE_MP3_128")]
    size_mp3_128: u64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "FILESIZE_MP3_320")]
    size_mp3_320: u64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "FILESIZE_FLAC")]
    size_flac: u64,
}

impl DeezerUnofficialMusic {
    pub fn get_size(&self, format: &DeezerMusicFormats) -> u64 {
        match format {
            DeezerMusicFormats::MP3_128 => self.size_mp3_128,
            DeezerMusicFormats::MP3_320 => self.size_mp3_320,
            DeezerMusicFormats::FLAC => self.size_flac,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeezerUnofficialMedia {
    format: DeezerMusicFormats,
    sources: Vec<DeezerUnofficialMediaSource>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeezerMusicFormatCiper {
    cipher: String,
    format: DeezerMusicFormats,
}

impl DeezerMusicFormatCiper {
    pub fn new(cipher: String, format: DeezerMusicFormats) -> Self {
        Self { cipher, format }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeezerUnofficialMediaSource {
    url: String,
}

type Result<T> = std::result::Result<T, DeezerDownloaderError>;

pub struct DeezerDownloader {
    token: String,
    license_token: String,
    http_client: Client,
    connected: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DeezerMusicFormats {
    MP3_128,
    MP3_320,
    FLAC,
}

impl Default for DeezerMusicFormats {
    fn default() -> Self {
        DeezerMusicFormats::MP3_128
    }
}

impl DeezerMusicFormats {
    pub fn get_formats_below(&self) -> Vec<DeezerMusicFormats> {
        match self {
            DeezerMusicFormats::MP3_128 => vec![DeezerMusicFormats::MP3_128],
            DeezerMusicFormats::MP3_320 => {
                vec![DeezerMusicFormats::MP3_128, DeezerMusicFormats::MP3_320]
            }
            DeezerMusicFormats::FLAC => vec![
                DeezerMusicFormats::FLAC,
                DeezerMusicFormats::MP3_320,
                DeezerMusicFormats::MP3_128,
            ],
        }
    }

    pub fn get_mime_type(&self) -> String {
        match self {
            DeezerMusicFormats::MP3_128 => "audio/mpeg".to_string(),
            DeezerMusicFormats::MP3_320 => "audio/mpeg".to_string(),
            DeezerMusicFormats::FLAC => "audio/flac".to_string(),
        }
    }
}

impl std::str::FromStr for DeezerMusicFormats {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "MP3_128" => Ok(DeezerMusicFormats::MP3_128),
            "MP3_320" => Ok(DeezerMusicFormats::MP3_320),
            "FLAC" => Ok(DeezerMusicFormats::FLAC),
            _ => Err(()),
        }
    }
}

impl DeezerMusicFormats {
    pub fn to_string(&self) -> String {
        match self {
            DeezerMusicFormats::MP3_128 => "MP3_128".to_string(),
            DeezerMusicFormats::MP3_320 => "MP3_320".to_string(),
            DeezerMusicFormats::FLAC => "FLAC".to_string(),
        }
    }
}

impl DeezerUnofficialMusic {
    pub fn get_bf_key(&self) -> String {
        let secret = "g4el58wc0zvf9na1";
        let md5_music_id = hex::encode(md5::compute(self.id.to_string().as_bytes()).0);
        let mut blowfish_key = String::new();
        for i in 0..16 {
            blowfish_key.push_str(
                &String::from_utf16(&[md5_music_id.chars().nth(i).unwrap() as u16
                    ^ md5_music_id.chars().nth(i + 16).unwrap() as u16
                    ^ secret.chars().nth(i).unwrap() as u16])
                .unwrap(),
            );
        }
        blowfish_key
    }
}

static DOWNLOADER: OnceCell<RwLock<DeezerDownloader>> = OnceCell::new();
static DOWNLOADER_INITIALIZED: OnceCell<Mutex<bool>> = OnceCell::new();

pub fn get_dz_downloader(arl: Option<String>) -> &'static RwLock<DeezerDownloader> {
    if let Some(c) = DOWNLOADER.get() {
        return c;
    }

    let initializing_mutex = DOWNLOADER_INITIALIZED.get_or_init(|| Mutex::new(false));

    let mut initialized = initializing_mutex.lock().unwrap();

    if !*initialized {
        let client = DeezerDownloader::new(arl.unwrap());
        if DOWNLOADER.set(RwLock::new(client)).is_ok() {
            *initialized = true;
        }
    }

    drop(initialized);
    DOWNLOADER.get().unwrap()
}

impl DeezerDownloader {
    pub fn new(arl: String) -> Self {
        let cookie_jar = Arc::new(Jar::default());
        cookie_jar.add_cookie_str(
            &format!("arl={}; Domain=www.deezer.com; Path=/", arl),
            &Url::parse("http://www.deezer.com").unwrap(),
        );

        Self {
            http_client: Client::builder()
                .cookie_provider(cookie_jar)
                .gzip(true)
                .build()
                .unwrap(),
            token: String::new(),
            license_token: String::new(),
            connected: false,
        }
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub async fn authenticate(&mut self) -> Result<()> {
        let response = self
            .http_client
            .get("http://www.deezer.com/ajax/gw-light.php?api_token=null&method=deezer.getUserData&api_version=1.0&input=3")
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        if response["results"]["USER"]["USER_ID"]
            .as_i64()
            .ok_or(DeezerDownloaderError::InvalidArlToken)?
            == 0
        {
            return Err(DeezerDownloaderError::InvalidArlToken);
        }

        self.token = response["results"]["checkForm"]
            .as_str()
            .ok_or(DeezerDownloaderError::InvalidArlToken)?
            .to_string();
        self.license_token = response["results"]["USER"]["OPTIONS"]["license_token"]
            .as_str()
            .ok_or(DeezerDownloaderError::InvalidArlToken)?
            .to_string();
        self.connected = true;
        Ok(())
    }

    async fn get_music_by_id(&self, id: DeezerId) -> Result<DeezerUnofficialMusic> {
        let url = format!(
            "http://www.deezer.com/ajax/gw-light.php?api_token={}&api_version=1.0&input=3&method=song.getListData",
            self.token
        );

        let body = json!({
            "sng_ids": vec![id.to_string()],
        });

        let res = self
            .http_client
            .post(url)
            .json(&body)
            .header(reqwest::header::ACCEPT, "application/json")
            .header(reqwest::header::CONNECTION, "keep-alive")
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let sng = res["results"]["data"]
            .as_array()
            .ok_or(DeezerDownloaderError::SessionExpired)?
            .first()
            .ok_or(DeezerDownloaderError::SessionExpired)?;
        // Check if there is a FALLBACK entry
        if sng["FALLBACK"] != serde_json::Value::Null {
            if let Ok(fallback) =
                serde_json::from_value::<DeezerUnofficialMusic>(sng["FALLBACK"].clone())
            {
                debug!(target: "mop-rs::deezer_downloader", "Fallback entry ({}) found for {}", fallback.id, id);
                return Ok(fallback);
            }
        }
        // if the value is null, it means the session expired
        Ok(serde_json::from_value(
            res["results"]["data"]
                .as_array()
                .ok_or(DeezerDownloaderError::SessionExpired)?
                .first()
                .ok_or(DeezerDownloaderError::SessionExpired)?
                .clone(),
        )
        .unwrap())
    }

    async fn get_music_download_url(
        &self,
        music: &DeezerUnofficialMusic,
        allowed_formats: &Vec<DeezerMusicFormats>,
    ) -> Result<DeezerUnofficialMedia> {
        let url = "https://media.deezer.com/v1/get_url";
        let format_sorted: Vec<&DeezerMusicFormats> =
            allowed_formats.iter().sorted_by(|a, b| b.cmp(a)).collect();

        let format = format_sorted
            .iter()
            .map(|x| DeezerMusicFormatCiper::new("BF_CBC_STRIPE".to_string(), **x))
            .collect::<Vec<DeezerMusicFormatCiper>>();

        let body = json!({
            "media" : vec![
                json!({
                    "type": "FULL",
                    "formats": format,
                })
            ],
            "track_tokens": vec![music.token.clone()],
            "license_token": self.license_token,
        });

        let response = self
            .http_client
            .post(url)
            .json(&body)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        let v = &response["data"][0]["media"][0];

        Ok(serde_json::from_value(v.clone()).unwrap())
    }

    async fn download_media(
        &self,
        music: &DeezerUnofficialMusic,
        media: &DeezerUnofficialMedia,
    ) -> Result<Vec<u8>> {
        let response = self
            .http_client
            .get(media.sources[0].url.clone())
            .send()
            .await?
            .bytes()
            .await?;

        let chunks = response.chunks(2048);
        let mut decrypted_file: Vec<u8> = Vec::with_capacity(chunks.len());
        let bf_key = music.get_bf_key();
        type BfCBC = Cbc<Blowfish, NoPadding>;

        let cipher = BfCBC::new_from_slices(bf_key.as_bytes(), &[0, 1, 2, 3, 4, 5, 6, 7]).unwrap();

        for (iter, ch) in chunks.enumerate() {
            if iter % 3 > 0 || ch.len() != 2048 {
                decrypted_file.extend_from_slice(ch);
            } else {
                decrypted_file.append(&mut cipher.clone().decrypt_vec(ch).unwrap());
            }
        }

        Ok(decrypted_file)
    }

    async fn stream_media(
        &self,
        music: &DeezerUnofficialMusic,
        media: &DeezerUnofficialMedia,
        start: usize,
    ) -> (
        impl futures::stream::Stream<Item = core::result::Result<bytes::Bytes, MusicError>>,
        ByteRangeSpec,
    ) {
        let dl_start = start / (3 * 2048);
        let dl_start = dl_start * (3 * 2048);
        let response = self
            .http_client
            .get(media.sources[0].url.clone())
            .header("range", format!("bytes={}-", dl_start))
            .send()
            .await
            .unwrap();
        let size = response.content_length().unwrap() as usize;
        let response = response.bytes_stream();

        (
            DeezerStreamDecrypt::new(music.get_bf_key(), response),
            ByteRangeSpec::FromTo(dl_start as u64, (dl_start + size - 1) as u64),
        )
    }

    pub async fn download_music(
        &self,
        id: DeezerId,
        allowed_formats: &Vec<DeezerMusicFormats>,
    ) -> Result<(Vec<u8>, DeezerMusicFormats)> {
        let music = self.get_music_by_id(id).await?;
        let media = self
            .get_music_download_url(&music, &allowed_formats)
            .await?;
        let file = self.download_media(&music, &media).await?;

        Ok((file, media.format))
    }

    pub async fn stream_music(
        &self,
        id: DeezerId,
        allowed_formats: &Vec<DeezerMusicFormats>,
        start: Option<u64>,
    ) -> Result<(
        impl futures::stream::Stream<Item = core::result::Result<bytes::Bytes, MusicError>>,
        DeezerMusicFormats,
        u64,
        ByteRangeSpec,
    )> {
        let music = self.get_music_by_id(id).await?;
        let media = self
            .get_music_download_url(&music, &allowed_formats)
            .await?;
        let (stream, stream_range) = self
            .stream_media(&music, &media, start.unwrap_or(0) as usize)
            .await;
        Ok((
            stream,
            media.format,
            music.get_size(&media.format),
            stream_range,
        ))
    }
}

#[pin_project]
pub struct DeezerStreamDecrypt<Inner: futures::stream::Stream<Item = reqwest::Result<bytes::Bytes>>>
{
    bf_cipher: Cbc<Blowfish, NoPadding>,
    #[pin]
    input: futures::stream::TryChunks<stream_flatten_iters::TryFlattenIters<Inner>>,
    iter: usize,
}

impl<Inner: futures::stream::Stream<Item = reqwest::Result<bytes::Bytes>>>
    DeezerStreamDecrypt<Inner>
{
    pub fn new(bf_key: String, input: Inner) -> Self {
        let bf_cipher = Cbc::new_from_slices(bf_key.as_bytes(), &[0, 1, 2, 3, 4, 5, 6, 7]).unwrap();
        Self {
            bf_cipher,
            input: stream_flatten_iters::TryStreamExt::try_flatten_iters(input).try_chunks(2048),
            iter: 0,
        }
    }
}

impl<Inner: futures::stream::Stream<Item = reqwest::Result<bytes::Bytes>>> futures::Stream
    for DeezerStreamDecrypt<Inner>
{
    type Item = core::result::Result<bytes::Bytes, MusicError>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<Self::Item>> {
        let mut this = self.as_mut().project();
        match this.input.as_mut().poll_next(cx) {
            std::task::Poll::Ready(Some(Ok(chunk))) => {
                if *this.iter % 3 > 0 || chunk.len() != 2048 {
                    *this.iter += 1;
                    std::task::Poll::Ready(Some(Ok(bytes::Bytes::copy_from_slice(&chunk))))
                } else {
                    *this.iter += 1;
                    let cipher = this.bf_cipher.clone();
                    let decrypted = cipher.decrypt_vec(&chunk).unwrap();
                    std::task::Poll::Ready(Some(Ok(bytes::Bytes::copy_from_slice(&decrypted))))
                }
            }
            std::task::Poll::Ready(Some(Err(e))) => {
                std::task::Poll::Ready(Some(Err(MusicError::from(e.1))))
            }
            std::task::Poll::Ready(None) => std::task::Poll::Ready(None),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}
