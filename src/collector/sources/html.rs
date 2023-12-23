use super::{Data, Item, Channel};
use anyhow::Result;
use chrono::NaiveDateTime;
use scraper::ElementRef;
use scraper::Html;
use scraper::Selector as S;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Getter {
    Text,
    Href,
}

#[derive(Deserialize, Debug)]
pub struct TextExtractor {
    sel: String,
    get: Getter,
}

#[derive(Deserialize, Debug)]
pub struct DateExtractor {
    sel: String,
    format: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    url: String,
    name: String,
    language: String,
    items: String,
    title: TextExtractor,
    guid: TextExtractor,
    link: TextExtractor,
    pub_date: DateExtractor,
}

impl TextExtractor {
    fn extract<'a>(&self, el: ElementRef<'a>) -> &'a str {
        let el = el.select(&S::parse(&self.sel).unwrap()).next().unwrap();

        match self.get {
            Getter::Text => el.text().next().unwrap().trim(),
            Getter::Href => el.attr("href").unwrap(),
        }
    }
}

impl DateExtractor {
    fn extract<'a>(&self, el: ElementRef<'a>) -> NaiveDateTime {
        let el = el.select(&S::parse(&self.sel).unwrap()).next().unwrap();
        let text = el.text().next().unwrap().trim();

        NaiveDateTime::parse_from_str(text, &self.format).unwrap()
    }
}

pub fn fetch(config: &Config) -> Result<Data> {
    println!("Fetch data from {}", &config.url);

    let res = reqwest::blocking::get(&config.url)?.text()?;
    let document = Html::parse_document(&res);

    let channel = Channel {
        title: config.name.to_owned(),
        link: config.url.to_owned(),
        language: config.language.to_owned(),
        last_build_date: None,
    };

    let items = document
        .select(&S::parse(&config.items).unwrap())
        .map(|el| Item {
            guid: config.guid.extract(el).to_owned(),
            title: config.title.extract(el).to_owned(),
            link: config.link.extract(el).to_owned(),
            pub_date: config.pub_date.extract(el),
            tags: None,
        })
        .collect();

    Ok(Data { channel, items })
}
