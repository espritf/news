use super::model::{News, NewsInput};
use crate::app::AppState;
use crate::news::security::auth;
use anyhow::Result;
use axum::async_trait;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::middleware;
use axum::routing::{get, post};
use axum::Json;
use axum::Router;
#[cfg(test)]
use mockall::automock;

pub fn routes(token: &String) -> Router<AppState> {
    Router::new()
        .route("/news", post(publish))
        .route_layer(middleware::from_fn_with_state(token.clone(), auth))
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
pub async fn list(
    State(state): State<AppState>,
    days_ago: Option<Path<u8>>,
) -> Result<Json<Vec<News>>, StatusCode> {
    tracing::info!("Listing news");

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

pub async fn publish(
    State(state): State<AppState>,
    Json(input): Json<NewsInput>,
) -> Result<Json<News>, StatusCode> {
    tracing::info!("Publishing news");

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
    use super::*;
    use axum::body::Body;
    use axum::http::{header, Method, Request};
    use chrono::NaiveDateTime;
    use serde_json::json;
    use std::str::FromStr;
    use std::sync::Arc;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_create_unauth() {
        let mut repo = MockNewsRepository::new();
        repo.expect_create().never();

        let repo = Arc::new(repo);
        let state = AppState { repo };
        let token = "test".to_string();

        let app = routes(&token).with_state(state);
        let request = Request::builder()
            .method(Method::POST)
            .uri("/news")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_create_auth() {
        let mut repo = MockNewsRepository::new();
        repo.expect_create().return_once(|_| {
            Ok(News::new(
                1,
                "title".to_string(),
                NaiveDateTime::from_str("2024-01-01T18:00:00").unwrap(),
                Vec::new(),
            ))
        });

        let repo = Arc::new(repo);
        let state = AppState { repo };
        let token = "test".to_string();

        let app = routes(&token).with_state(state);
        let request: Request<String> = Request::builder()
            .method(Method::POST)
            .uri("/news")
            .header("auth", token)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(
                json!({
                    "title": "title",
                    "pub_date": "2024-01-01T18:00:00",
                    "sources": ["test"]
                })
                .to_string()
                .into(),
            )
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
