use super::model::{News, NewsInput, NewsData, QueryParams};
use crate::app::AppState;
use crate::news::security::auth;
use anyhow::Result;
use axum::async_trait;
use axum::extract::State;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::middleware;
use axum::routing::{get, post};
use axum::Json;
use axum::Router;
use pgvector::Vector;
#[cfg(test)]
use mockall::automock;

pub fn routes(token: &str) -> Router<AppState> {
    Router::new()
        .route("/news", post(publish))
        .route_layer(middleware::from_fn_with_state(token.to_owned(), auth))
        .route("/news", get(list))
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait NewsRepository: Send + Sync {
    async fn list(&self, params: QueryParams) -> Result<Vec<News>, Box<dyn std::error::Error>>;
    async fn create(&self, input: NewsData) -> Result<News, Box<dyn std::error::Error>>;
}

// get news list handler
pub async fn list(
    State(state): State<AppState>,
    params: Option<Query<QueryParams>>,
) -> Result<Json<Vec<News>>, StatusCode> {
    tracing::info!("Listing news");

    let Query(params) = params.unwrap_or_default();

    match state.repo.list(params).await {
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
    
    let title = input.get_title().to_owned();
    let embeddings = tokio::spawn(async move {
        state.model.forward(&title)
    }).await.unwrap().unwrap();

    let (_, n_tokens, _) = embeddings.dims3().unwrap();
    let embeddings = (embeddings.sum(1).unwrap() / (n_tokens as f64)).unwrap();
    
    tracing::info!("pooled embeddings shape: {:?}", embeddings.shape());

    let v = Vector::from(embeddings.get(0).unwrap().to_vec1::<f32>().unwrap());
    let data = NewsData::new(&input, v);

    match state.repo.create(data).await {
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
