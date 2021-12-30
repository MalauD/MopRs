use bson::{doc, oid::ObjectId};
use mongodb::{error::Result, options::IndexOptions, IndexModel};

use crate::{
    db::MongoClient,
    models::{Playlist, User},
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

    pub async fn remove_playlist(&self, playlist_id: &ObjectId) -> Result<()> {
        let coll = self._database.collection::<Playlist>("Playlist");
        let _ = coll.delete_one(doc! {"_id": playlist_id}, None).await?;
        Ok(())
    }

    pub async fn create_playlist(
        &self,
        name: String,
        musics: &Vec<i32>,
        public: bool,
        creator: &User,
    ) -> Result<ObjectId> {
        let coll = self._database.collection::<Playlist>("Playlist");
        let r = coll
            .insert_one(
                doc! {"name":name, "creator": creator.id(), "public": public, "musics":musics},
                None,
            )
            .await?;
        Ok(r.inserted_id.as_object_id().unwrap())
    }
}
