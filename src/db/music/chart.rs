use crate::{db::MongoClient, models::Chart};
use chrono::Utc;
use mongodb::{bson::doc, error::Result};
use now::DateTimeNow;

impl MongoClient {
    pub async fn get_chart_today(&self) -> Result<Option<Chart>> {
        let coll = self._database.collection::<Chart>("Chart");
        let today = Utc::now();
        let res =coll.find_one(doc!{"published_date": {"$gt": today.beginning_of_day(), "$lt": today.end_of_day()}}, None).await.unwrap();
        Ok(res)
    }

    pub async fn insert_chart(&self, ch: &Chart) -> Result<()> {
        let coll = self._database.collection::<Chart>("Chart");
        coll.insert_one(ch, None).await?;
        Ok(())
    }
}
