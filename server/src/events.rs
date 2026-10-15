use crate::app::EventSink;
use anyhow::Result;
use axum::async_trait;
use clickhouse::{Client, Row};
use serde::Serialize;

/// Outcome of one embedding attempt, stored in `embedding_events` (see `clickhouse/events.sql`).
#[derive(Row, Serialize, Debug, Clone, PartialEq)]
pub struct EmbeddingEvent {
    pub kind: String,
    pub news_id: i32,
    pub attempts: i32,
    pub duration_ms: u32,
    pub chunk_count: u32,
    pub content_chars: u32,
    pub error: Option<String>,
}

pub struct ClickHouseEvents {
    client: Client,
}

impl ClickHouseEvents {
    pub fn new(url: &str, database: &str) -> Self {
        Self {
            client: Client::default().with_url(url).with_database(database),
        }
    }
}

#[async_trait]
impl EventSink for ClickHouseEvents {
    async fn embedding(&self, event: EmbeddingEvent) -> Result<()> {
        let mut insert = self.client.insert::<EmbeddingEvent>("embedding_events").await?;
        insert.write(&event).await?;
        insert.end().await?;
        Ok(())
    }
}
