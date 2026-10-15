use super::model::EmbeddingJob;
use crate::app::{EmbeddingJobRepository, VectorProvider};
use crate::news::chunker::chunk_text;
use crate::news::model::ChunkInput;
use anyhow::Result;
use std::sync::Arc;
use std::time::Duration;

/// How long a claimed job stays invisible to other workers. Must comfortably exceed the time
/// to embed a long article; if the worker dies, the job is picked up again once it expires.
const LEASE: Duration = Duration::from_secs(30 * 60);
/// How long to sleep when the queue is empty (or the database is unreachable).
const POLL_INTERVAL: Duration = Duration::from_secs(5);
const BASE_BACKOFF: Duration = Duration::from_secs(30);
const MAX_BACKOFF: Duration = Duration::from_secs(60 * 60);

/// Drains the `embedding_jobs` queue: chunks each news item's content, embeds every chunk and
/// stores the result. Failed jobs are retried with exponential backoff.
pub struct EmbeddingWorker {
    jobs: Arc<dyn EmbeddingJobRepository>,
    model: Arc<dyn VectorProvider>,
    max_chunk_chars: usize,
}

impl EmbeddingWorker {
    pub fn new(
        jobs: Arc<dyn EmbeddingJobRepository>,
        model: Arc<dyn VectorProvider>,
        max_chunk_chars: usize,
    ) -> Self {
        Self {
            jobs,
            model,
            max_chunk_chars,
        }
    }

    pub async fn run(&self) {
        loop {
            match self.run_once().await {
                Ok(true) => {}
                Ok(false) => tokio::time::sleep(POLL_INTERVAL).await,
                Err(e) => {
                    tracing::error!("Embedding worker error: {:?}", e);
                    tokio::time::sleep(POLL_INTERVAL).await;
                }
            }
        }
    }

    /// Processes a single job, returning `false` when there was nothing to do.
    pub async fn run_once(&self) -> Result<bool> {
        let Some(job) = self.jobs.claim(LEASE).await? else {
            return Ok(false);
        };

        tracing::info!("Embedding news {} (attempt {})", job.news_id, job.attempts);

        match self.embed(&job).await {
            Ok(chunks) => self.jobs.complete(job.news_id, chunks).await?,
            Err(e) => {
                let retry_in = backoff(job.attempts);
                tracing::error!(
                    "Failed to embed news {} (attempt {}), retrying in {:?}: {:?}",
                    job.news_id,
                    job.attempts,
                    retry_in,
                    e
                );
                self.jobs.fail(job.news_id, format!("{:#}", e), retry_in).await?;
            }
        }

        Ok(true)
    }

    async fn embed(&self, job: &EmbeddingJob) -> Result<Vec<ChunkInput>> {
        let mut chunks = Vec::new();
        for (i, text) in chunk_text(&job.content, self.max_chunk_chars).into_iter().enumerate() {
            let chunk_v = self.model.vector(&text).await?;
            chunks.push(ChunkInput {
                chunk_index: i as i32,
                chunk_text: text,
                chunk_v,
            });
        }
        Ok(chunks)
    }
}

/// Exponential backoff: 30s, 1m, 2m, ... capped at one hour.
fn backoff(attempts: i32) -> Duration {
    let exp = attempts.saturating_sub(1).clamp(0, 16) as u32;
    BASE_BACKOFF.saturating_mul(1 << exp).min(MAX_BACKOFF)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::{MockEmbeddingJobRepository, MockVectorProvider};
    use mockall::predicate::eq;

    fn job(content: &str) -> EmbeddingJob {
        EmbeddingJob {
            news_id: 7,
            attempts: 2,
            content: content.to_string(),
        }
    }

    #[tokio::test]
    async fn test_idle_when_queue_empty() {
        let mut jobs = MockEmbeddingJobRepository::new();
        jobs.expect_claim().return_once(|_| Ok(None));

        let worker = EmbeddingWorker::new(Arc::new(jobs), Arc::new(MockVectorProvider::new()), 10);

        assert!(!worker.run_once().await.unwrap());
    }

    #[tokio::test]
    async fn test_completes_job_with_all_chunks() {
        let mut jobs = MockEmbeddingJobRepository::new();
        jobs.expect_claim().return_once(|_| Ok(Some(job("one two three"))));
        jobs.expect_complete()
            .withf(|id, chunks| {
                *id == 7
                    && chunks.iter().map(|c| c.chunk_text.as_str()).collect::<Vec<_>>()
                        == ["one two", "three"]
            })
            .return_once(|_, _| Ok(()));
        jobs.expect_fail().never();

        let mut vp = MockVectorProvider::new();
        vp.expect_vector()
            .times(2)
            .returning(|_| Ok(pgvector::Vector::from(vec![1.0])));

        let worker = EmbeddingWorker::new(Arc::new(jobs), Arc::new(vp), 7);

        assert!(worker.run_once().await.unwrap());
    }

    #[tokio::test]
    async fn test_reschedules_failed_job() {
        let mut jobs = MockEmbeddingJobRepository::new();
        jobs.expect_claim().return_once(|_| Ok(Some(job("one two three"))));
        jobs.expect_complete().never();
        jobs.expect_fail()
            .with(eq(7), eq("ollama down".to_string()), eq(Duration::from_secs(60)))
            .return_once(|_, _, _| Ok(()));

        let mut vp = MockVectorProvider::new();
        vp.expect_vector()
            .return_once(|_| Err(anyhow::anyhow!("ollama down")));

        let worker = EmbeddingWorker::new(Arc::new(jobs), Arc::new(vp), 7);

        assert!(worker.run_once().await.unwrap());
    }

    #[test]
    fn test_backoff() {
        assert_eq!(backoff(1), Duration::from_secs(30));
        assert_eq!(backoff(3), Duration::from_secs(120));
        assert_eq!(backoff(100), MAX_BACKOFF);
    }
}
