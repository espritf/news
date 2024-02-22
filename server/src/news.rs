use crate::app::AppState;
use crate::schema::news;
use anyhow::Result;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use chrono::NaiveDateTime;
use deadpool_diesel::sqlite::Pool;
use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::backend::Backend;
use serde::Serialize;
use diesel::prelude::*;
use diesel::sql_types::Text;

#[derive(Serialize, Debug, PartialEq, FromSqlRow)]
#[diesel(sql_type = Text)]
pub struct Sources(Vec<String>);

impl<DB> FromSql<Text, DB> for Sources
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let s = String::from_sql(bytes)?;
        Ok(Self(serde_json::from_str(&s)?))
    }
}

#[derive(Serialize, Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = news)]
pub struct News {
    title: String,
    pub_date: NaiveDateTime,
    sources: Sources,
}

async fn get_news(pool: &Pool, days_ago: u8) -> Result<Vec<News>, Box<dyn std::error::Error>> {

    use crate::schema::news::dsl::*;
    use diesel::dsl::{sql, date};

    let conn = pool.get().await?;
    let res = conn
        .interact(move |c| {
            news.select(News::as_select())
                .filter(date(pub_date).eq(sql(&format!("DATE('now', '-{} days', 'localtime')", days_ago))))
                .order(pub_date.desc())
                .load::<News>(c)
        })
        .await??;

    Ok(res)
}

// get news list handler
pub async fn news_list(State(state): State<AppState>, days_ago: Option<Path<u8>>) -> Result<Json<Vec<News>>, StatusCode> {

    let days_ago: u8 = match days_ago {
        Some(Path(s)) => s,
        None => 0,
    };

    let pool = state.get_pool();
    match get_news(pool, days_ago).await {
        Ok(news) => Ok(Json(news)),
        Err(e) => {
            tracing::error!("Error ocurred: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
