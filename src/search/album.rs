use serde::{Deserialize, Serialize};

use meilisearch_sdk::{errors::Error, task_info::TaskInfo};

use crate::{
    db::PaginationOptions,
    models::{Album, DeezerId},
    search::MeilisearchClient,
};

#[derive(Serialize, Deserialize)]
struct AlbumMeilisearch {
    pub id: DeezerId,
    pub name: String,
    pub cover: String,
}

impl From<Album> for AlbumMeilisearch {
    fn from(album: Album) -> Self {
        AlbumMeilisearch {
            id: album.id,
            name: album.name,
            cover: album.cover,
        }
    }
}

impl Into<Album> for AlbumMeilisearch {
    fn into(self) -> Album {
        Album {
            id: self.id,
            name: self.name,
            cover: self.cover,
            is_complete: false,
            musics: None,
        }
    }
}

impl MeilisearchClient {
    pub async fn init_albums_index(&self) -> Result<(), Error> {
        let index = self.client.index("albums");
        let _ = index.set_searchable_attributes(&["name"]).await?;
        Ok(())
    }

    pub async fn index_albums(&self, albums: Vec<Album>) -> Result<TaskInfo, Error> {
        let index = self.client.index("albums");
        let albums: Vec<AlbumMeilisearch> =
            albums.into_iter().map(AlbumMeilisearch::from).collect();
        let t = index.add_documents(&albums, Some("id")).await?;
        Ok(t)
    }

    pub async fn search_albums(
        &self,
        query: String,
        page: PaginationOptions,
    ) -> Result<Vec<Album>, Error> {
        let index = self.client.index("albums");
        let response = index
            .search()
            .with_query(&query)
            .with_limit(page.get_max_results())
            .with_offset(page.get_page() * page.get_max_results())
            .execute::<AlbumMeilisearch>()
            .await?;
        Ok(response.hits.into_iter().map(|m| m.result.into()).collect())
    }
}
