use crate::app::{NewsRepository, VectorProvider};
use crate::news::model::{ListParams, News, QueryParams};
use anyhow::Result;
use std::sync::Arc;

/// Lists news items, embedding an optional `search` query into a vector so results can be
/// ordered by semantic similarity instead of publish date.
#[derive(Clone)]
pub struct NewsLister {
    repo: Arc<dyn NewsRepository>,
    model: Arc<dyn VectorProvider>,
}

impl NewsLister {
    pub fn new(repo: Arc<dyn NewsRepository>, model: Arc<dyn VectorProvider>) -> Self {
        Self { repo, model }
    }

    pub async fn list(&self, params: QueryParams) -> Result<Vec<News>> {
        let search = match params.search {
            Some(s) => Some(self.model.vector(&s).await?),
            None => None,
        };

        let params = ListParams {
            limit: params.limit.unwrap_or(100),
            search,
        };

        self.repo.list(params).await
    }
}
