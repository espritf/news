use crate::schema::news;
use chrono::NaiveDateTime;
use diesel::prelude::*;
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

#[derive(Deserialize, Debug, PartialEq, Insertable)]
#[diesel(table_name = news)]
#[diesel(check_for_backend(Backend))]
pub struct NewsInput {
    title: String,
    pub_date: NaiveDateTime,
    sources: serde_json::Value,
}
