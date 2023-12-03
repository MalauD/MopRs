use block_modes::{block_padding::NoPadding, BlockMode, Cbc};
use blowfish::Blowfish;
use itertools::Itertools;
use log::debug;
use once_cell::sync::OnceCell;
use reqwest::Client;
use serde::{ser::SerializeMap, Deserialize, Serialize};
use serde_json::json;
use std::sync::{Mutex, RwLock};
use thiserror::Error;

use crate::models::DeezerId;

#[derive(Error, Debug)]
pub enum DeezerDownloaderError {
    #[error("Something went wrong with the request")]
    ApiBackendError(#[from] reqwest::Error),
    #[error("Invalid arl token")]
    InvalidArlToken,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeezerUnofficialMusic {
    #[serde(rename = "SNG_ID")]
    id: String,
    #[serde(rename = "MD5_ORIGIN")]
    md5: String,
    #[serde(rename = "TRACK_TOKEN")]
    token: String,
    #[serde(rename = "MEDIA_VERSION")]
    media_version: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeezerUnofficialMedia {
    format: DeezerMusicFormats,
    sources: Vec<DeezerUnofficialMediaSource>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeezerUnofficialMediaSource {
    url: String,
}

type Result<T> = std::result::Result<T, DeezerDownloaderError>;

pub struct DeezerDownloader {
    arl: String,
    sid: String,
    token: String,
    license_token: String,
    http_client: Client,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

impl Serialize for DeezerMusicFormats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("cipher", "BF_CBC_STRIPE")?;
        map.serialize_entry("format", &self.to_string())?;
        map.end()
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
        Self {
            arl,
            http_client: Client::new(),
            token: String::new(),
            license_token: String::new(),
            sid: String::new(),
        }
    }

    fn get_cookie(&self, include_sid: bool) -> String {
        if include_sid {
            format!("arl={}; sid={}", self.arl, self.sid)
        } else {
            format!("arl={}", self.arl)
        }
    }

    pub async fn authenticate(&mut self) -> Result<()> {
        let response = self
            .http_client
            .get("http://www.deezer.com/ajax/gw-light.php?api_token=null&method=deezer.getUserData&api_version=1.0&input=3")
            .header("cookie", self.get_cookie(false))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        if response["results"]["USER"]["USER_ID"].as_i64().unwrap() == 0 {
            return Err(DeezerDownloaderError::InvalidArlToken);
        }

        self.token = response["results"]["checkForm"]
            .as_str()
            .unwrap()
            .to_string();
        self.license_token = response["results"]["USER"]["OPTIONS"]["license_token"]
            .as_str()
            .unwrap()
            .to_string();

        self.sid = response["results"]["SESSION_ID"]
            .as_str()
            .unwrap()
            .to_string();

        Ok(())
    }

    async fn get_music_by_id(&self, id: DeezerId) -> Result<DeezerUnofficialMusic> {
        let url = format!(
            "http://www.deezer.com/ajax/gw-light.php?api_token={}&api_version=1.0&input=3&method=song.getData",
            self.token
        );

        let body = json!({
            "sng_id": id,
        });

        let res = self
            .http_client
            .post(url)
            .json(&body)
            .header("cookie", self.get_cookie(true))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        Ok(serde_json::from_value(res["results"].clone()).unwrap())
    }

    async fn get_music_download_url(
        &self,
        music: &DeezerUnofficialMusic,
        allowed_formats: &Vec<DeezerMusicFormats>,
    ) -> Result<DeezerUnofficialMedia> {
        let url = "https://media.deezer.com/v1/get_url";
        let format_sorted: Vec<&DeezerMusicFormats> =
            allowed_formats.iter().sorted_by(|a, b| b.cmp(a)).collect();
        let body = json!({
            "media" : vec![
                json!({
                    "type": "FULL",
                    "formats": format_sorted,
                })
            ],
            "track_tokens": vec![music.token.clone()],
            "license_token": self.license_token,
        });

        let response = self
            .http_client
            .post(url)
            .json(&body)
            .header("cookie", self.get_cookie(true))
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
            .header("cookie", self.get_cookie(true))
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

    pub async fn download_music(
        &self,
        id: DeezerId,
        allowed_formats: &Vec<DeezerMusicFormats>,
    ) -> Result<(Vec<u8>, DeezerMusicFormats)> {
        let music = self.get_music_by_id(id).await?;
        let media = self
            .get_music_download_url(&music, &allowed_formats)
            .await?;
        debug!(target: "mop-rs::deezer_downloader", "Downloading music: {:?} -  {:?}", music.id, media.format);
        let file = self.download_media(&music, &media).await?;

        Ok((file, media.format))
    }
}
