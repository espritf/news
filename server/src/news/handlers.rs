use super::model::{ChunkInput, News, NewsInput, NewsData, QueryParams};
use crate::app::{AppState, NewsRepository, VectorProvider};
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
use std::sync::Arc;
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::{Modify, OpenApi};

pub fn routes(token: &str) -> Router<AppState> {
    Router::new()
        .route("/news", post(publish))
        .route_layer(middleware::from_fn_with_state(token.to_owned(), auth))
        .route("/news", get(list))
}

#[derive(OpenApi)]
#[openapi(
    paths(list, publish),
    components(schemas(News, NewsInput)),
    tags((name = "news", description = "News publishing and listing")),
    modifiers(&SecurityAddon),
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().expect("components exist");
        components.add_security_scheme(
            "api_token",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("auth"))),
        );
    }
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

/// List news
///
/// Returns a paginated listing of news items ordered by publish date, or by semantic
/// similarity to `search` (matched against chunked content embeddings) when that query
/// param is provided.
#[utoipa::path(
    get,
    path = "/news",
    tag = "news",
    params(QueryParams),
    responses(
        (status = 200, description = "News items", body = [News]),
        (status = 500, description = "Internal server error"),
    ),
)]
pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<QueryParams>,
) -> Result<Json<Vec<News>>, AppError> {
    tracing::info!("Listing news");

    tracing::debug!("Query params: {:?}", params);

    let search = match params.search {
        Some(s) => Some(state.model.vector(&s).await?),
        None => None,
    };

    let params = ListParams {
        limit: params.limit.unwrap_or(100),
        search,
    };

    let news = state.repo.list(params).await?;
    Ok(Json(news))
}

/// Publish news
///
/// Stores the news item and returns immediately; the content is chunked and embedded via the
/// configured Ollama model in the background, so search results for a just-published item may
/// lag briefly behind its availability in `GET /news`. Requires the `auth` header to match the
/// configured API token.
#[utoipa::path(
    post,
    path = "/news",
    tag = "news",
    request_body = NewsInput,
    responses(
        (status = 200, description = "Created news item", body = News),
        (status = 401, description = "Missing or invalid auth header"),
        (status = 500, description = "Internal server error"),
    ),
    security(("api_token" = [])),
)]
pub async fn publish(
    State(state): State<AppState>,
    Json(input): Json<NewsInput>,
) -> Result<Json<News>, AppError> {
    tracing::info!("Publishing news");

    let chunks = input.search_chunks(state.max_chunk_chars);
    let data = NewsData::new(&input);

    let news = state.repo.create(data).await?;

    let news_id = news.id();
    let model = state.model.clone();
    let repo = state.repo.clone();

    tokio::spawn(async move {
        if let Err(e) = embed_chunks(model, repo, news_id, chunks).await {
            tracing::error!("Failed to embed chunks for news {}: {:?}", news_id, e);
        }
    });

    Ok(Json(news))
}

/// Embeds each chunk of a just-published news item and stores it, run out-of-band from the
/// `publish` request so slow embedding calls don't hold up the HTTP response.
async fn embed_chunks(
    model: Arc<dyn VectorProvider>,
    repo: Arc<dyn NewsRepository>,
    news_id: i32,
    texts: Vec<String>,
) -> Result<()> {
    let mut chunks = Vec::new();
    for (i, text) in texts.into_iter().enumerate() {
        let chunk_v = model.vector(&text).await?;
        chunks.push(ChunkInput {
            chunk_index: i as i32,
            chunk_text: text,
            chunk_v,
        });
    }

    repo.insert_chunks(news_id, chunks).await
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
        let state = AppState {
            repo,
            model,
            max_chunk_chars: 1024,
        };
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
                "content".to_string(),
            ))
        });
        repo.expect_insert_chunks().returning(|_, _| Ok(()));

        let mut vp = MockVectorProvider::new();
        vp.expect_vector().returning(|_| {
            Ok(pgvector::Vector::from(vec![1.0, 2.0, 3.0]))
        });

        let repo = Arc::new(repo);
        let model = Arc::new(vp);
        let state = AppState {
            repo,
            model,
            max_chunk_chars: 1024,
        };
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
                    "sources": ["test"],
                    "content": "content"
                })
                .to_string()
                .into(),
            )
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
