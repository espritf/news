use crate::app::AppState;
use crate::schema::news;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use chrono::NaiveDateTime;
use deadpool_diesel::sqlite::Pool;
use diesel::deserialize::Queryable;
use diesel::query_dsl::methods::{OrderDsl, SelectDsl};
use diesel::{ExpressionMethods, RunQueryDsl, Selectable, SelectableHelper};
use serde::Serialize;
use anyhow::Result;

#[derive(Serialize, Selectable, Queryable)]
#[diesel(table_name = news)]
pub struct News {
    title: String,
    pub_date: NaiveDateTime,
}

async fn get_news(pool: &Pool) -> Result<Vec<News>, Box<dyn std::error::Error>> {
    use crate::schema::news::dsl::*;

    let conn = pool.get().await?;
    let res = conn.interact(|c| {
        news.select(News::as_select())
            .order(pub_date.desc())
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
