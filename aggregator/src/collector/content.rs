use anyhow::{anyhow, Result};
use readability::{extract, ExtractOptions};
use url::Url;

// Fetch the article page at `link` and extract its main body text.
pub fn fetch(link: &str) -> Result<String> {
    let url = Url::parse(link)?;
    let html = reqwest::blocking::get(link)?.text()?;
    let readable = extract(&mut html.as_bytes(), &url, ExtractOptions::default())?;

    if readable.text.trim().is_empty() {
        return Err(anyhow!("no content extracted from {}", link));
    }

    Ok(readable.text)
}
