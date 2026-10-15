use crate::{
    db::MongoClient,
    deezer::DeezerMusicFormats,
    models::{DeezerId, PublicUser, User, UserReq},
};
use bson::oid::ObjectId;
use mongodb::{bson::doc, error::{Error, Result}};

fn required_user_id(user: &User) -> Result<ObjectId> {
    user.id()
        .ok_or_else(|| Error::custom("user is missing an object id"))
}

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

    pub async fn get_user_public(&self, user: &ObjectId) -> Result<Option<PublicUser>> {
        let coll = self._database.collection::<PublicUser>("User");
        let opt = {
            let mut opt = mongodb::options::FindOneOptions::default();
            opt.projection = Some(doc! { "viewed_musics": 0});
            opt
        };
        coll.find_one(doc! {"_id": user}, opt).await
    }

    pub async fn save_user(&self, user: User) -> Result<ObjectId> {
        let coll = self._database.collection::<User>("User");
        let res = coll.insert_one(user, None).await?;
        res.inserted_id
            .as_object_id()
            .ok_or_else(|| Error::custom("inserted user id is not an ObjectId"))
    }

    pub async fn has_user_by_name(&self, user: &User) -> Result<bool> {
        let coll = self._database.collection::<User>("User");
        coll.count_documents(doc! {"username": user.get_username()}, None)
            .await
            .map(|c| c != 0)
    }

    pub async fn like_music(&self, user: &User, music_id: &DeezerId) -> Result<bool> {
        let coll = self._database.collection::<User>("User");
        let user_id = required_user_id(user)?;
        if user.liked_musics().contains(music_id) {
            let _ = coll
                .update_one(
                    doc! {"_id": user_id},
                    doc! {"$pull": {"liked_musics": music_id}},
                    None,
                )
                .await?;
            Ok(false)
        } else {
            let _ = coll
                .update_one(
                    doc! {"_id": user_id},
                    doc! {"$push": {"liked_musics": music_id}},
                    None,
                )
                .await?;
            Ok(true)
        }
    }

    pub async fn set_current_playlist_musics(
        &self,
        user: &User,
        musics_id: &[DeezerId],
    ) -> Result<()> {
        let coll = self._database.collection::<User>("User");
        let user_id = required_user_id(user)?;
        coll.update_one(
            doc! {"_id": user_id},
            doc! {"$set": {"current_playlist": musics_id}},
            None,
        )
        .await?;
        Ok(())
    }

    pub async fn set_current_playlist_index(&self, user: &User, index: &DeezerId) -> Result<()> {
        let coll = self._database.collection::<User>("User");
        let user_id = required_user_id(user)?;
        coll.update_one(
            doc! {"_id": user_id},
            doc! {"$set": {"current_playing": index}},
            None,
        )
        .await?;
        Ok(())
    }

    pub async fn add_to_history(&self, user: &User, music_id: &DeezerId) -> Result<()> {
        let coll = self._database.collection::<User>("User");
        let user_id = required_user_id(user)?;

        let _ = coll
            .update_one(
                doc! {"_id": user_id},
                doc! {"$push": {"viewed_musics": music_id}},
                None,
            )
            .await?;
        Ok(())
    }

    pub async fn get_history(&self, user: &User, size: i64) -> Result<Vec<DeezerId>> {
        let coll = self._database.collection::<User>("User");
        let user_id = required_user_id(user)?;
        let opt = {
            let mut opt = mongodb::options::FindOneOptions::default();
            opt.projection = Some(doc! {"viewed_musics": doc! {"$slice": -size}});
            opt
        };
        let user = coll
            .find_one(doc! {"_id": user_id}, opt)
            .await?
            .ok_or_else(|| Error::custom("user not found when reading history"))?;
        Ok(user.viewed_musics().to_vec())
    }

    pub async fn get_liked_musics(&self, user: &User, size: i64) -> Result<Vec<DeezerId>> {
        let coll = self._database.collection::<User>("User");
        let user_id = required_user_id(user)?;
        let opt = {
            let mut opt = mongodb::options::FindOneOptions::default();
            opt.projection = Some(doc! {"liked_musics": doc! {"$slice": -size}});
            opt
        };
        let user = coll
            .find_one(doc! {"_id": user_id}, opt)
            .await?
            .ok_or_else(|| Error::custom("user not found when reading liked musics"))?;
        Ok(user.liked_musics().to_vec())
    }

    pub async fn set_prefered_format(
        &self,
        user: &User,
        format: &DeezerMusicFormats,
    ) -> Result<()> {
        let coll = self._database.collection::<User>("User");
        let user_id = required_user_id(user)?;
        coll.update_one(
            doc! {"_id": user_id},
            doc! {"$set": {"prefered_format": format.to_string()}},
            None,
        )
        .await?;
        Ok(())
    }
}
