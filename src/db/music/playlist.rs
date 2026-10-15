use bson::{doc, oid::ObjectId, Bson};
use futures::StreamExt;
use mongodb::{
    error::{Error, Result},
    options::{FindOptions, IndexOptions},
    IndexModel,
};

use crate::{
    db::{MongoClient, PaginationOptions},
    models::{DeezerId, Playlist, PopulatedPlaylist, User},
};

impl MongoClient {
    pub async fn create_playlist_text_indexes(&self) -> Result<()> {
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
            .collection::<Playlist>("Playlist")
            .create_index(index, None)
            .await?;
        Ok(())
    }

    pub async fn get_playlist(&self, playlist_id: &ObjectId) -> Result<Option<Playlist>> {
        let coll = self._database.collection::<Playlist>("Playlist");
        coll.find_one(doc! {"_id": playlist_id}, None).await
    }

    pub async fn get_populated_playlist(
        &self,
        playlist_id: &ObjectId,
    ) -> Result<Option<PopulatedPlaylist>> {
        let coll = self._database.collection::<Playlist>("Playlist");
        let pipeline = vec![
            doc! {"$match": {"_id": playlist_id}},
            doc! {
                "$lookup": {
                    "from": "Music",
                    "localField": "musics",
                    "foreignField": "_id",
                    "as": "populated_musics"
                }
            },
            doc! {
                "$lookup": {
                    "from": "User",
                    "localField": "creator",
                    "foreignField": "_id",
                    "as": "creator_doc"
                }
            },
            doc! {"$unwind": "$creator_doc"},
            doc! {
                "$set": {
                    "creator": "$creator_doc",
                    "musics": {
                        "$cond": [
                            {"$eq": ["$musics", Bson::Null]},
                            Bson::Null,
                            "$populated_musics"
                        ]
                    }
                }
            },
            doc! {"$project": {"creator_doc": 0, "populated_musics": 0}},
        ];

        let mut cursor = coll.aggregate(pipeline, None).await?;
        match cursor.next().await.transpose()? {
            Some(document) => Ok(Some(bson::from_document(document)?)),
            None => Ok(None),
        }
    }

    pub async fn get_user_playlists(
        &self,
        user: &ObjectId,
        pagination: &PaginationOptions,
    ) -> Result<Vec<Playlist>> {
        let coll = self._database.collection::<Playlist>("Playlist");

        let find_options = FindOptions::builder()
            .limit(pagination.get_max_results() as i64)
            .skip(Some(
                (pagination.get_page() * pagination.get_max_results()) as u64,
            ))
            .build();

        let cursor = coll.find(doc! {"creator": user}, find_options).await?;
        let docs: Vec<Result<Playlist>> = cursor.collect().await;

        Ok(docs.into_iter().flatten().collect())
    }

    pub async fn remove_playlist(&self, playlist_id: &ObjectId) -> Result<()> {
        let coll = self._database.collection::<Playlist>("Playlist");
        let _ = coll.delete_one(doc! {"_id": playlist_id}, None).await?;
        Ok(())
    }

    pub async fn create_playlist(
        &self,
        name: String,
        musics: &[DeezerId],
        public: bool,
        creator: &User,
    ) -> Result<ObjectId> {
        let coll = self._database.collection::<Playlist>("Playlist");
        let id = ObjectId::new();
        let creator_id = creator
            .id()
            .ok_or_else(|| Error::custom("playlist creator is missing an object id"))?;
        let pl = Playlist::new(id, name, creator_id, public, Some(musics.to_vec()));
        let _ = coll.insert_one(pl, None).await?;
        Ok(id)
    }

    pub async fn add_musics_playlist(
        &self,
        playlist_id: ObjectId,
        music: &[DeezerId],
    ) -> Result<()> {
        let coll = self._database.collection::<Playlist>("Playlist");
        let _r = coll
            .update_one(
                doc! {"_id": playlist_id},
                doc! {"$push": {"musics": {"$each": music}}},
                None,
            )
            .await?;
        Ok(())
    }

    pub async fn remove_music_playlist(&self, playlist_id: ObjectId, index: usize) -> Result<()> {
        let coll = self._database.collection::<Playlist>("Playlist");
        let _r = coll
            .update_one(
                doc! {"_id": playlist_id},
                doc! {"$unset": {format!("musics.{}", index): 1}},
                None,
            )
            .await?;
        let _r = coll
            .update_one(
                doc! {"_id": playlist_id},
                doc! {"$pull": {"musics": null}},
                None,
            )
            .await?;
        Ok(())
    }

    pub async fn edit_musics_playlist(
        &self,
        playlist_id: ObjectId,
        new_musics: &[DeezerId],
    ) -> Result<()> {
        let coll = self._database.collection::<Playlist>("Playlist");
        let _r = coll
            .update_one(
                doc! {"_id": playlist_id},
                doc! {"$set": {"musics": new_musics}},
                None,
            )
            .await?;
        Ok(())
    }

    pub async fn edit_playlist(
        &self,
        playlist_id: ObjectId,
        name: String,
        is_public: bool,
    ) -> Result<()> {
        let coll = self._database.collection::<Playlist>("Playlist");
        let _r = coll
            .update_one(
                doc! {"_id": playlist_id},
                doc! {"$set": {"name": name, "public": is_public }},
                None,
            )
            .await?;
        Ok(())
    }
}
