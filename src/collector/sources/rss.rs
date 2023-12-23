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
    println!("Fetch data from {}", &config.url);

    let res = reqwest::blocking::get(&config.url)?.text()?;
    let ch = rss::Channel::read_from(res.as_bytes())?;

    let last_build_date = DateTime::parse_from_rfc2822(ch.last_build_date().is_required()?)?;

    let channel = Channel {
        title: ch.title().to_owned(),
        link: config.url.as_str().to_owned(),
        language: ch.language().is_required()?.to_owned(),
        last_build_date: Some(last_build_date.naive_local()),
    };

    let items = ch
        .items()
        .iter()
        .map(|i| {
            let pub_date = DateTime::parse_from_rfc2822(i.pub_date().is_required()?)?;
            let tags: Vec<&str> = i.categories().iter().map(|c| c.name()).collect();

            let item = Item {
                guid: i.guid().is_required()?.value().to_owned(),
                title: i.title().is_required()?.to_owned(),
                link: i.link().is_required()?.to_owned(),
                pub_date: pub_date.naive_local().to_owned(),
                tags: Some(serde_json::to_string(&tags)?),
            };

            Ok(item)
        })
        .collect::<Result<Vec<Item>>>()?;

    Ok(Data { channel, items })
}
