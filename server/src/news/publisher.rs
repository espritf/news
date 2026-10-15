use crate::app::NewsRepository;
use crate::news::model::{News, NewsData, NewsInput};
use anyhow::Result;
use std::sync::Arc;

/// Publishes a news item: persists it together with a pending embedding job. Chunking and
/// embedding happen later in the `embedder` worker, so slow embedding calls don't hold up the
/// caller and failed embeddings are retried instead of lost.
#[derive(Clone)]
pub struct NewsPublisher {
    repo: Arc<dyn NewsRepository>,
}

impl NewsPublisher {
    pub fn new(repo: Arc<dyn NewsRepository>) -> Self {
        Self { repo }
    }

    pub async fn publish(&self, input: NewsInput) -> Result<News> {
        self.repo.create(NewsData::new(&input)).await
    }
}
