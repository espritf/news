use crate::app::NewsRepository;
use super::model::{ChunkInput, ListParams, News, NewsChunkData, NewsData};
use crate::pool::Pool;
use crate::schema::{news, news_chunks};
use anyhow::Result;
use axum::async_trait;
use diesel::prelude::*;
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::{AsyncConnection, RunQueryDsl};
use pgvector::VectorExpressionMethods;

pub struct NewsRepositoryImpl {
    pool: Pool,
}

impl NewsRepositoryImpl {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NewsRepository for NewsRepositoryImpl {
    async fn list(&self, params: ListParams) -> Result<Vec<News>> {
        let mut conn = self.pool.get().await?;

        match params.search {
            Some(query) => {
                // Over-fetch chunk matches (closest first) and dedupe by news_id in order,
                // since a single article can have several matching chunks.
                let matches: Vec<i32> = news_chunks::table
                    .select(news_chunks::news_id)
                    .order(news_chunks::chunk_v.l2_distance(query))
                    .limit(params.limit as i64 * 5)
                    .load::<i32>(&mut conn)
                    .await?;

                let mut ids: Vec<i32> = Vec::new();
                for id in matches {
                    if !ids.contains(&id) {
                        ids.push(id);
                    }
                    if ids.len() >= params.limit as usize {
                        break;
                    }
                }

                let mut res = news::table
                    .select(News::as_select())
                    .filter(news::id.eq_any(&ids))
                    .load::<News>(&mut conn)
                    .await?;

                res.sort_by_key(|n| ids.iter().position(|id| *id == n.id()).unwrap_or(usize::MAX));

                Ok(res)
            }
            None => {
                let res = news::table
                    .select(News::as_select())
                    .order(news::pub_date.desc())
                    .limit(params.limit as i64)
                    .load::<News>(&mut conn)
                    .await?;

                Ok(res)
            }
        }
    }

    async fn create(&self, input: NewsData, chunks: Vec<ChunkInput>) -> Result<News> {
        let mut conn = self.pool.get().await?;

        let news: News = conn
            .transaction::<_, diesel::result::Error, _>(|conn| {
                async move {
                    let news = diesel::insert_into(news::table)
                        .values(&input)
                        .returning(News::as_returning())
                        .get_result::<News>(conn)
                        .await?;

                    let rows: Vec<NewsChunkData> = chunks
                        .into_iter()
                        .map(|chunk| NewsChunkData::new(news.id(), chunk))
                        .collect();

                    if !rows.is_empty() {
                        diesel::insert_into(news_chunks::table)
                            .values(&rows)
                            .execute(conn)
                            .await?;
                    }

                    Ok(news)
                }
                .scope_boxed()
            })
            .await?;

        Ok(news)
    }
}
