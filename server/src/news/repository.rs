use crate::app::NewsRepository;
use super::model::{News, NewsData, ListParams};
use crate::pool::Pool;
use crate::schema::news;
use anyhow::Result;
use axum::async_trait;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use pgvector::VectorExpressionMethods;

use diesel::expression::expression_types::NotSelectable;
type DB = diesel::pg::Pg;

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
    async fn list(&self, params: ListParams) -> Result<Vec<News>, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get().await?;
        let order: Box<dyn BoxableExpression<news::table, DB, SqlType = NotSelectable>> = match params.search {
            Some(query) => Box::new(news::title_v.l2_distance(query).asc()),
            None => Box::new(news::pub_date.desc()),
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
