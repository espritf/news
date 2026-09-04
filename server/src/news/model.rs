use crate::schema::{news, news_chunks};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use pgvector::Vector;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[allow(dead_code)]
type Backend = diesel::pg::Pg;

/// Splits `text` into word-bounded chunks no longer than `max_chars`, so each chunk stays
/// within the embedding model's context window.
pub fn chunk_text(text: &str, max_chars: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        let extra = if current.is_empty() { 0 } else { 1 };
        if !current.is_empty() && current.len() + extra + word.len() > max_chars {
            chunks.push(std::mem::take(&mut current));
        }
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(word);
    }
    if !current.is_empty() {
        chunks.push(current);
    }

    chunks
}

#[derive(Serialize, Queryable, Selectable, Debug, PartialEq, Insertable, ToSchema)]
#[diesel(table_name = news)]
#[diesel(check_for_backend(Backend))]
pub struct News {
    id: i32,
    title: String,
    pub_date: NaiveDateTime,
    #[schema(value_type = Vec<String>)]
    sources: serde_json::Value,
    content: String,
}

impl News {
    pub fn new(
        id: i32,
        title: String,
        pub_date: NaiveDateTime,
        sources: Vec<String>,
        content: String,
    ) -> Self {
        Self {
            id,
            title,
            pub_date,
            sources: sources.into(),
            content,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct NewsInput {
    pub title: String,
    pub_date: NaiveDateTime,
    #[schema(value_type = Vec<String>)]
    sources: serde_json::Value,
    content: String,
}

impl NewsInput {
    pub fn get_content(&self) -> &str {
        &self.content
    }

    /// Text used for content-based semantic search: the article content, chunked by
    /// `chunk_text` before embedding so long articles don't exceed the model's context window.
    pub fn search_chunks(&self, max_chars: usize) -> Vec<String> {
        chunk_text(&self.content, max_chars)
    }
}

#[derive(Debug, PartialEq, Insertable)]
#[diesel(table_name = news)]
#[diesel(check_for_backend(Backend))]
pub struct NewsData {
    title: String,
    pub_date: NaiveDateTime,
    sources: serde_json::Value,
    content: String,
}

impl NewsData {
    pub fn new(input: &NewsInput) -> Self {
        Self {
            title: input.title.clone(),
            pub_date: input.pub_date,
            sources: input.sources.clone(),
            content: input.content.clone(),
        }
    }
}

/// A chunk of searchable text paired with its embedding, ready to be persisted once the
/// parent `News` row (and its id) exists.
pub struct ChunkInput {
    pub chunk_index: i32,
    pub chunk_text: String,
    pub chunk_v: Vector,
}

#[derive(Debug, PartialEq, Insertable)]
#[diesel(table_name = news_chunks)]
#[diesel(check_for_backend(Backend))]
pub struct NewsChunkData {
    news_id: i32,
    chunk_index: i32,
    chunk_text: String,
    chunk_v: Vector,
}

impl NewsChunkData {
    pub fn new(news_id: i32, chunk: ChunkInput) -> Self {
        Self {
            news_id,
            chunk_index: chunk.chunk_index,
            chunk_text: chunk.chunk_text,
            chunk_v: chunk.chunk_v,
        }
    }
}

#[derive(Deserialize, Debug, IntoParams)]
pub struct QueryParams {
    /// Maximum number of results to return (defaults to 100).
    pub limit: Option<u8>,
    /// Semantic search query; when present, results are ordered by embedding similarity.
    pub search: Option<String>,
}

pub struct ListParams {
    pub limit: u8,
    pub search: Option<Vector>,
}
