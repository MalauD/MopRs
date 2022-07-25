use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct AppSettings {
    pub music_path: String,
    pub arl: String,
    pub mongo_url: String,
}
