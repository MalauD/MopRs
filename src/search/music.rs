use serde::{Deserialize, Serialize};

use meilisearch_sdk::{errors::Error, task_info::TaskInfo};

use crate::{
    db::PaginationOptions,
    models::{DeezerId, Music},
    search::MeilisearchClient,
};

#[derive(Serialize, Deserialize)]
pub struct MusicMeilisearch {
    pub id: DeezerId,
    pub title: String,
    pub artist_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    pub rank: i64,
}

impl From<Music> for MusicMeilisearch {
    fn from(music: Music) -> Self {
        MusicMeilisearch {
            id: music.id,
            title: music.title,
            artist_name: music.artist_name,
            image_url: music.image_url,
            rank: music.rank,
        }
    }
}

impl MeilisearchClient {
    pub async fn init_musics_index(&self) -> Result<(), Error> {
        let index = self.client.index("musics");
        let _ = index
            .set_searchable_attributes(&["title", "artist_name", "rank"])
            .await?;
        let _ = index
            .set_ranking_rules(&[
                "words",
                "typo",
                "proximity",
                "attribute",
                "sort",
                "exactness",
                "rank:desc",
            ])
            .await?;
        Ok(())
    }

    pub async fn index_musics(&self, musics: Vec<Music>) -> Result<TaskInfo, Error> {
        let index = self.client.index("musics");
        let musics: Vec<MusicMeilisearch> = musics
            .into_iter()
            .map(|m| MusicMeilisearch::from(m))
            .collect();
        let t = index.add_documents(&musics, Some("id")).await?;
        Ok(t)
    }

    pub async fn search_musics(
        &self,
        query: String,
        page: PaginationOptions,
    ) -> Result<Vec<MusicMeilisearch>, Error> {
        let index = self.client.index("musics");
        let response = index
            .search()
            .with_query(&query)
            .with_limit(page.get_max_results())
            .with_offset(page.get_page() * page.get_max_results())
            .execute::<MusicMeilisearch>()
            .await?;
        Ok(response.hits.into_iter().map(|m| m.result).collect())
    }
}
