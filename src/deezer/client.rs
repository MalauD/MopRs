use once_cell::sync::OnceCell;
use reqwest::{Client, Result};
use std::sync::Mutex;

use crate::{deezer::SearchMusicsResult, models::DeezerId};

use super::{
    AlbumTracksResult, ArtistAlbumsResult, ArtistTopTracksResult, ChartResult, RelatedArtists,
};

pub struct DeezerClient {
    http_client: Client,
    base_url: String,
}

static DEEZER: OnceCell<DeezerClient> = OnceCell::new();
static DEEZER_INITIALIZED: OnceCell<Mutex<bool>> = OnceCell::new();

pub fn get_dz_client() -> &'static DeezerClient {
    if let Some(c) = DEEZER.get() {
        return c;
    }

    let initializing_mutex = DEEZER_INITIALIZED.get_or_init(|| Mutex::new(false));

    let mut initialized = initializing_mutex.lock().unwrap();

    if !*initialized {
        let client = DeezerClient::new("https://api.deezer.com/".to_string());
        if DEEZER.set(client).is_ok() {
            *initialized = true;
        }
    }

    drop(initialized);
    DEEZER.get().unwrap()
}

impl DeezerClient {
    pub fn new(base_url: String) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
        }
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

    pub async fn get_cover(&self, url: &String) -> Result<Vec<u8>> {
        Ok(self
            .http_client
            .get(url)
            .send()
            .await?
            .bytes()
            .await?
            .to_vec())
    }
}
