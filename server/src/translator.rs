use std::time::Duration;

use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
struct Response {
    response: String,
}

pub fn translate(text: &str) -> Result<String> {
    tracing::info!("Translate text: {}", text);

    let json = &serde_json::json!({
        "model": "translator",
        "stream": false,
        "prompt": text,
    });

    let res = reqwest::blocking::Client::builder()
        .timeout(Some(Duration::new(240, 0)))
        .build()?
        .post("http://localhost:11434/api/generate")
        .json(json)
        .send()?
        .json::<Response>()?;

    Ok(res.response.trim().to_owned())
}
