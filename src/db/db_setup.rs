use mongodb::{options::ClientOptions, Client, Database};
use once_cell::sync::OnceCell;
use serde::Deserialize;
use tokio::sync::Mutex;

static MONGO: OnceCell<MongoClient> = OnceCell::new();
static MONGO_INITIALIZED: OnceCell<Mutex<bool>> = OnceCell::new();

pub struct MongoClient {
    pub(in crate::db) _database: Database,
}

pub async fn get_mongo() -> &'static MongoClient {
    if let Some(c) = MONGO.get() {
        return c;
    }

    let initializing_mutex = MONGO_INITIALIZED.get_or_init(|| tokio::sync::Mutex::new(false));

    let mut initialized = initializing_mutex.lock().await;

    if !*initialized {
        if let Ok(client_options) =
            ClientOptions::parse("mongodb://localhost:27017/?appName=MopRs").await
        {
            if let Ok(client) = Client::with_options(client_options) {
                if MONGO
                    .set(MongoClient {
                        _database: client.database("MopRs"),
                    })
                    .is_ok()
                {
                    *initialized = true;
                }
            }
        }
    }
    if let Some(c) = MONGO.get() {
        let _ = c.create_music_text_indexes().await;
        let _ = c.create_album_text_indexes().await;
        let _ = c.create_artist_text_indexes().await;
    }
    drop(initialized);
    MONGO.get().unwrap()
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
    pub fn get_max_results(&self) -> u32 {
        self.max_results
    }
}
