use super::{Channel, Data, Item};
use crate::error::IsRequired;
use anyhow::Result;
use chrono::DateTime;
use serde::Deserialize;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Deserialize, Debug)]
pub struct Config {
    url: String,
}

pub fn hash_id(id: String) -> String {
    let mut hasher = DefaultHasher::new();
    id.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

pub fn fetch(config: &Config) -> Result<Data> {
    tracing::info!("Fetch data from {}", &config.url);

    let res = reqwest::blocking::get(&config.url)?.text()?;
    let ch = rss::Channel::read_from(res.as_bytes())?;

    let last_build_date = ch.last_build_date()
        .and_then(|d| DateTime::parse_from_rfc2822(d).ok())
        .map(|d| d.naive_local());

    let channel = Channel {
        title: ch.title().to_owned(),
        link: config.url.as_str().to_owned(),
        language: ch.language().required("channel language")?.to_owned(),
        last_build_date,
    };

    let items = ch
        .items()
        .iter()
        .map(|i| {
            let pub_date = DateTime::parse_from_rfc2822(i.pub_date().required("item pub_date")?)?;
            let tags: Vec<&str> = i.categories().iter().map(|c| c.name()).collect();
            let link = i.link().required("item link")?.to_owned();

            let guid = match i.guid() {
                Some(g) => g.value().to_owned(),
                None => link.clone(),
            };

            let guid = hash_id(guid);

            tracing::debug!("Fetch item: {}", guid);

            let item = Item {
                guid,
                link,
                title: i.title().required("item title")?.to_owned(),
                pub_date: pub_date.naive_local().to_owned(),
                tags: Some(serde_json::to_string(&tags)?),
            };

            Ok(item)
        })
        .collect::<Result<Vec<Item>>>()?;

    Ok(Data { channel, items })
}
