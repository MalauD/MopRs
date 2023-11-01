use serde::{Deserialize, Serialize};

use meilisearch_sdk::{errors::Error, task_info::TaskInfo};

use crate::{
    db::PaginationOptions,
    models::{Artist, DeezerId},
    search::MeilisearchClient,
};

#[derive(Serialize, Deserialize)]
struct ArtistMeilisearch {
    pub id: DeezerId,
    pub name: String,
    pub picture: String,
}

impl From<Artist> for ArtistMeilisearch {
    fn from(artist: Artist) -> Self {
        ArtistMeilisearch {
            id: artist.id,
            name: artist.name,
            picture: artist.picture,
        }
    }
}

impl MeilisearchClient {
    pub async fn init_artists_index(&self) -> Result<(), Error> {
        let index = self.client.index("artists");
        let _ = index.set_searchable_attributes(&["name"]).await?;
        Ok(())
    }

    pub async fn index_artists(&self, artists: Vec<Artist>) -> Result<TaskInfo, Error> {
        let index = self.client.index("artists");
        let artists: Vec<ArtistMeilisearch> = artists
            .into_iter()
            .map(ArtistMeilisearch::from)
            .collect();
        let t = index.add_documents(&artists, Some("id")).await?;
        Ok(t)
    }

    pub async fn search_artists(
        &self,
        query: String,
        page: PaginationOptions,
    ) -> Result<Vec<DeezerId>, Error> {
        let index = self.client.index("artists");
        let response = index
            .search()
            .with_query(&query)
            .with_limit(page.get_max_results())
            .with_offset(page.get_page() * page.get_max_results())
            .execute::<ArtistMeilisearch>()
            .await?;
        let ids: Vec<DeezerId> = response.hits.into_iter().map(|m| m.result.id).collect();
        Ok(ids)
    }
}
