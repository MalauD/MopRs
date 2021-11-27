use crate::{
    db::MongoClient,
    models::{User, UserReq},
};
use mongodb::{bson::doc, error::Result};

impl MongoClient {
    pub async fn get_user(&self, user: &UserReq) -> Result<Option<User>> {
        let coll = self._database.collection::<User>("User");
        coll.find_one(doc! {"username": user.get_username()}, None)
            .await
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
}
