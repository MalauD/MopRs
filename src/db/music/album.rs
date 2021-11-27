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

    pub async fn search_album(
        &self,
        search: String,
        pagination: &PaginationOptions,
    ) -> Result<Option<Vec<Album>>> {
        let coll = self._database.collection::<Album>("Album");
        let find_option = FindOptions::builder()
            .limit(pagination.get_max_results() as i64)
            .skip(Some(
                (pagination.get_page() as u32 * pagination.get_max_results()) as u64,
            ))
            .build();
        let mut cursor = coll
            .find(doc! { "$text": { "$search": search } }, find_option)
            .await?;
        let mut result =
            Vec::<Album>::with_capacity(pagination.get_max_results().max(20).try_into().unwrap());
        while let Some(value) = cursor.next().await {
            if let Ok(res) = value {
                result.push(res);
            }
        }
        Ok(Some(result))
    }
}
