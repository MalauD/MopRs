use std::time::Duration;

use log::info;
use meilisearch_sdk::{client::*, errors, task_info::TaskInfo};

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
    pub async fn connect(config: MeilisearchConfig) -> Result<Self, errors::Error> {
        info!(target: "mop-rs::meilisearch", "Connecting to meilisearch");
        let client = Self {
            client: Client::new(config.host, config.api_key),
        };
        client.init_musics_index().await?;
        client.init_artists_index().await?;
        client.init_albums_index().await?;
        info!(target: "mop-rs::meilisearch", "3/3 indexes initialized");
        info!(target: "mop-rs::meilisearch", "Connected to meilisearch");
        Ok(client)
    }

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
