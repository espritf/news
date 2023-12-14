use anyhow::Result;
use chrono::DateTime;
use crate::models::input::Channel;

pub fn fetch_channel(url: &String) -> Result<Channel> {
    let res = reqwest::blocking::get(url)?.text()?;
    let channel = rss::Channel::read_from(res.as_bytes())?;
    let last_build_date =
        DateTime::parse_from_rfc2822(channel.last_build_date().unwrap())?.naive_local();

    Ok(Channel {
        title: channel.title,
        link: url.clone(),
        language: channel.language.unwrap(),
        last_build_date,
    })
}

