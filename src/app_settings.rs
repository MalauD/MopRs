use once_cell::sync::OnceCell;
use serde::Deserialize;
use serde_with::{serde_as, DurationMilliSeconds, DurationSeconds};
use std::{convert::TryFrom, time::Duration};
use tokio::sync::Mutex;

fn default_session_duration() -> Duration {
    Duration::from_secs(3600 * 24 * 7)
}

fn default_artists_update_interval() -> Duration {
    Duration::from_secs(60 * 60)
}

fn default_artist_scrape_update_interval() -> Duration {
    Duration::from_secs(60 * 60 * 24 * 3)
}

fn default_artist_periodic_scrape_check_interval() -> Duration {
    Duration::from_secs(60 * 30)
}

fn default_artist_periodic_scrape_update_interval() -> Duration {
    Duration::from_secs(60 * 60 * 24 * 7)
}

fn default_artist_scrape_cooldown() -> Duration {
    Duration::from_millis(100)
}

fn default_s3_region() -> String {
    "".to_string()
}

#[serde_as]
#[derive(Deserialize, Debug, Clone)]
pub struct AppSettings {
    pub s3_url: String,
    #[serde(default = "default_s3_region")]
    pub s3_region: String,
    #[serde(default)]
    pub s3_bucket: String,
    pub arl: String,
    pub mongo_url: String,
    pub meilisearch_host: String,
    pub meilisearch_api_key: String,
    pub redis_service_host: String,
    pub redis_service_port: String,
    pub redis_password: Option<String>,
    pub redis_username: Option<String>,
    pub session_key: Option<String>,
    #[serde(default = "default_session_duration")]
    #[serde_as(as = "DurationSeconds<u64>")]
    pub session_duration: Duration,
    #[serde(default = "default_artists_update_interval")]
    #[serde_as(as = "DurationSeconds<u64>")]
    pub artist_update_interval: Duration,
    #[serde(default = "default_artist_scrape_update_interval")]
    #[serde_as(as = "DurationSeconds<u64>")]
    pub artist_scrape_update_interval: Duration,
    #[serde(default = "default_artist_periodic_scrape_check_interval")]
    #[serde_as(as = "DurationSeconds<u64>")]
    pub artist_periodic_scrape_check_interval: Duration,
    #[serde(default = "default_artist_periodic_scrape_update_interval")]
    #[serde_as(as = "DurationSeconds<u64>")]
    pub artist_periodic_scrape_update_interval: Duration,
    #[serde(default = "default_artist_scrape_cooldown")]
    #[serde_as(as = "DurationMilliSeconds<u64>")]
    pub artist_scrape_cooldown: Duration,
}

impl AppSettings {
    pub fn get_session_duration(&self) -> actix_web::cookie::time::Duration {
        actix_web::cookie::time::Duration::try_from(self.session_duration).unwrap()
    }

    pub fn get_artist_update_interval(&self) -> chrono::Duration {
        chrono::Duration::from_std(self.artist_update_interval).unwrap()
    }

    pub fn get_artist_scrape_update_interval(&self) -> chrono::Duration {
        chrono::Duration::from_std(self.artist_scrape_update_interval).unwrap()
    }

    pub fn get_artist_periodic_scrape_update_interval(&self) -> chrono::Duration {
        chrono::Duration::from_std(self.artist_periodic_scrape_update_interval).unwrap()
    }
}

static APP_SETTINGS: OnceCell<AppSettings> = OnceCell::new();
static APP_SETTINGS_INITIALIZED: OnceCell<Mutex<bool>> = OnceCell::new();

pub async fn get_settings(base_settings: Option<AppSettings>) -> &'static AppSettings {
    if let Some(c) = APP_SETTINGS.get() {
        return c;
    }
    let initializing_mutex =
        APP_SETTINGS_INITIALIZED.get_or_init(|| tokio::sync::Mutex::new(false));

    let mut initialized = initializing_mutex.lock().await;

    if !*initialized {
        if APP_SETTINGS.set(base_settings.unwrap()).is_ok() {
            *initialized = true;
        }
    }
    drop(initialized);
    APP_SETTINGS.get().unwrap()
}

pub fn get_settings_sync() -> &'static AppSettings {
    APP_SETTINGS.get().unwrap()
}
