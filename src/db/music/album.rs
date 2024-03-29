use mongodb::{
    bson::doc,
    error::Result,
    options::{IndexOptions, InsertManyOptions},
    IndexModel,
};
use tokio_stream::StreamExt;

use crate::{
    db::MongoClient,
    models::{Album, DeezerId},
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

    pub async fn append_to_album(&self, music_id: &DeezerId, album_id: &DeezerId) -> Result<()> {
        let coll = self._database.collection::<Album>("Album");
        coll.update_one(
            doc! {"_id": album_id },
            doc! {"$addToSet": {"musics": music_id}},
            None,
        )
        .await?;
        Ok(())
    }

    pub async fn set_album_musics(
        &self,
        music_ids: Vec<DeezerId>,
        album_id: &DeezerId,
    ) -> Result<()> {
        let coll = self._database.collection::<Album>("Album");
        coll.update_one(
            doc! {"_id": album_id },
            doc! {"$set": {"musics": music_ids}},
            None,
        )
        .await?;
        Ok(())
    }

    pub async fn get_album(&self, album_id: &DeezerId) -> Result<Option<Album>> {
        let coll = self._database.collection::<Album>("Album");
        coll.find_one(doc! {"_id": album_id}, None).await
    }

    pub async fn get_albums(&self, album_ids: &Vec<DeezerId>) -> Result<Option<Vec<Album>>> {
        let coll = self._database.collection::<Album>("Album");
        let mut cursor = coll.find(doc! {"_id": {"$in": album_ids}}, None).await?;
        let mut result = Vec::<Album>::with_capacity(30);
        while let Some(value) = cursor.next().await {
            if let Ok(res) = value {
                result.push(res);
            }
        }
        Ok(Some(result))
    }

    pub async fn get_album_of_music(&self, music_id: &DeezerId) -> Result<Option<Album>> {
        let coll = self._database.collection::<Album>("Album");
        coll.find_one(doc! {"musics": music_id}, None).await
    }
}
