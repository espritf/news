use super::model::{News, NewsInput};
use crate::schema::news;
use anyhow::Result;
use axum::async_trait;
use deadpool_diesel::sqlite::Pool;
use diesel::prelude::*;
use super::handlers::NewsRepository;

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

        let conn = self.pool.get().await?;
        let res = conn
            .interact(move |c| {
                news::table
                    .select(News::as_select())
                    .filter(date(news::pub_date).eq(sql(&format!(
                        "DATE('now', '-{} days', 'localtime')",
                        days_ago
                    ))))
                    .order(news::pub_date.desc())
                    .load::<News>(c)
            })
            .await??;

        Ok(res)
    }

    async fn create(&self, input: NewsInput) -> Result<News, Box<dyn std::error::Error>> {
        let conn = self.pool.get().await?;
        let res = conn
            .interact(move |c| {
                diesel::insert_into(news::table)
                    .values(&input)
                    .returning(News::as_returning())
                    .get_result(c)
            })
            .await??;

        Ok(res)
    }
}
