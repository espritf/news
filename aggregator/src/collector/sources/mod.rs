pub mod html;
pub mod rss;

use crate::schema::channels;
use crate::schema::items;
use anyhow::Result;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
#[allow(clippy::large_enum_variant)]
pub enum Config {
    Rss(rss::Config),
    Html(html::Config),
}

#[derive(Debug, Insertable)]
pub struct Channel {
    pub title: String,
    pub link: String,
    pub language: String,
    pub last_build_date: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable)]
pub struct Item {
    pub guid: String,
    pub title: String,
    pub link: String,
    pub pub_date: NaiveDateTime,
    pub tags: Option<String>,
}

pub struct Data {
    pub channel: Channel,
    pub items: Vec<Item>,
}

pub fn fetch(config: &Config) -> Result<Data> {
    match config {
        Config::Rss(s) => rss::fetch(s),
        Config::Html(s) => html::fetch(s),
    }
}
