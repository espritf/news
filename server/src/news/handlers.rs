use crate::app::AppState;
use anyhow::Result;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use super::model::{News, NewsInput};
use axum::routing::{get, post};
use axum::Router;
use axum::middleware;
use axum::async_trait;

#[cfg(test)]
use mockall::automock;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/news", post(publish)).route_layer(middleware::from_fn(super::security::auth))
        .route("/news", get(list))
        .route("/news/:days_ago", get(list))
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait NewsRepository: Send + Sync {
    async fn list(&self, days_ago: u8) -> Result<Vec<News>, Box<dyn std::error::Error>>;
    async fn create(&self, input: NewsInput) -> Result<News, Box<dyn std::error::Error>>;
}

// get news list handler
pub async fn list(State(state): State<AppState>, days_ago: Option<Path<u8>>) -> Result<Json<Vec<News>>, StatusCode> {
    let days_ago: u8 = match days_ago {
        Some(Path(s)) => s,
        None => 0,
    };

    match state.repo.list(days_ago).await {
        Ok(news) => Ok(Json(news)),
        Err(e) => {
            tracing::error!("Error occurred: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn publish(State(state): State<AppState>, Json(input): Json<NewsInput>) -> Result<Json<News>, StatusCode> {
    match state.repo.create(input).await {
        Ok(news) => Ok(Json(news)),
        Err(e) => {
            tracing::error!("Error occurred: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::sync::Arc;
    use axum::body::Body;
    use axum::http::{Request, Method};
    use chrono::NaiveDateTime;
    use tower::ServiceExt;
    use super::*;

    #[ignore]
    #[tokio::test]
    async fn test_create_unauth() {
        let mut repo = MockNewsRepository::new();
        repo.expect_create()
            .return_once(|_| {
                Ok(News::new(1, "title".to_string(), NaiveDateTime::from_str("2024-01-01").unwrap(), Vec::new()))
            });

        let repo = Arc::new(repo);
        let state = AppState { repo };

        let app = routes().with_state(state);
        let request = Request::builder()
            .method(Method::POST)
            .uri("/news")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
