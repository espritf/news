pub mod schema;
pub mod app;
pub mod security;
pub mod news;

use std::env;
use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
    middleware,
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use news::handlers::{list, publish};
use app::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv_flow::dotenv_flow().ok();

    tracing_subscriber::fmt::init();

    let uri = &env::var("DATABASE_URL")?;
    let address = &env::var("SERVER_ADDR")?;

    let state = AppState::build(uri)?;

    let cors = CorsLayer::new()
        .allow_origin(Any);

    let app = Router::new()
        .route("/news", post(publish)).route_layer(middleware::from_fn(security::auth))
        .route("/news", get(list))
        .route("/news/:days_ago", get(list))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(address).await?;
    tracing::info!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}