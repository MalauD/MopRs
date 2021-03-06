use mongodb::{
    bson::doc,
    error::Result,
    options::{FindOptions, IndexOptions, InsertManyOptions},
    IndexModel,
};
use tokio_stream::StreamExt;

use crate::{
    db::{MongoClient, PaginationOptions},
    models::Album,
};

impl MongoClient {
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

    pub async fn bulk_insert_albums(&self, albums: Vec<Album>) -> Result<()> {
        let coll = self._database.collection::<Album>("Album");

        let _ = coll
            .insert_many(
                albums,
                Some(InsertManyOptions::builder().ordered(false).build()),
            )
            .await;
        Ok(())
    }

    pub async fn append_to_album(&self, music_id: &i32, album_id: &i32) -> Result<()> {
        let coll = self._database.collection::<Album>("Album");
        coll.update_one(
            doc! {"_id": album_id },
            doc! {"$addToSet": {"musics": music_id}},
            None,
        )
        .await?;
        Ok(())
    }

    pub async fn set_album_musics(&self, music_ids: Vec<i32>, album_id: &i32) -> Result<()> {
        let coll = self._database.collection::<Album>("Album");
        coll.update_one(
            doc! {"_id": album_id },
            doc! {"$set": {"musics": music_ids}},
            None,
        )
        .await?;
        Ok(())
    }

    pub async fn search_album(
        &self,
        search: String,
        pagination: &PaginationOptions,
    ) -> Result<Option<Vec<Album>>> {
        let coll = self._database.collection::<Album>("Album");
        let find_option = FindOptions::builder()
            .limit(pagination.get_max_results() as i64)
            .skip(Some(
                (pagination.get_page() * pagination.get_max_results()) as u64,
            ))
            .build();
        let mut cursor = coll
            .find(doc! { "$text": { "$search": search } }, find_option)
            .await?;
        let mut result = Vec::<Album>::with_capacity(pagination.get_max_results().max(20));
        while let Some(value) = cursor.next().await {
            if let Ok(res) = value {
                result.push(res);
            }
        }
        Ok(Some(result))
    }

    pub async fn get_album(&self, album_id: &i32) -> Result<Option<Album>> {
        let coll = self._database.collection::<Album>("Album");
        Ok(coll.find_one(doc! {"_id": album_id}, None).await?)
    }

    pub async fn get_albums(&self, album_ids: &Vec<i32>) -> Result<Option<Vec<Album>>> {
        let coll = self._database.collection::<Album>("Album");
        let mut cursor = coll.find(doc! {"_id": {"$in": album_ids}}, None).await?;
        let mut result = Vec::<Album>::with_capacity(30);
        while let Some(value) = cursor.next().await {
            if let Ok(res) = value {
                result.push(res);
            }
        }
        return Ok(Some(result));
    }
}
