pub mod rss;
pub mod html;

use serde::Deserialize;
use anyhow::Result;
use crate::schema::channels;
use crate::schema::items;
use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Config {
    Rss(rss::Config),
    Html(html::Config),
}

#[derive(Debug, Insertable)]
pub struct Channel {
    pub title: String,
    pub link: String,
    pub language: String,
    pub last_build_date: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
pub struct Item {
    pub guid: String,
    pub title: String,
    pub link: String,
    pub description: String,
    pub pub_date: NaiveDateTime,
    pub tags: String,
}

pub struct Data {
    pub channel: Channel,
    pub items: Vec<Item>
}

pub fn fetch(config: &Config) -> Result<Data> {
    match config {
        Config::Rss(s) => rss::fetch(s),
        Config::Html(s) => html::fetch(s),
    }
}
