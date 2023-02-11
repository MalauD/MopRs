use bson::{doc, oid::ObjectId};
use futures::StreamExt;
use mongodb::{
    error::Result,
    options::{FindOptions, IndexOptions},
    IndexModel,
};

use crate::{
    db::{MongoClient, PaginationOptions},
    models::{DeezerId, Playlist, User},
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
            .await
            .unwrap();
        Ok(())
    }

    pub async fn get_playlist(&self, playlist_id: &ObjectId) -> Result<Option<Playlist>> {
        let coll = self._database.collection::<Playlist>("Playlist");
        Ok(coll.find_one(doc! {"_id": playlist_id}, None).await?)
    }

    pub async fn get_user_playlists(
        &self,
        user: &ObjectId,
        pagination: &PaginationOptions,
    ) -> Result<Vec<Playlist>> {
        let coll = self._database.collection::<Playlist>("Playlist");

        let _ = FindOptions::builder()
            .limit(pagination.get_max_results() as i64)
            .skip(Some(
                (pagination.get_page() * pagination.get_max_results()) as u64,
            ))
            .build();

        let mut cursor = coll.find(doc! {"creator": user}, None).await?;

        let mut result = Vec::<Playlist>::with_capacity(pagination.get_max_results().max(20));
        while let Some(value) = cursor.next().await {
            if let Ok(res) = value {
                result.push(res);
            }
        }
        Ok(result)
    }

    pub async fn remove_playlist(&self, playlist_id: &ObjectId) -> Result<()> {
        let coll = self._database.collection::<Playlist>("Playlist");
        let _ = coll.delete_one(doc! {"_id": playlist_id}, None).await?;
        Ok(())
    }

    pub async fn create_playlist(
        &self,
        name: String,
        musics: &Vec<DeezerId>,
        public: bool,
        creator: &User,
    ) -> Result<ObjectId> {
        let coll = self._database.collection::<Playlist>("Playlist");
        let id = ObjectId::new();
        let pl = Playlist::new(
            id,
            name,
            creator.id().unwrap(),
            public,
            Some(musics.to_vec()),
        );
        let _ = coll.insert_one(pl, None).await?;
        Ok(id)
    }

    pub async fn add_musics_playlist(
        &self,
        playlist_id: ObjectId,
        music: &Vec<DeezerId>,
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
        new_musics: &Vec<DeezerId>,
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
}
