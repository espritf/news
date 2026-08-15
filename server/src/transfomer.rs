use crate::app::VectorProvider;
use anyhow::Result;
use axum::async_trait;
use pgvector::Vector;

pub mod ollama {

    use super::*;
    use serde::Deserialize;
    use std::time::Duration;

    pub struct Model {
        name: String,
    }

    impl Model {
        pub fn new(name: &str) -> Self {
            Self {
                name: name.to_owned(),
            }
        }
    }

    #[derive(Deserialize)]
    struct Response {
        embedding: Vec<f32>,
    }

    #[async_trait]
    impl VectorProvider for Model {
        async fn vector(&self, input: &str) -> Result<Vector> {
            let json = &serde_json::json!({
                "model": self.name,
                "prompt": input,
            });

            let response = reqwest::Client::builder()
                .timeout(Duration::new(240, 0))
                .build()?
                .post("http://localhost:11434/api/embeddings")
                .json(json)
                .send()
                .await?;

            if ! response.status().is_success() {
                let status = response.status();
                let body = response.text().await?;

                tracing::error!("Request failed with status {:?}: {}", status, body);
                return Err(anyhow::anyhow!("Request failed with status {}: {}", status, body));
            }
            
            let res = response.json::<Response>().await?;

            tracing::info!("embeddings size: {:?}", res.embedding.len());

            let v = Vector::from(res.embedding);
            Ok(v)
        }
    }
}
