use super::model::EmbeddingJob;
use crate::app::EmbeddingJobRepository;
use crate::news::model::{ChunkInput, NewsChunkData};
use crate::pool::Pool;
use crate::schema::{embedding_jobs, news_chunks};
use anyhow::Result;
use axum::async_trait;
use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::sql_types::{Double, Timestamptz};
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::{AsyncConnection, RunQueryDsl};
use std::time::Duration;

pub struct EmbeddingJobRepositoryImpl {
    pool: Pool,
}

impl EmbeddingJobRepositoryImpl {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

/// Picks the oldest due, unleased job with `SKIP LOCKED` so concurrent workers never claim the
/// same row, bumps its attempt counter and leases it for `$1` seconds.
const CLAIM_SQL: &str = r#"
update embedding_jobs j
set attempts     = j.attempts + 1,
    locked_until = now() + make_interval(secs => $1)
from news n
where n.id = j.news_id
  and j.news_id = (select news_id
                   from embedding_jobs
                   where run_after <= now()
                     and (locked_until is null or locked_until < now())
                   order by run_after
                   limit 1 for update skip locked)
returning j.news_id, j.attempts, n.content
"#;

#[async_trait]
impl EmbeddingJobRepository for EmbeddingJobRepositoryImpl {
    async fn claim(&self, lease: Duration) -> Result<Option<EmbeddingJob>> {
        let mut conn = self.pool.get().await?;

        let job = diesel::sql_query(CLAIM_SQL)
            .bind::<Double, _>(lease.as_secs_f64())
            .get_result::<EmbeddingJob>(&mut conn)
            .await
            .optional()?;

        Ok(job)
    }

    async fn complete(&self, news_id: i32, chunks: Vec<ChunkInput>) -> Result<()> {
        let mut conn = self.pool.get().await?;

        let rows: Vec<NewsChunkData> = chunks
            .into_iter()
            .map(|chunk| NewsChunkData::new(news_id, chunk))
            .collect();

        conn.transaction::<_, anyhow::Error, _>(|conn| {
            async move {
                // Drop chunks left by an earlier attempt whose lease expired mid-way.
                diesel::delete(news_chunks::table.filter(news_chunks::news_id.eq(news_id)))
                    .execute(conn)
                    .await?;

                if !rows.is_empty() {
                    diesel::insert_into(news_chunks::table)
                        .values(&rows)
                        .execute(conn)
                        .await?;
                }

                diesel::delete(embedding_jobs::table.find(news_id))
                    .execute(conn)
                    .await?;

                Ok(())
            }
            .scope_boxed()
        })
        .await
    }

    async fn fail(&self, news_id: i32, error: String, retry_in: Duration) -> Result<()> {
        let mut conn = self.pool.get().await?;

        let run_after = sql::<Timestamptz>("now() + make_interval(secs => ")
            .bind::<Double, _>(retry_in.as_secs_f64())
            .sql(")");

        diesel::update(embedding_jobs::table.find(news_id))
            .set((
                embedding_jobs::locked_until.eq(None::<chrono::DateTime<chrono::Utc>>),
                embedding_jobs::last_error.eq(Some(error)),
                embedding_jobs::run_after.eq(run_after),
            ))
            .execute(&mut conn)
            .await?;

        Ok(())
    }
}
