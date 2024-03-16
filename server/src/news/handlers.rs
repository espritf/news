use crate::app::AppState;
use anyhow::Result;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use super::model::{News, NewsInput, NewsRepository};
use super::repository::NewsRepositoryImpl;
use axum::routing::{get, post};
use axum::Router;
use axum::middleware;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/news", post(publish)).route_layer(middleware::from_fn(super::security::auth))
        .route("/news", get(list))
        .route("/news/:days_ago", get(list))
}


// get news list handler
pub async fn list(State(state): State<AppState>, days_ago: Option<Path<u8>>) -> Result<Json<Vec<News>>, StatusCode> {

    let days_ago: u8 = match days_ago {
        Some(Path(s)) => s,
        None => 0,
    };

    let repo = state.get_repo::<NewsRepositoryImpl>();
    match repo.list(days_ago).await {
        Ok(news) => Ok(Json(news)),
        Err(e) => {
            tracing::error!("Error occurred: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn publish(State(state): State<AppState>, Json(input): Json<NewsInput>) -> Result<Json<News>, StatusCode> {
    let repo = state.get_repo::<NewsRepositoryImpl>();
    match repo.create(input).await {
        Ok(news) => Ok(Json(news)),
        Err(e) => {
            tracing::error!("Error occurred: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
