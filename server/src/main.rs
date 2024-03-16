pub mod schema;
pub mod app;
pub mod news;

use std::env;
use anyhow::Result;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
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

    let app = news::handlers::routes()
        .layer(TraceLayer::new_for_http())
        .with_state(state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(address).await?;
    tracing::info!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
