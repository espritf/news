use crate::app::{NewsRepository, VectorProvider};
use crate::news::model::{ChunkInput, News, NewsData, NewsInput};
use anyhow::Result;
use std::sync::Arc;

/// Publishes a news item: persists it immediately, then chunks and embeds its content
/// out-of-band so slow embedding calls don't hold up the caller.
#[derive(Clone)]
pub struct NewsPublisher {
    repo: Arc<dyn NewsRepository>,
    model: Arc<dyn VectorProvider>,
    max_chunk_chars: usize,
}

impl NewsPublisher {
    pub fn new(
        repo: Arc<dyn NewsRepository>,
        model: Arc<dyn VectorProvider>,
        max_chunk_chars: usize,
    ) -> Self {
        Self {
            repo,
            model,
            max_chunk_chars,
        }
    }

    pub async fn publish(&self, input: NewsInput) -> Result<News> {
        let chunks = input.search_chunks(self.max_chunk_chars);
        let data = NewsData::new(&input);

        let news = self.repo.create(data).await?;

        let news_id = news.id();
        let model = self.model.clone();
        let repo = self.repo.clone();

        tokio::spawn(async move {
            if let Err(e) = embed_chunks(model, repo, news_id, chunks).await {
                tracing::error!("Failed to embed chunks for news {}: {:?}", news_id, e);
            }
        });

        Ok(news)
    }
}

/// Embeds each chunk of a just-published news item and stores it, run out-of-band from the
/// `publish` request so slow embedding calls don't hold up the HTTP response.
async fn embed_chunks(
    model: Arc<dyn VectorProvider>,
    repo: Arc<dyn NewsRepository>,
    news_id: i32,
    texts: Vec<String>,
) -> Result<()> {
    let mut chunks = Vec::new();
    for (i, text) in texts.into_iter().enumerate() {
        let chunk_v = model.vector(&text).await?;
        chunks.push(ChunkInput {
            chunk_index: i as i32,
            chunk_text: text,
            chunk_v,
        });
    }

    repo.insert_chunks(news_id, chunks).await
}
