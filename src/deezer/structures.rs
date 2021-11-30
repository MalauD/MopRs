use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchMusicsResult {
    pub data: Vec<SearchMusicsResultItem>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SearchMusicsResultItem {
    pub id: i32,
    pub title: String,
    pub rank: i32,
    pub duration: i32,
    pub artist: SearchMusicsResultItemArtist,
    pub album: SearchMusicsResultItemAlbum,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SearchMusicsResultItemArtist {
    pub id: i32,
    pub name: String,
    pub picture_big: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SearchMusicsResultItemAlbum {
    pub id: i32,
    pub title: String,
    pub cover_big: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AlbumTracksResult {
    pub data: Vec<AlbumTracksResultItem>,
    pub next: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AlbumTracksResultItem {
    pub id: i32,
    pub title: String,
    pub rank: i32,
    pub duration: i32,
    pub track_position: i32,
    pub disk_number: i32,
    pub artist: AlbumTracksResultArtist,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AlbumTracksResultArtist {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ArtistAlbumsResult {
    pub data: Vec<ArtistAlbumsResultItem>,
    pub next: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ArtistAlbumsResultItem {
    pub id: i32,
    pub title: String,
    pub cover_big: String,
}
