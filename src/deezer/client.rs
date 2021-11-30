use reqwest::Client;

use crate::deezer::SearchMusicsResult;

use super::{AlbumTracksResult, ArtistAlbumsResult};

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
