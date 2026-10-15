use std::sync::Arc;
use std::time::Duration;
use crate::embedder::model::EmbeddingJob;
use crate::events::EmbeddingEvent;
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
    /// Stores the news item and enqueues its embedding job in the same transaction.
    async fn create(&self, input: NewsData) -> Result<News>;
}

/// Durable queue of pending embedding work, consumed by the `embedder` worker binary.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait EmbeddingJobRepository: Send + Sync {
    /// Leases the next due job for `lease`, so other workers skip it; a job whose lease
    /// expires (e.g. the worker crashed) becomes claimable again.
    async fn claim(&self, lease: Duration) -> Result<Option<EmbeddingJob>>;
    /// Replaces the news item's chunks with `chunks` and removes its job.
    async fn complete(&self, news_id: i32, chunks: Vec<ChunkInput>) -> Result<()>;
    /// Releases the job and schedules it to be retried after `retry_in`.
    async fn fail(&self, news_id: i32, error: String, retry_in: Duration) -> Result<()>;
}

/// Statistics events for later analysis.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait EventSink: Send + Sync {
    async fn embedding(&self, event: EmbeddingEvent) -> Result<()>;
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

impl AppState {
    pub fn publisher(&self) -> NewsPublisher {
        NewsPublisher::new(self.repo.clone())
    }

    pub fn lister(&self) -> NewsLister {
        NewsLister::new(self.repo.clone(), self.model.clone())
    }
}
