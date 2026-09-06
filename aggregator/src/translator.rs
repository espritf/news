use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub fn translate(text: &str) -> Result<String> {
    ollama::Translator::new().translate(text)
}

mod ollama {
    use super::*;

    pub struct Translator {
        client: reqwest::blocking::Client,
        endpoint: String,
        model: String,
    }

    #[derive(Serialize)]
    struct Request {
        model: String,
        prompt: String,
        stream: bool,
    }

    #[derive(Deserialize)]
    struct Response {
        response: String,
    }

    impl Translator {
        pub fn new() -> Self {
            let client = reqwest::blocking::Client::builder()
                .timeout(Some(Duration::new(600, 0)))
                .build()
                .unwrap();
            let endpoint = std::env::var("OLLAMA_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:11433/api/generate".to_owned());
            let model = std::env::var("OLLAMA_TRANSLATOR_MODEL")
                .unwrap_or_else(|_| "translator".to_owned());
            Self { client, endpoint, model }
        }

        #[tracing::instrument(skip(self, text), fields(text_len = text.len()))]
        pub fn translate(&self, text: &str) -> Result<String> {
            let req = Request {
                model: self.model.clone(),
                prompt: text.to_owned(),
                stream: false,
            };

            let res = self.client.post(&self.endpoint).json(&req).send()?;

            if !res.status().is_success() {
                return Err(anyhow::anyhow!("Failed to translate. Server responded with {}", res.status()));
            }

            let res = res.json::<Response>()?;

            Ok(res.response.trim().to_owned())
        }
    }
}
