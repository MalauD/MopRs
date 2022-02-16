use std::time::{SystemTime, UNIX_EPOCH};

use bson::oid::ObjectId;
use bson::serde_helpers::serialize_object_id_as_hex_string;
use chrono::Utc;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

use crate::deezer::{
    AlbumTracksResultItem, ArtistAlbumsResultItem, ChartResult, SearchMusicsResultItem,
};

#[derive(Deserialize, Serialize, Clone)]
pub struct Music {
    #[serde(rename = "_id")]
    pub id: i32,
    pub title: String,
    pub artist_name: String,
    pub published_date: DateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_number: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disc_number: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    pub views: i32,
    pub likes: i32,
    pub rank: i32,
    pub last_view: DateTime,
}

impl Music {
    pub fn get_rank(&self) -> &i32 {
        &self.rank
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Album {
    #[serde(rename = "_id")]
    pub id: i32,
    name: String,
    pub cover: String,
    is_complete: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub musics: Option<Vec<i32>>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Chart {
    #[serde(rename = "_id")]
    pub id: i32,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    published_date: chrono::DateTime<Utc>,
    pub musics: Vec<i32>,
    pub albums: Vec<i32>,
    pub artists: Vec<i32>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PopulatedAlbum {
    #[serde(rename = "_id")]
    pub id: i32,
    name: String,
    pub cover: String,
    is_complete: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub musics: Option<Vec<Music>>,
}

impl From<ChartResult> for Chart {
    fn from(ch: ChartResult) -> Self {
        Chart {
            id: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() as i32,
            published_date: Utc::now(),
            musics: ch.tracks.data.iter().map(|x| x.id).collect(),
            albums: ch.albums.data.iter().map(|x| x.id).collect(),
            artists: ch.artists.data.iter().map(|x| x.id).collect(),
        }
    }
}

impl From<Album> for PopulatedAlbum {
    fn from(al: Album) -> Self {
        PopulatedAlbum {
            id: al.id,
            name: al.name,
            cover: al.cover,
            is_complete: al.is_complete,
            musics: None,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Artist {
    #[serde(rename = "_id")]
    pub id: i32,
    name: String,
    picture: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub albums: Option<Vec<i32>>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PopulatedArtist {
    #[serde(rename = "_id")]
    pub id: i32,
    name: String,
    pub picture: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub albums: Option<Vec<Album>>,
}

impl From<Artist> for PopulatedArtist {
    fn from(al: Artist) -> Self {
        PopulatedArtist {
            id: al.id,
            name: al.name,
            picture: al.picture,
            albums: None,
        }
    }
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
            image_url: Some(music_search_result.album.cover_big),
            views: 0,
            likes: 0,
            rank: music_search_result.rank,
            last_view: DateTime::now(),
        }
    }
}

impl From<AlbumTracksResultItem> for Music {
    fn from(album_track: AlbumTracksResultItem) -> Self {
        Music {
            id: album_track.id,
            title: album_track.title,
            artist_name: album_track.artist.name,
            published_date: DateTime::now(),
            track_number: Some(album_track.track_position),
            disc_number: Some(album_track.disk_number),
            file_path: None,
            image_url: None,
            views: 0,
            likes: 0,
            rank: album_track.rank,
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

impl From<ArtistAlbumsResultItem> for Album {
    fn from(album: ArtistAlbumsResultItem) -> Self {
        Album {
            id: album.id,
            name: album.title,
            cover: album.cover_big,
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

#[derive(Deserialize, Serialize, Clone)]
pub struct Playlist {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    name: String,
    creator: ObjectId,
    public: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub musics: Option<Vec<i32>>,
}

impl Playlist {
    pub fn new(
        id: ObjectId,
        name: String,
        creator: ObjectId,
        public: bool,
        musics: Option<Vec<i32>>,
    ) -> Self {
        Self {
            id,
            name,
            creator,
            public,
            musics,
        }
    }

    pub fn is_authorized_read(&self, user: &ObjectId) -> bool {
        self.creator == *user || self.public
    }

    pub fn is_authorized_write(&self, user: &ObjectId) -> bool {
        self.creator == *user
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PopulatedPlaylist {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    name: String,
    creator: ObjectId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub musics: Option<Vec<Music>>,
}

impl From<Playlist> for PopulatedPlaylist {
    fn from(p: Playlist) -> Self {
        PopulatedPlaylist {
            id: p.id,
            name: p.name,
            creator: p.creator,
            musics: None,
        }
    }
}
