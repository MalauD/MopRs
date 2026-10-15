use log::info;
use mongodb::{options::ClientOptions, Client, Database};
use serde::Deserialize;

pub struct MongoClient {
    pub(in crate::db) _database: Database,
}

impl MongoClient {
    pub async fn connect(mongo_url: &str) -> mongodb::error::Result<Self> {
        info!(target: "mop-rs::mongodb", "Connecting to database");
        let client_options = ClientOptions::parse(mongo_url).await?;
        let client = Client::with_options(client_options)?;
        Ok(Self {
            _database: client.database("MopRs"),
        })
    }

    pub async fn initialize_indexes(&self) {
        info!(target: "mop-rs::mongodb", "Creating indexes");
        let _ = self.create_music_text_indexes().await;
        info!(target: "mop-rs::mongodb", "Creating indexes 1/4");
        let _ = self.create_album_text_indexes().await;
        info!(target: "mop-rs::mongodb", "Creating indexes 2/4");
        let _ = self.create_artist_text_indexes().await;
        info!(target: "mop-rs::mongodb", "Creating indexes 3/4");
        let _ = self.create_playlist_text_indexes().await;
        info!(target: "mop-rs::mongodb", "Creating indexes 4/4 - Done.");
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationOptions {
    page: usize,
    max_results: u32,
}

impl PaginationOptions {
    pub fn get_page(&self) -> usize {
        self.page
    }
    pub fn get_max_results(&self) -> usize {
        self.max_results as usize
    }

    pub fn trim_vec<T: Copy>(&self, input: &[T]) -> Vec<T> {
        let rng = self.get_max_results() * self.get_page()
            ..self.get_max_results() * (self.get_page() + 1);
        let mut vec: Vec<T> = Vec::with_capacity(rng.len());
        for (i, e) in input.iter().enumerate() {
            if rng.contains(&i) {
                vec.push(*e);
            }
        }
        vec
    }
}
