use crate::app::AppState;
use crate::schema::channels;
use crate::schema::items;
use crate::schema::news;
use anyhow::Result;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use chrono::NaiveDateTime;
use deadpool_diesel::sqlite::Pool;
use diesel::deserialize::Queryable;
use diesel::dsl::{date, now};
use diesel::RunQueryDsl;
use diesel::{ExpressionMethods, QueryDsl};
use serde::Serialize;

#[derive(Serialize, Queryable, PartialEq, Debug)]
pub struct News {
    title: String,
    pub_date: NaiveDateTime,
    link: String,
    source: String,
}

async fn get_news(pool: &Pool) -> Result<Vec<News>, Box<dyn std::error::Error>> {
    let conn = pool.get().await?;
    let res = conn
        .interact(|c| {
            news::table
                .inner_join(items::table.inner_join(channels::table))
                .select((news::title, news::pub_date, items::link, channels::title))
                .filter(date(news::pub_date).eq(date(now)))
                .order(news::pub_date.desc())
                .load::<News>(c)
        })
        .await??;

    Ok(res)
}

// get news list handler
pub async fn news_list(State(state): State<AppState>) -> Result<Json<Vec<News>>, StatusCode> {
    let pool = state.get_pool();
    match get_news(pool).await {
        Ok(news) => Ok(Json(news)),
        Err(e) => {
            tracing::error!("Error ocurred: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
