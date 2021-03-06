use crate::{
    db::MongoClient,
    models::{User, UserReq},
};
use bson::oid::ObjectId;
use mongodb::{bson::doc, error::Result};

impl MongoClient {
    pub async fn get_user_req(&self, user: &UserReq) -> Result<Option<User>> {
        let coll = self._database.collection::<User>("User");
        coll.find_one(doc! {"username": user.get_username()}, None)
            .await
    }

    pub async fn get_user(&self, user: &ObjectId) -> Result<Option<User>> {
        let coll = self._database.collection::<User>("User");
        coll.find_one(doc! {"_id": user}, None).await
    }

    pub async fn save_user(&self, user: User) -> Result<ObjectId> {
        let coll = self._database.collection::<User>("User");
        let res = coll.insert_one(user, None).await?;
        Ok(res.inserted_id.as_object_id().unwrap())
    }

    pub async fn has_user_by_name(&self, user: &User) -> Result<bool> {
        let coll = self._database.collection::<User>("User");
        coll.count_documents(doc! {"username": user.get_username()}, None)
            .await
            .map(|c| c != 0)
    }

    pub async fn like_music(&self, user: &User, music_id: &i32) -> Result<bool> {
        let coll = self._database.collection::<User>("User");
        if user.liked_musics().contains(music_id) {
            let _ = coll
                .update_one(
                    doc! {"_id": user.id().unwrap()},
                    doc! {"$pull": {"liked_musics": music_id}},
                    None,
                )
                .await?;
            return Ok(false);
        } else {
            let _ = coll
                .update_one(
                    doc! {"_id": user.id().unwrap()},
                    doc! {"$push": {"liked_musics": music_id}},
                    None,
                )
                .await?;
            return Ok(true);
        }
    }

    pub async fn set_current_playlist_musics(
        &self,
        user: &User,
        musics_id: &Vec<i32>,
    ) -> Result<()> {
        let coll = self._database.collection::<User>("User");
        coll.update_one(
            doc! {"_id": user.id().unwrap()},
            doc! {"$set": {"current_playlist": musics_id}},
            None,
        )
        .await?;
        Ok(())
    }

    pub async fn set_current_playlist_index(&self, user: &User, index: &i32) -> Result<()> {
        let coll = self._database.collection::<User>("User");
        coll.update_one(
            doc! {"_id": user.id().unwrap()},
            doc! {"$set": {"current_playing": index}},
            None,
        )
        .await?;
        Ok(())
    }

    pub async fn add_to_history(&self, user: &User, music_id: &i32) -> Result<()> {
        let coll = self._database.collection::<User>("User");

        let _ = coll
            .update_one(
                doc! {"_id": user.id().unwrap()},
                doc! {"$push": {"viewed_musics": music_id}},
                None,
            )
            .await?;
        Ok(())
    }
}
