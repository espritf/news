use std::sync::Arc;
use crate::news::lister::NewsLister;
use crate::news::model::{ChunkInput, ListParams, News, NewsData};
use crate::news::publisher::NewsPublisher;
use anyhow::Result;
use axum::async_trait;
use pgvector::Vector;
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait NewsRepository: Send + Sync {
    async fn list(&self, params: ListParams) -> Result<Vec<News>>;
    async fn create(&self, input: NewsData) -> Result<News>;
    async fn insert_chunks(&self, news_id: i32, chunks: Vec<ChunkInput>) -> Result<()>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait VectorProvider: Send + Sync {
   async fn vector(&self, text: &str) -> Result<Vector>; 
}

#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<dyn NewsRepository>,
    pub model: Arc<dyn VectorProvider>,
    pub max_chunk_chars: usize,
}

impl AppState {
    pub fn publisher(&self) -> NewsPublisher {
        NewsPublisher::new(self.repo.clone(), self.model.clone(), self.max_chunk_chars)
    }

    pub fn lister(&self) -> NewsLister {
        NewsLister::new(self.repo.clone(), self.model.clone())
    }
}
