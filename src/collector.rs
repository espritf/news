use crate::models::input::Channel;
use anyhow::Result;
use chrono::DateTime;

impl Channel {
    fn from(url: &String, ch: rss::Channel) -> Result<Self> {
        let last_build_date =
            DateTime::parse_from_rfc2822(ch.last_build_date().unwrap())?.naive_local();

        Ok(Self {
            title: ch.title,
            link: url.clone(),
            language: ch.language.unwrap(),
            last_build_date,
        })
    }
}

pub fn fetch_channel(url: &String) -> Result<Channel> {
    let res = reqwest::blocking::get(url)?.text()?;
    let channel = rss::Channel::read_from(res.as_bytes())?;

    Channel::from(url, channel)
}
