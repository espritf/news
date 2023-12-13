use anyhow::Result;
use chrono::DateTime;
use crate::models::NewChannel;

pub fn fetch_channel(url: &String) -> Result<NewChannel> {
    let res = reqwest::blocking::get(url)?.text()?;
    let channel = rss::Channel::read_from(res.as_bytes())?;
    let last_build_date =
        DateTime::parse_from_rfc2822(channel.last_build_date().unwrap())?.naive_local();

    Ok(NewChannel {
        title: channel.title,
        link: url.clone(),
        language: channel.language.unwrap(),
        last_build_date,
    })
}

