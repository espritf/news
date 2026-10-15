use diesel::prelude::*;
use diesel::sql_types::{Int4, Text};

/// A leased embedding job: the news item whose content must be chunked and embedded.
#[derive(QueryableByName, Debug, Clone, PartialEq)]
pub struct EmbeddingJob {
    #[diesel(sql_type = Int4)]
    pub news_id: i32,
    /// Number of times the job has been claimed, including the current attempt.
    #[diesel(sql_type = Int4)]
    pub attempts: i32,
    #[diesel(sql_type = Text)]
    pub content: String,
}
