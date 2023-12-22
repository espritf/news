use serde::Deserialize;
use super::{Result, Data};

#[derive(Deserialize, Debug)]
pub struct Config {
    url: String,
    items: String,
    title: String,
    guid: String,
    link: String,
    pub_date: String,
}

pub fn fetch(_config: &Config) -> Result<Data> {
    todo!()
}
