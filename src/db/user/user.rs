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

    pub async fn get_user(&self, user: &User) -> Result<Option<User>> {
        let coll = self._database.collection::<User>("User");
        coll.find_one(doc! {"_id": user.id()}, None).await
    }

    pub async fn save_user(&self, user: User) -> Result<()> {
        let coll = self._database.collection::<User>("User");
        coll.insert_one(user, None).await?;
        Ok(())
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
}
