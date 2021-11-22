use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use crate::deezer::SearchMusicsResultItem;

#[derive(Deserialize, Serialize, Clone)]
pub struct Music {
    #[serde(rename = "_id")]
    pub id: i32,
    title: String,
    artist_name: String,
    published_date: DateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    track_number: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disc_number: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_path: Option<String>,
    views: i32,
    likes: i32,
    rank: i32,
    last_view: DateTime,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Album {
    #[serde(rename = "_id")]
    pub id: i32,
    name: String,
    cover: String,
    is_complete: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    musics: Option<Vec<Music>>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Artist {
    #[serde(rename = "_id")]
    pub id: i32,
    name: String,
    picture: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    albums: Option<Vec<Album>>,
}

impl From<SearchMusicsResultItem> for Music {
    fn from(music_search_result: SearchMusicsResultItem) -> Self {
        Music {
            id: music_search_result.id,
            title: music_search_result.title,
            artist_name: music_search_result.artist.name,
            published_date: DateTime::now(),
            track_number: None,
            disc_number: None,
            file_path: None,
            views: 0,
            likes: 0,
            rank: music_search_result.rank,
            last_view: DateTime::now(),
        }
    }
}

impl From<SearchMusicsResultItem> for Album {
    fn from(music: SearchMusicsResultItem) -> Self {
        Album {
            id: music.album.id,
            name: music.album.title,
            cover: music.album.cover_big,
            is_complete: false,
            musics: None,
        }
    }
}

impl From<SearchMusicsResultItem> for Artist {
    fn from(music: SearchMusicsResultItem) -> Self {
        Artist {
            id: music.artist.id,
            name: music.artist.name,
            picture: music.artist.picture_big,
            albums: None,
        }
    }
}
