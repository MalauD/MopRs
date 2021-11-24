use futures::StreamExt;
use itertools::Itertools;
use mongodb::{
    bson::{doc, Bson},
    error::Result,
    options::{ClientOptions, FindOptions, IndexOptions, InsertManyOptions},
    Client, Database, IndexModel,
};
use once_cell::sync::OnceCell;
use serde::Deserialize;
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

impl MongoClient {
    pub async fn create_music_text_indexes(&self) -> Result<()> {
        let index_opt = IndexOptions::builder()
            .weights(doc! {
                "title": 10,
                "artist_name": 5
            })
            .build();
        let index = IndexModel::builder()
            .keys(doc! {
                "title": "text",
                "content": "text",
            })
            .options(index_opt)
            .build();
        self._database
            .collection::<Music>("Music")
            .create_index(index, None)
            .await
            .unwrap();
        Ok(())
    }

    pub async fn create_album_text_indexes(&self) -> Result<()> {
        let index_opt = IndexOptions::builder()
            .weights(doc! {
                "name": 10,
            })
            .build();
        let index = IndexModel::builder()
            .keys(doc! {
                "name": "text"
            })
            .options(index_opt)
            .build();
        self._database
            .collection::<Album>("Album")
            .create_index(index, None)
            .await
            .unwrap();
        Ok(())
    }

    pub async fn create_artist_text_indexes(&self) -> Result<()> {
        let index_opt = IndexOptions::builder()
            .weights(doc! {
                "name": 10,
            })
            .build();
        let index = IndexModel::builder()
            .keys(doc! {
                "name": "text"
            })
            .options(index_opt)
            .build();
        self._database
            .collection::<Artist>("Artist")
            .create_index(index, None)
            .await
            .unwrap();
        Ok(())
    }

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

    pub async fn append_to_album(&self, music_id: i32, album_id: i32) -> Result<()> {
        let coll = self._database.collection::<Album>("Album");
        coll.update_one(
            doc! {"_id": album_id },
            doc! {"$addToSet": {"musics": music_id}},
            None,
        )
        .await?;
        Ok(())
    }

    pub async fn append_to_artist(&self, album_id: i32, artist_id: i32) -> Result<()> {
        let coll = self._database.collection::<Artist>("Artist");
        coll.update_one(
            doc! {"_id": artist_id },
            doc! {"$addToSet": {"albums": album_id}},
            None,
        )
        .await?;
        Ok(())
    }

    pub async fn search_music(
        &self,
        search: String,
        pagination: &PaginationOptions,
    ) -> Result<Option<Vec<Music>>> {
        let coll = self._database.collection::<Music>("Music");
        let find_option = FindOptions::builder()
            .batch_size(pagination.max_results.max(20))
            //    .sort(doc!{ "$meta": "textScore" })
            .build();
        let mut cursor = coll
            .find(doc! { "$text": { "$search": search } }, find_option)
            .await?;
        let mut result =
            Vec::<Music>::with_capacity(pagination.max_results.max(50).try_into().unwrap());
        while let Some(value) = cursor.next().await {
            if let Ok(res) = value {
                result.push(res);
            }
        }
        result.sort_by(|x, y| x.get_rank().cmp(y.get_rank()));
        Ok(Some(result))
    }
}
