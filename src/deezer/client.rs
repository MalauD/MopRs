use reqwest::Client;

use crate::deezer::SearchMusicsResult;

use super::AlbumTracksResult;

pub struct DeezerClient {
    http_client: Client,
    base_url: String,
}

impl DeezerClient {
    pub fn new(base_url: String) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
        }
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
        let response: AlbumTracksResult = self
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
}
