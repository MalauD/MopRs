use itertools::Itertools;
use mongodb::{
    bson::{doc, Bson},
    error::Result,
    options::{ClientOptions, InsertManyOptions},
    Client, Database,
};
use once_cell::sync::OnceCell;
use tokio::sync::Mutex;

use crate::models::{Album, Artist, Music};

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
    drop(initialized);
    MONGO.get().unwrap()
}

impl MongoClient {
    pub async fn bulk_insert_musics(&self, musics: Vec<Music>) -> Result<()> {
        let coll = self._database.collection::<Music>("Music");

        let _ = coll
            .insert_many(
                musics,
                Some(InsertManyOptions::builder().ordered(false).build()),
            )
            .await;
        Ok(())
    }

    pub async fn bulk_insert_albums(&self, musics: Vec<Album>) -> Result<()> {
        let coll = self._database.collection::<Album>("Album");

        let _ = coll
            .insert_many(
                musics,
                Some(InsertManyOptions::builder().ordered(false).build()),
            )
            .await;
        Ok(())
    }

    pub async fn bulk_insert_artists(&self, musics: Vec<Artist>) -> Result<()> {
        let coll = self._database.collection::<Artist>("Artist");

        let _ = coll
            .insert_many(
                musics,
                Some(InsertManyOptions::builder().ordered(false).build()),
            )
            .await;
        Ok(())
    }
}
