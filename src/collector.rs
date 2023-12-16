use crate::models::{Channel, Item};
use anyhow::Result;
use chrono::DateTime;
use crate::error::IsRequired;

pub fn fetch_channel(url: &String) -> Result<Channel> {
    let res = reqwest::blocking::get(url)?.text()?;
    let ch = rss::Channel::read_from(res.as_bytes())?;

    let last_build_date = DateTime::parse_from_rfc2822(ch.last_build_date().is_required()?)?;

    let channel = Channel::new(
        ch.title(),
        &url,
        ch.language().is_required()?,
        last_build_date.naive_local(),
        ch.items()
            .iter()
            .map(|i| -> Result<Item> {
                let pub_date = DateTime::parse_from_rfc2822(i.pub_date().is_required()?)?;

                let item = Item::new(
                    i.guid().is_required()?.value(),
                    i.title().is_required()?,
                    i.link().is_required()?,
                    i.description().is_required()?,
                    pub_date.naive_local(),
                );

                Ok(item)
            })
            .collect::<Result<Vec<Item>>>()?,
    );

    Ok(channel)
}
