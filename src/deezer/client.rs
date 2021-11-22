use reqwest::Client;

use crate::deezer::SearchMusicsResult;

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
}
