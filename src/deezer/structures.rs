use serde::{Deserialize, Deserializer, Serialize};

use super::StreamMusic;

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchMusicsResult {
    pub data: Vec<SearchMusicsResultItem>,
    pub next: Option<String>,
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
    #[serde(deserialize_with = "parse_cover")]
    pub cover_big: String,
}

fn parse_cover<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or("".to_string()))
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InitSessionResult {
    pub results: Session,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Session {
    #[serde(rename = "SESSION")]
    pub session: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UnofficialMusicResult {
    pub results: StreamMusic,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChartResult {
    pub tracks: ChartResultTracks,
    pub albums: ChartResultAlbums,
    pub artists: ChartResultArtists,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChartResultTracks {
    pub data: Vec<SearchMusicsResultItem>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChartResultAlbums {
    pub data: Vec<SearchMusicsResultItemAlbum>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChartResultArtists {
    pub data: Vec<SearchMusicsResultItemArtist>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RelatedArtists {
    pub data: Vec<SearchMusicsResultItemArtist>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArtistTopTracksResult {
    pub data: Vec<ArtistTopTracksItem>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ArtistTopTracksItem {
    pub id: i32,
    pub title: String,
    pub rank: i32,
    pub duration: i32,
    pub album: ArtistTopTracksItemAlbum,
    pub artist: ArtistTopTracksItemArtist,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ArtistTopTracksItemAlbum {
    pub id: i32,
    pub title: String,
    #[serde(deserialize_with = "parse_cover")]
    pub cover_big: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ArtistTopTracksItemArtist {
    pub id: i32,
    pub name: String,
}
