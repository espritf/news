use super::{Channel, Data, Item};
use crate::error::IsRequired;
use anyhow::Result;
use chrono::DateTime;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    url: String,
}

pub fn fetch(config: &Config) -> Result<Data> {
    tracing::info!("Fetch data from {}", &config.url);

    let res = reqwest::blocking::get(&config.url)?.text()?;
    let ch = rss::Channel::read_from(res.as_bytes())?;

    let last_build_date = DateTime::parse_from_rfc2822(ch.last_build_date().required("channel last_build_date")?)?;

    let channel = Channel {
        title: ch.title().to_owned(),
        link: config.url.as_str().to_owned(),
        language: ch.language().required("channel language")?.to_owned(),
        last_build_date: Some(last_build_date.naive_local()),
    };

    let items = ch
        .items()
        .iter()
        .map(|i| {
            let pub_date = DateTime::parse_from_rfc2822(i.pub_date().required("item pub_date")?)?;
            let tags: Vec<&str> = i.categories().iter().map(|c| c.name()).collect();

            let item = Item {
                guid: i.guid().required("item guid")?.value().to_owned(),
                title: i.title().required("item title")?.to_owned(),
                link: i.link().required("item link")?.to_owned(),
                pub_date: pub_date.naive_local().to_owned(),
                tags: Some(serde_json::to_string(&tags)?),
            };

            Ok(item)
        })
        .collect::<Result<Vec<Item>>>()?;

    Ok(Data { channel, items })
}
