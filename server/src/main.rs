pub mod app;
pub mod schema;
pub mod pool;
pub mod transfomer;
pub mod news;

use crate::news::repository::NewsRepositoryImpl;
use anyhow::Result;
use app::AppState;
use pool::Pool;
use std::env;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use transfomer::Model;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv_flow::dotenv_flow().ok();

    tracing_subscriber::fmt::init();

    let uri = &env::var("DATABASE_URL")?;
    let address = &env::var("SERVER_ADDR")?;

    let pool = Pool::new(uri)?;
    let repo = Arc::new(NewsRepositoryImpl::new(pool));

    let config = env::var("MODEL_CONFIG").unwrap_or("MODEL_CONFIG not set".to_string());
    let tokenizer = env::var("MODELTOKENIZER").unwrap_or("MODELTOKENIZER not set".to_string());
    let weights = env::var("MODEL_WEIGHTS").unwrap_or("MODEL_WEIGHTS not set".to_string());
    let model = Arc::new(Model::build(&config, &tokenizer, &weights)?);

    let state = AppState { repo, model };

    let cors = CorsLayer::new().allow_origin(Any);

    let token = env::var("NEWS_API_TOKEN")?;

    let app = news::handlers::routes(&token)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(address).await?;
    tracing::info!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
