pub mod app;
pub mod news;
pub mod schema;

use crate::news::repository::NewsRepositoryImpl;
use anyhow::Result;
use app::{AppState, Pool};
use std::env;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv_flow::dotenv_flow().ok();

    tracing_subscriber::fmt::init();

    let uri = &env::var("DATABASE_URL")?;
    let address = &env::var("SERVER_ADDR")?;

    let pool = Pool::new(uri)?;
    let repo = Arc::new(NewsRepositoryImpl::new(pool));
    let token = env::var("NEWS_API_TOKEN")?;
    let state = AppState { repo };

    let cors = CorsLayer::new().allow_origin(Any);

    let app = news::handlers::routes(&token)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(address).await?;
    tracing::info!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
