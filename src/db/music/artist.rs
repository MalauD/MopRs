use std::collections::HashMap;

use chrono::{Duration, Utc};
use mongodb::{
    bson::{doc, Bson},
    error::Result,
    options::{FindOptions, IndexOptions, InsertManyOptions},
    IndexModel,
};
use tokio_stream::StreamExt;

use crate::{
    db::{MongoClient, PaginationOptions},
    models::{Artist, DeezerId, Playlist},
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
            .await?;
        Ok(())
    }

    pub async fn bulk_insert_artists(&self, musics: &[Artist]) -> Result<()> {
        let coll = self._database.collection::<Artist>("Artist");

        let _ = coll
            .insert_many(
                musics,
                Some(InsertManyOptions::builder().ordered(false).build()),
            )
            .await;
        Ok(())
    }

    pub async fn bulk_append_to_artists(&self, relations: &[(DeezerId, DeezerId)]) -> Result<()> {
        if relations.is_empty() {
            return Ok(());
        }

        let mut grouped_relations: HashMap<DeezerId, Vec<DeezerId>> = HashMap::new();
        for (album_id, artist_id) in relations {
            grouped_relations
                .entry(*artist_id)
                .or_default()
                .push(*album_id);
        }

        let updates: Vec<Bson> = grouped_relations
            .into_iter()
            .map(|(artist_id, album_ids)| {
                Bson::Document(doc! {
                    "q": {"_id": artist_id},
                    "u": {"$addToSet": {"albums": {"$each": album_ids}}},
                    "multi": false,
                    "upsert": false,
                })
            })
            .collect();

        self._database
            .run_command(
                doc! {"update": "Artist", "updates": updates, "ordered": false},
                None,
            )
            .await?;
        Ok(())
    }

    pub async fn search_playlist(
        &self,
        search: String,
        pagination: &PaginationOptions,
    ) -> Result<Option<Vec<Playlist>>> {
        let coll = self._database.collection::<Playlist>("Playlist");
        let find_option = FindOptions::builder()
            .limit(pagination.get_max_results() as i64)
            .skip(Some(
                (pagination.get_page() * pagination.get_max_results()) as u64,
            ))
            .build();
        let mut cursor = coll
            .find(doc! { "$text": { "$search": search } }, find_option)
            .await?;
        let mut result = Vec::<Playlist>::with_capacity(pagination.get_max_results().max(20));
        while let Some(value) = cursor.next().await {
            if let Ok(res) = value {
                result.push(res);
            }
        }
        Ok(Some(result))
    }

    pub async fn get_artist(&self, artist_id: &DeezerId) -> Result<Option<Artist>> {
        let coll = self._database.collection::<Artist>("Artist");
        coll.find_one(doc! {"_id": artist_id}, None).await
    }

    pub async fn get_artists(&self, artist_ids: &[DeezerId]) -> Result<Option<Vec<Artist>>> {
        let coll = self._database.collection::<Artist>("Artist");
        let cursor = coll.find(doc! {"_id": {"$in": artist_ids}}, None).await?;
        let docs: Vec<Result<Artist>> = cursor.collect().await;
        let mut result_hash = HashMap::with_capacity(artist_ids.len());
        for res in docs.into_iter().flatten() {
            result_hash.entry(res.id).or_insert(res);
        }
        let mut final_arranged: Vec<Artist> = Vec::with_capacity(artist_ids.len());
        for e in artist_ids {
            if let Some(artist) = result_hash.get(e) {
                final_arranged.push(artist.clone());
            }
        }
        Ok(Some(final_arranged))
    }

    pub async fn get_artists_by_name(
        &self,
        artist_names: &[String],
    ) -> Result<Option<Vec<Artist>>> {
        let coll = self._database.collection::<Artist>("Artist");
        let mut cursor = coll
            .find(doc! {"name": {"$in": artist_names}}, None)
            .await?;
        let mut result_hash = HashMap::with_capacity(artist_names.len());
        while let Some(value) = cursor.next().await {
            if let Ok(res) = value {
                result_hash.entry(res.name.clone()).or_insert(res);
            }
        }
        let mut final_arranged: Vec<Artist> = Vec::with_capacity(artist_names.len());
        for e in artist_names {
            if result_hash.contains_key(e) {
                final_arranged.push(result_hash[e].clone());
            }
        }
        Ok(Some(final_arranged))
    }

    pub async fn append_multiple_to_an_artist(
        &self,
        album_ids: Vec<DeezerId>,
        artist_id: &DeezerId,
    ) -> Result<()> {
        let coll = self._database.collection::<Artist>("Artist");
        coll.update_one(
            doc! {"_id": artist_id },
            doc! {"$addToSet": {"albums": {"$each": album_ids}}},
            None,
        )
        .await?;
        Ok(())
    }

    pub async fn set_related_artists(
        &self,
        artist_id: &DeezerId,
        related_artists: Vec<DeezerId>,
    ) -> Result<()> {
        let coll = self._database.collection::<Artist>("Artist");
        coll.update_one(
            doc! {"_id": artist_id },
            doc! {"$set": {"related_artists": related_artists, "last_update": Utc::now()}},
            None,
        )
        .await?;
        Ok(())
    }

    pub async fn set_top_tracks(
        &self,
        artist_id: &DeezerId,
        top_tracks: &[DeezerId],
    ) -> Result<()> {
        let coll = self._database.collection::<Artist>("Artist");
        coll.update_one(
            doc! {"_id": artist_id },
            doc! {"$set": {"top_tracks": top_tracks, "last_update": Utc::now()}},
            None,
        )
        .await?;
        Ok(())
    }

    pub async fn get_outdated_artist(&self, duration: Duration) -> Result<Vec<Artist>> {
        let coll = self._database.collection::<Artist>("Artist");
        let mut cursor = coll
            .find(doc! {"last_update": {"$lt": Utc::now() - duration}}, None)
            .await?;
        let mut result = Vec::<Artist>::new();
        while let Some(value) = cursor.next().await {
            if let Ok(res) = value {
                result.push(res);
            }
        }
        Ok(result)
    }
}
