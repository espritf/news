use crate::schema::news;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use pgvector::Vector;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[allow(dead_code)]
type Backend = diesel::pg::Pg;

#[derive(Serialize, Queryable, Selectable, Debug, PartialEq, Insertable, ToSchema)]
#[diesel(table_name = news)]
#[diesel(check_for_backend(Backend))]
pub struct News {
    id: i32,
    title: String,
    pub_date: NaiveDateTime,
    #[schema(value_type = Vec<String>)]
    sources: serde_json::Value,
    content: String,
}

impl News {
    pub fn new(
        id: i32,
        title: String,
        pub_date: NaiveDateTime,
        sources: Vec<String>,
        content: String,
    ) -> Self {
        Self {
            id,
            title,
            pub_date,
            sources: sources.into(),
            content,
        }
    }
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct NewsInput {
    pub title: String,
    pub_date: NaiveDateTime,
    #[schema(value_type = Vec<String>)]
    sources: serde_json::Value,
    content: String,
}

impl NewsInput {
    pub fn get_title(&self) -> &str {
        &self.title
    }
}

#[derive(Debug, PartialEq, Insertable)]
#[diesel(table_name = news)]
#[diesel(check_for_backend(Backend))]
pub struct NewsData {
    title: String,
    pub_date: NaiveDateTime,
    sources: serde_json::Value,
    title_v: Vector,
    content: String,
}

impl NewsData {
    pub fn new(input: &NewsInput, title_v: Vector) -> Self {
        Self {
            title: input.title.clone(),
            pub_date: input.pub_date,
            sources: input.sources.clone(),
            title_v,
            content: input.content.clone(),
        }
    }
}

#[derive(Deserialize, Debug, IntoParams)]
pub struct QueryParams {
    /// Maximum number of results to return (defaults to 100).
    pub limit: Option<u8>,
    /// Semantic search query; when present, results are ordered by embedding similarity.
    pub search: Option<String>,
}

pub struct ListParams {
    pub limit: u8,
    pub search: Option<Vector>,
}
