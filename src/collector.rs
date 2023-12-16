use crate::models::{Channel, Item};
use anyhow::Result;
use chrono::DateTime;

pub fn fetch_channel(url: &String) -> Result<Channel> {
    let res = reqwest::blocking::get(url)?.text()?;
    let ch = rss::Channel::read_from(res.as_bytes())?;

    let last_build_date = DateTime::parse_from_rfc2822(ch.last_build_date().unwrap())?;

    let channel = Channel::new(
        ch.title(),
        &url,
        ch.language().unwrap(),
        last_build_date.naive_local(),
        ch.items()
            .iter()
            .map(|i| -> Result<Item> {
                let pub_date = DateTime::parse_from_rfc2822(i.pub_date().unwrap())?;

                let item = Item::new(
                    i.guid().unwrap().value(),
                    i.title().unwrap(),
                    i.link().unwrap(),
                    i.description().unwrap(),
                    pub_date.naive_local(),
                );

                Ok(item)
            })
            .collect::<Result<Vec<Item>>>()?,
    );

    Ok(channel)
}
