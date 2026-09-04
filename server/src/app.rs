use std::sync::Arc;
use crate::news::model::{ChunkInput, ListParams, News, NewsData};
use anyhow::Result;
use axum::async_trait;
use pgvector::Vector;
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait NewsRepository: Send + Sync {
    async fn list(&self, params: ListParams) -> Result<Vec<News>>;
    async fn create(&self, input: NewsData, chunks: Vec<ChunkInput>) -> Result<News>;
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
}
