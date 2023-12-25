use std::time::Duration;

use log::info;
use meilisearch_sdk::{client::*, errors, task_info::TaskInfo};
use once_cell::sync::OnceCell;
use tokio::sync::Mutex;

pub struct MeilisearchClient {
    pub(in crate::search) client: Client,
}

pub struct MeilisearchConfig {
    pub host: String,
    pub api_key: Option<String>,
}

impl MeilisearchConfig {
    pub fn new(host: String, api_key: Option<String>) -> Self {
        Self { host, api_key }
    }
}

impl MeilisearchClient {
    pub async fn wait_for_task(&self, task: TaskInfo) -> Result<(), errors::Error> {
        task.wait_for_completion(&self.client, Some(Duration::from_millis(50)), None)
            .await?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn wait_for_tasks(&self, tasks: Vec<TaskInfo>) {
        let tasks: Vec<_> = tasks.into_iter().map(|t| self.wait_for_task(t)).collect();
        futures::future::join_all(tasks).await;
    }
}

static MEILISEARCH_CLIENT: OnceCell<MeilisearchClient> = OnceCell::new();
static MEILISEARCH_CLIENT_INITIALIZED: OnceCell<Mutex<bool>> = OnceCell::new();

pub async fn get_meilisearch(config: Option<MeilisearchConfig>) -> &'static MeilisearchClient {
    if let Some(c) = MEILISEARCH_CLIENT.get() {
        return c;
    }
    let initializing_mutex =
        MEILISEARCH_CLIENT_INITIALIZED.get_or_init(|| tokio::sync::Mutex::new(false));

    let mut initialized = initializing_mutex.lock().await;

    if !*initialized {
        info!(target: "mop-rs::meilisearch", "Connecting to meilisearch");
        if let Some(cfg) = config {
            let client = Client::new(cfg.host, cfg.api_key);
            let client = MeilisearchClient { client };
            client.init_musics_index().await.unwrap();
            client.init_artists_index().await.unwrap();
            client.init_albums_index().await.unwrap();
            info!(target: "mop-rs::meilisearch", "3/3 indexes initialized");
            if MEILISEARCH_CLIENT.set(client).is_ok() {
                info!(target: "mop-rs::meilisearch", "Connected to meilisearch");
                *initialized = true;
            }
        }
    }
    drop(initialized);
    MEILISEARCH_CLIENT.get().unwrap()
}
