use std::sync::Arc;
use crate::news::model::{News, NewsData, QueryParams};
use anyhow::Result;
use axum::async_trait;
use pgvector::Vector;
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait NewsRepository: Send + Sync {
    async fn list(&self, params: QueryParams) -> Result<Vec<News>, Box<dyn std::error::Error>>;
    async fn create(&self, input: NewsData) -> Result<News, Box<dyn std::error::Error>>;
}

#[cfg_attr(test, automock)]
pub trait VectorProvider: Send + Sync {
   fn vector(&self, text: &str) -> Result<Vector>; 
}

#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<dyn NewsRepository>,
    pub model: Arc<dyn VectorProvider>,
}
