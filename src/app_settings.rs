use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct AppSettings {
    pub s3_url: String,
    pub arl: String,
    pub mongo_url: String,
}
