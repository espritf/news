use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub fn translate(source_lang: &str, target_lang: &str, text: &str) -> Result<String> {
    let translator = google::Translator::new(
        std::env::var("GOOGLE_TRANSLATE_API_KEY")?,
        source_lang.to_owned(),
        target_lang.to_owned(),
    );

    translator.translate(text)
}

mod google {
    use super::*;

    pub struct Translator {
        client: reqwest::blocking::Client,
        api_key: String,
        source_lang: String,
        target_lang: String,
    }

    #[derive(Serialize)]
    struct Request {
        q: String,
        source: String,
        target: String,
        format: String,
    }

    #[derive(Deserialize)]
    struct Response {
        data: Data,
    }

    #[derive(Deserialize)]
    struct Data {
        translations: Vec<Translation>,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Translation {
        translated_text: String,
    }

    impl Translator {
        pub fn new(api_key: String, source_lang: String, target_lang: String) -> Self {
            let client = reqwest::blocking::Client::builder()
                .timeout(Some(Duration::new(240, 0)))
                .build()
                .unwrap();
            Self {
                client,
                api_key,
                source_lang,
                target_lang,
            }
        }

        pub fn translate(&self, text: &str) -> Result<String> {
            let url = "https://translation.googleapis.com/language/translate/v2";
            let req = Request {
                q: text.to_owned(),
                source: self.source_lang.clone(),
                target: self.target_lang.clone(),
                format: "text".to_owned(),
            };
            let res = self.client
                .post(url)
                .json(&req)
                .query(&[("key", self.api_key.clone())])
                .send()?
                .json::<Response>()?;

            Ok(res.data.translations[0].translated_text.clone())
        }
    }
}
