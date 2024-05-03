use crate::app::NewsRepository;
use super::model::{News, NewsData, QueryParams};
use crate::pool::Pool;
use crate::schema::news;
use anyhow::Result;
use axum::async_trait;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

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
    async fn list(&self, params: QueryParams) -> Result<Vec<News>, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get().await?;
        let order = match params.search {
            Some(_) => {
                todo!("implement semantic search")
            }
            None => news::pub_date.desc(),
        };
        let res = news::table
            .select(News::as_select())
            .order(order)
            .limit(params.limit as i64)
            .load::<News>(&mut conn)
            .await?;

        Ok(res)
    }

    async fn create(&self, input: NewsData) -> Result<News, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get().await?;

        let res = diesel::insert_into(news::table)
            .values(&input)
            .returning(News::as_returning())
            .get_result(&mut conn)
            .await?;

        Ok(res)
    }
}
