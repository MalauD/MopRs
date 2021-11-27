use mongodb::{
    bson::doc,
    error::Result,
    options::{FindOptions, IndexOptions, InsertManyOptions},
    IndexModel,
};
use tokio_stream::StreamExt;

use crate::{
    db::{MongoClient, PaginationOptions},
    models::Artist,
};

impl MongoClient {
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

    pub async fn search_artist(
        &self,
        search: String,
        pagination: &PaginationOptions,
    ) -> Result<Option<Vec<Artist>>> {
        let coll = self._database.collection::<Artist>("Artist");
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
            Vec::<Artist>::with_capacity(pagination.get_max_results().max(20).try_into().unwrap());
        while let Some(value) = cursor.next().await {
            if let Ok(res) = value {
                result.push(res);
            }
        }
        Ok(Some(result))
    }
}
