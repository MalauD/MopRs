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
