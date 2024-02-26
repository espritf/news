use super::{Data, Item, Channel};
use anyhow::Result;
use chrono::NaiveDateTime;
use scraper::ElementRef;
use scraper::Html;
use scraper::Selector as S;
use serde::Deserialize;
use url::Url;

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
pub struct LinkExtractor {
    sel: String,
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
    link: LinkExtractor,
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

impl LinkExtractor {
    fn extract(&self, el: ElementRef, base: &str) -> String {
        let el = el.select(&S::parse(&self.sel).unwrap()).next().unwrap();
        let href = el.attr("href").unwrap();
        
        // make sure the href is an absolute URL
        if href.starts_with("http") {
            href.to_owned()
        } else {
            let base = Url::parse(base).unwrap();
            base.join(href).unwrap().to_string()
        }
    }
}

impl DateExtractor {
    fn extract(&self, el: ElementRef) -> NaiveDateTime {
        let el = el.select(&S::parse(&self.sel).unwrap()).next().unwrap();
        let text = el.text().next().unwrap().trim();

        NaiveDateTime::parse_from_str(text, &self.format).unwrap()
    }
}

pub fn fetch(config: &Config) -> Result<Data> {
    tracing::info!("Fetch data from {}", &config.url);

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
            link: config.link.extract(el, &config.url).to_owned(),
            pub_date: config.pub_date.extract(el),
            tags: None,
        })
        .collect();

    Ok(Data { channel, items })
}
