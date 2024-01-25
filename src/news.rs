use crate::app::AppState;
use crate::schema::news;
use axum::extract::State;
use axum::Json;
use chrono::NaiveDateTime;
use deadpool_diesel::sqlite::Pool;
use diesel::deserialize::Queryable;
use diesel::query_dsl::methods::SelectDsl;
use diesel::{RunQueryDsl, Selectable, SelectableHelper};
use serde::Serialize;
use anyhow::Result;

#[derive(Serialize, Selectable, Queryable)]
#[diesel(table_name = news)]
pub struct News {
    title: String,
    pub_date: NaiveDateTime,
}

async fn get_news(pool: &Pool) -> Result<Vec<News>> {
    use crate::schema::news::dsl::*;

    let conn = pool.get().await?;
    let res = conn.interact(|c| {
        news.select(News::as_select())
            .load::<News>(c)
    })
    .await
    .unwrap()
    .unwrap(); // TODO handle errors

    Ok(res)
}


// get news list handler
pub async fn news_list(State(state): State<AppState>) -> Json<Vec<News>> {

    let pool = state.get_pool();
    let news = get_news(pool).await.unwrap(); // TODO handle errors

    Json(news)
}
