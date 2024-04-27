use super::handlers::NewsRepository;
use super::model::{News, NewsData};
use crate::schema::news;
use anyhow::Result;
use axum::async_trait;
use diesel::prelude::*;
use crate::pool::Pool;
use diesel_async::{RunQueryDsl};

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
    async fn list(&self, days_ago: u8) -> Result<Vec<News>, Box<dyn std::error::Error>> {
        use diesel::dsl::{date, sql};

        let mut conn = self.pool.get().await?;
        let res = news::table
            .select(News::as_select())
            .filter(date(news::pub_date).eq(sql(&format!(
                "date(now() - interval '{} days')",
                days_ago
            ))))
            .order(news::pub_date.desc())
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
