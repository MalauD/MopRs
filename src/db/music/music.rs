use std::collections::HashMap;

use futures::StreamExt;
use mongodb::{
    bson::doc,
    error::Result,
    options::{FindOptions, IndexOptions, InsertManyOptions},
    IndexModel,
};

use crate::{
    db::{MongoClient, PaginationOptions},
    models::{Music, DeezerId},
};

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

    pub async fn get_musics(&self, music_ids: &Vec<DeezerId>) -> Result<Option<Vec<Music>>> {
        let coll = self._database.collection::<Music>("Music");
        let mut cursor = coll.find(doc! {"_id": {"$in": music_ids}}, None).await?;
        let mut result_hash = HashMap::with_capacity(music_ids.len());
        while let Some(value) = cursor.next().await {
            if let Ok(res) = value {
                result_hash.entry(res.id).or_insert(res);
            }
        }
        let mut final_arranged: Vec<Music> = Vec::with_capacity(music_ids.len());
        for e in music_ids {
            final_arranged.push(result_hash[e].clone());
        }
        return Ok(Some(final_arranged));
    }

    pub async fn modify_like_count(&self, music_id: &DeezerId, inc: DeezerId) -> Result<()> {
        let coll = self._database.collection::<Music>("Music");
        coll.update_one(doc! {"_id": music_id}, doc! {"$inc": {"likes": inc}}, None)
            .await?;
        Ok(())
    }

    pub async fn get_musics_count(&self) -> Result<u64> {
        let coll = self._database.collection::<Music>("Music");
        Ok(coll.estimated_document_count(None).await?)
    }
}
