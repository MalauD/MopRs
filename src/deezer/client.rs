use std::{fs, io::Write, path::Path};

use block_modes::{
    block_padding::{NoPadding, ZeroPadding},
    BlockMode, Cbc,
};
use blowfish::Blowfish;
use log::info;
use reqwest::Client;
use serde_json::json;

use crate::deezer::SearchMusicsResult;

use super::{
    AlbumTracksResult, ArtistAlbumsResult, InitSessionResult, StreamMusic, StreamingCredentials,
    UnofficialMusicResult,
};

pub struct DeezerClient {
    http_client: Client,
    base_url: String,
    pub cred: StreamingCredentials,
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

    pub async fn init_session(&mut self) -> Result<(), String> {
        let response: InitSessionResult = self
            .http_client
            .get("http://www.deezer.com/ajax/gw-light.php?method=deezer.ping&api_version=1.0&api_token")
            .header("cookie", self.get_cookie())
            .send()
            .await
            .expect("Failed to init session")
            .json()
            .await
            .expect("Failed to parse session");

        self.cred.set_sid(response.results.SESSION);
        Ok(())
    }

    pub async fn init_user(&mut self) -> Result<(), String> {
        let response : serde_json::Value = self
            .http_client
            .get("http://www.deezer.com/ajax/gw-light.php?api_token=null&method=deezer.getUserData&api_version=1.0&input=3") 
            .header("cookie", self.get_cookie())
            .send()
            .await
            .expect("Failed to init session")
            .json()
            .await
            .expect("Failed to parse session");
        let res = response.get("results").unwrap();
        self.cred
            .set_token(res.get("checkForm").unwrap().as_str().unwrap().to_string());
        Ok(())
    }

    pub async fn get_music_by_id_unofficial(&self, id: i32) -> Result<StreamMusic, String> {
        let url = format!(
            "http://www.deezer.com/ajax/gw-light.php?api_token={}&api_version=1.0&input=3&method=song.getData",
            self.cred.token
        );
        let response: UnofficialMusicResult = self
            .http_client
            .post(url)
            .json(&json!({ "sng_id": id }))
            .header("cookie", self.get_cookie())
            .send()
            .await
            .expect("Failed to get music")
            .json()
            .await
            .expect("Failed to parse music");

        Ok(response.results)
    }

    pub async fn download_music(&self, id: i32, dir: &Path) -> Result<String, String> {
        let m = self.get_music_by_id_unofficial(id).await.unwrap();
        let response = self
            .http_client
            .get(m.get_url())
            .header("cookie", self.get_cookie())
            .send()
            .await
            .expect("Failed to get music")
            .bytes()
            .await
            .expect("Failed to get music");
        let path = dir.join(format!("{}.mp3", id.to_string()));

        let mut f = fs::File::create(&path).unwrap();

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
        let _ = f.write_all(&decrypted_file);
        info!(target: "mop-rs::deezer","Downloaded music");
        Ok(path.into_os_string().into_string().unwrap())
    }

    pub async fn search_music(&self, search: String) -> Result<SearchMusicsResult, String> {
        let url = format!("{}/search?q={}", self.base_url, search);
        let response: SearchMusicsResult = self
            .http_client
            .get(&url)
            .send()
            .await
            .expect("Failed to get musics from Deezer Api")
            .json()
            .await
            .expect("Failed to parse music from Deezer Api");
        Ok(response)
    }

    pub async fn get_album_musics(&self, album_id: i32) -> Result<AlbumTracksResult, String> {
        let url = format!("{}/album/{}/tracks", self.base_url, album_id);
        let mut response = self.get_album_musics_aux(&url).await;
        if let Some(ref next_url) = response.next {
            let mut musics = self.get_album_musics_aux(next_url).await;
            response.data.append(&mut musics.data);
        }
        Ok(response.clone())
    }

    async fn get_album_musics_aux(&self, url: &String) -> AlbumTracksResult {
        let response: AlbumTracksResult = self
            .http_client
            .get(url)
            .send()
            .await
            .expect("Failed to get musics from Deezer Api")
            .json()
            .await
            .expect("Failed to parse music from Deezer Api");
        response
    }

    pub async fn get_artist_albums(&self, artist_id: &i32) -> Result<ArtistAlbumsResult, String> {
        let url = format!("{}/artist/{}/albums", self.base_url, artist_id);
        let mut response = self.get_artist_albums_aux(&url).await;
        if let Some(ref next_url) = response.next {
            let mut albums = self.get_artist_albums_aux(next_url).await;
            response.data.append(&mut albums.data);
        }
        Ok(response)
    }

    async fn get_artist_albums_aux(&self, url: &String) -> ArtistAlbumsResult {
        let response: ArtistAlbumsResult = self
            .http_client
            .get(url)
            .send()
            .await
            .expect("Failed to get albums from Deezer Api")
            .json()
            .await
            .expect("Failed to parse albums from Deezer Api");
        response
    }
}
