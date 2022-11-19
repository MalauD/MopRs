use block_modes::{block_padding::NoPadding, BlockMode, Cbc};
use blowfish::Blowfish;
use log::info;
use once_cell::sync::OnceCell;
use reqwest::{Client, Result};
use serde_json::json;
use tokio::sync::{Mutex, RwLock};

use crate::{deezer::SearchMusicsResult, models::DeezerId, s3::get_s3};

use super::{
    AlbumTracksResult, ArtistAlbumsResult, ArtistTopTracksResult, ChartResult, InitSessionResult,
    RelatedArtists, StreamMusic, StreamingCredentials, UnofficialMusicResult,
};

pub struct DeezerClient {
    http_client: Client,
    base_url: String,
    pub cred: StreamingCredentials,
}

static DEEZER: OnceCell<RwLock<DeezerClient>> = OnceCell::new();
static DEEZER_INITIALIZED: OnceCell<Mutex<bool>> = OnceCell::new();

pub async fn get_dz_client(arl: Option<String>) -> &'static RwLock<DeezerClient> {
    if let Some(c) = DEEZER.get() {
        return c;
    }

    let initializing_mutex = DEEZER_INITIALIZED.get_or_init(|| tokio::sync::Mutex::new(false));

    let mut initialized = initializing_mutex.lock().await;

    if !*initialized {
        if let Some(arl_value) = arl {
            let mut client = DeezerClient::new("https://api.deezer.com/".to_string(), arl_value);
            info!(target:"mop-rs::deezer","Initializing deezer client");
            let _ = client.init_session().await;
            let _ = client.init_user().await;
            if DEEZER.set(RwLock::new(client)).is_ok() {
                *initialized = true;
            }
        }
    }

    drop(initialized);
    DEEZER.get().unwrap()
}

pub async fn refresh_dz_client() {
    let mut dz = get_dz_client(None).await.write().await;
    info!(target:"mop-rs::deezer","Initializing deezer client");
    let _ = dz.init_session().await;
    let _ = dz.init_user().await;
}

impl DeezerClient {
    pub fn new(base_url: String, arl: String) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            cred: StreamingCredentials::new(arl),
        }
    }

    fn get_cookie(&self) -> String {
        if self.cred.sid.is_empty() {
            return format!("arl={}", self.cred.arl);
        }
        format!("arl={};sid={}", self.cred.arl, self.cred.sid)
    }

    pub async fn init_session(&mut self) -> Result<()> {
        let response: InitSessionResult = self
            .http_client
            .get("http://www.deezer.com/ajax/gw-light.php?method=deezer.ping&api_version=1.0&api_token")
            .header("cookie", self.get_cookie())
            .send()
            .await?
            .json()
            .await?;

        self.cred.set_sid(response.results.session);
        Ok(())
    }

    pub async fn init_user(&mut self) -> Result<()> {
        let response : serde_json::Value = self
            .http_client
            .get("http://www.deezer.com/ajax/gw-light.php?api_token=null&method=deezer.getUserData&api_version=1.0&input=3") 
            .header("cookie", self.get_cookie())
            .send()
            .await?
            .json()
            .await?;
        let res = response.get("results").unwrap();
        self.cred
            .set_token(res.get("checkForm").unwrap().as_str().unwrap().to_string());
        Ok(())
    }

    pub async fn get_music_by_id_unofficial(&self, id: DeezerId) -> Result<StreamMusic> {
        let url = format!(
            "http://www.deezer.com/ajax/gw-light.php?api_token={}&api_version=1.0&input=3&method=song.getData",
            self.cred.token
        );
        Ok(self
            .http_client
            .post(url)
            .json(&json!({ "sng_id": id }))
            .header("cookie", self.get_cookie())
            .send()
            .await?
            .json::<UnofficialMusicResult>()
            .await?
            .results)
    }

    pub async fn download_music(&self, id: DeezerId) -> Result<Vec<u8>> {
        let m = self.get_music_by_id_unofficial(id).await?;
        let response = self
            .http_client
            .get(m.get_url())
            .header("cookie", self.get_cookie())
            .send()
            .await?
            .bytes()
            .await?;

        let chunks = response.chunks(2048);
        let mut decrypted_file: Vec<u8> = Vec::with_capacity(chunks.len());
        let bf_key = m.get_bf_key();
        type BfCBC = Cbc<Blowfish, NoPadding>;

        let cipher = BfCBC::new_from_slices(bf_key.as_bytes(), &[0, 1, 2, 3, 4, 5, 6, 7]).unwrap();

        let mut iter = 0;

        for ch in chunks {
            if iter % 3 > 0 || ch.len() != 2048 {
                decrypted_file.extend_from_slice(ch);
            } else {
                decrypted_file.append(&mut cipher.clone().decrypt_vec(ch).unwrap());
            }
            // let mut decrypted_buf = decrypt(
            //     cipher,
            //     bf_key.as_bytes(),
            //     Some(&[0, 1, 2, 3, 4, 5, 6, 7]),
            //     ch,
            // )
            // .unwrap();

            iter = iter + 1;
        }
        let s3 = get_s3(None).await;
        let _ = s3
            .get_bucket()
            .put_object(format!("/{}", id), &decrypted_file)
            .await
            .unwrap();
        info!(target: "mop-rs::deezer","Downloaded music");
        Ok(decrypted_file)
    }

    pub async fn search_music(&self, search: String) -> Result<SearchMusicsResult> {
        let url = format!("{}/search?q={}", self.base_url, search);
        let response: SearchMusicsResult = self.http_client.get(&url).send().await?.json().await?;
        Ok(response)
    }

    pub async fn get_album_musics(&self, album_id: DeezerId) -> Result<AlbumTracksResult> {
        let url = format!("{}/album/{}/tracks?limit=50", self.base_url, album_id);
        let mut response = self.get_album_musics_aux(&url).await?;
        while let Some(ref next_url) = response.next {
            let mut musics = self.get_album_musics_aux(next_url).await?;
            response.data.append(&mut musics.data);
            response.next = musics.next;
        }
        Ok(response.clone())
    }

    async fn get_album_musics_aux(&self, url: &String) -> Result<AlbumTracksResult> {
        let response: AlbumTracksResult = self.http_client.get(url).send().await?.json().await?;
        Ok(response)
    }

    pub async fn get_artist_albums(&self, artist_id: &DeezerId) -> Result<ArtistAlbumsResult> {
        let url = format!("{}/artist/{}/albums?limit=50", self.base_url, artist_id);
        let mut response = self.get_artist_albums_aux(&url).await?;
        while let Some(ref next_url) = response.next {
            let mut albums = self.get_artist_albums_aux(next_url).await?;
            response.data.append(&mut albums.data);
            response.next = albums.next;
        }
        Ok(response)
    }

    async fn get_artist_albums_aux(&self, url: &String) -> Result<ArtistAlbumsResult> {
        let response: ArtistAlbumsResult = self.http_client.get(url).send().await?.json().await?;
        Ok(response)
    }

    pub async fn get_most_popular(&self) -> Result<ChartResult> {
        let url = format!("{}/chart?limit=100", self.base_url);
        let response: ChartResult = self.http_client.get(url).send().await?.json().await?;
        Ok(response)
    }

    pub async fn get_related_artists(&self, artist_id: &DeezerId) -> Result<RelatedArtists> {
        let url = format!("{}/artist/{}/related", self.base_url, artist_id);
        let response: RelatedArtists = self.http_client.get(url).send().await?.json().await?;
        Ok(response)
    }

    pub async fn get_artist_top_tracks(
        &self,
        artist_id: &DeezerId,
    ) -> Result<ArtistTopTracksResult> {
        let url = format!("{}/artist/{}/top?limit=50", self.base_url, artist_id);
        let response: ArtistTopTracksResult =
            self.http_client.get(url).send().await?.json().await?;
        Ok(response)
    }

    async fn get_playlist_musics_aux(&self, url: &String) -> Result<SearchMusicsResult> {
        let response: SearchMusicsResult = self.http_client.get(url).send().await?.json().await?;
        Ok(response)
    }

    pub async fn get_playlist_musics(&self, playlist_id: &DeezerId) -> Result<SearchMusicsResult> {
        let url = format!(
            "{}/playlist/{}/tracks?limit=100",
            self.base_url, playlist_id
        );
        let mut response = self.get_playlist_musics_aux(&url).await?;
        while let Some(ref next_url) = response.next {
            let mut musics = self.get_playlist_musics_aux(next_url).await?;
            response.data.append(&mut musics.data);
            response.next = musics.next;
        }
        Ok(response)
    }
}
