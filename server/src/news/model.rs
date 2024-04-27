use crate::schema::news;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use pgvector::Vector;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
type Backend = diesel::pg::Pg;

#[derive(Serialize, Queryable, Selectable, Debug, PartialEq, Insertable)]
#[diesel(table_name = news)]
#[diesel(check_for_backend(Backend))]
pub struct News {
    id: i32,
    title: String,
    pub_date: NaiveDateTime,
    sources: serde_json::Value,
}

impl News {
    pub fn new(id: i32, title: String, pub_date: NaiveDateTime, sources: Vec<String>) -> Self {
        Self {
            id,
            title,
            pub_date,
            sources: sources.into(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct NewsInput {
    pub title: String,
    pub_date: NaiveDateTime,
    sources: serde_json::Value,
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
}

impl NewsData {
    pub fn new(input: &NewsInput, title_v: Vector) -> Self {
        Self {
            title: input.title.clone(),
            pub_date: input.pub_date.clone(),
            sources: input.sources.clone(),
            title_v,
        }
    }
}
