use crate::app::AppState;
use crate::schema::news;
use anyhow::Result;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use chrono::NaiveDateTime;
use deadpool_diesel::sqlite::Pool;
use diesel::AsExpression;
use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::backend::Backend;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::Text;

#[derive(Serialize, Deserialize, Debug, PartialEq, FromSqlRow, AsExpression)]
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

// https://docs.rs/diesel/2.1.4/diesel/serialize/trait.ToSql.html#
impl ToSql<Text, diesel::sqlite::Sqlite> for Sources
    where
        String: ToSql<Text, diesel::sqlite::Sqlite>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::sqlite::Sqlite>) -> diesel::serialize::Result {
        let s = serde_json::to_string(&self.0)?;
        out.set_value(s);
        Ok(IsNull::No)
    }
}

#[derive(Serialize, Queryable, Selectable, Debug, PartialEq, Insertable)]
#[diesel(table_name = news)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct News {
    id: i32,
    title: String,
    pub_date: NaiveDateTime,
    sources: Sources,
}

#[derive(Deserialize, Debug, PartialEq, Insertable)]
#[diesel(table_name = news)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewsInput {
    title: String,
    pub_date: NaiveDateTime,
    sources: Sources,
}

mod repository {
    use super::*;

    pub(super) async fn list(pool: &Pool, days_ago: u8) -> Result<Vec<News>, Box<dyn std::error::Error>> {

        use diesel::dsl::{sql, date};

        let conn = pool.get().await?;
        let res = conn
            .interact(move |c| {
                news::table.select(News::as_select())
                    .filter(date(news::pub_date).eq(sql(&format!("DATE('now', '-{} days', 'localtime')", days_ago))))
                    .order(news::pub_date.desc())
                    .load::<News>(c)
            })
            .await??;

        Ok(res)
    }

    pub(super) async fn create(pool: &Pool, input: NewsInput) -> Result<News, Box<dyn std::error::Error>> {

        let conn = pool.get().await?;
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


// get news list handler
pub async fn list(State(state): State<AppState>, days_ago: Option<Path<u8>>) -> Result<Json<Vec<News>>, StatusCode> {

    let days_ago: u8 = match days_ago {
        Some(Path(s)) => s,
        None => 0,
    };

    let pool = state.get_pool();
    match repository::list(pool, days_ago).await {
        Ok(news) => Ok(Json(news)),
        Err(e) => {
            tracing::error!("Error ocurred: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn publish(State(state): State<AppState>, Json(input): Json<NewsInput>) -> Result<Json<News>, StatusCode> {
    let pool = state.get_pool();
    match repository::create(pool, input).await {
        Ok(news) => Ok(Json(news)),
        Err(e) => {
            tracing::error!("Error ocurred: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}