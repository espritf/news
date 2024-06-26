use super::model::{News, NewsInput, NewsData, QueryParams};
use crate::app::AppState;
use crate::news::model::ListParams;
use crate::news::security::auth;
use anyhow::Result;
use axum::extract::State;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::middleware;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::{get, post};
use axum::Json;
use axum::Router;

pub fn routes(token: &str) -> Router<AppState> {
    Router::new()
        .route("/news", post(publish))
        .route_layer(middleware::from_fn_with_state(token.to_owned(), auth))
        .route("/news", get(list))
}

pub struct AppError(anyhow::Error);

// tell axum how to convert our error type into a Response
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("Error occurred: {}", self.0);
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
    }
}

// enable the use of ? to simplify error handling
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

// get news list handler
pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<QueryParams>,
) -> Result<Json<Vec<News>>, AppError> {
    tracing::info!("Listing news");

    tracing::debug!("Query params: {:?}", params);

    let search = match params.search {
        Some(s) => Some(state.model.vector(&s).await.unwrap()),
        None => None,
    };

    let params = ListParams {
        limit: params.limit.unwrap_or(100),
        search,
    };

    let news = state.repo.list(params).await?;
    Ok(Json(news))
}

pub async fn publish(
    State(state): State<AppState>,
    Json(input): Json<NewsInput>,
) -> Result<Json<News>, AppError> {
    tracing::info!("Publishing news");
    
    let title = input.get_title().to_owned();
    let v = state.model.vector(&title).await.unwrap();
    let data = NewsData::new(&input, v);

    let news = state.repo.create(data).await?;
    Ok(Json(news))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::MockNewsRepository;
    use crate::app::MockVectorProvider;
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

        let mut vp = MockVectorProvider::new();
        vp.expect_vector().never();

        let repo = Arc::new(repo);
        let model = Arc::new(vp);
        let state = AppState { repo, model };
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

        let mut vp = MockVectorProvider::new();
        vp.expect_vector().return_once(|_| {
            Ok(pgvector::Vector::from(vec![1.0, 2.0, 3.0]))
        });

        let repo = Arc::new(repo);
        let model = Arc::new(vp);
        let state = AppState { repo, model };
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
