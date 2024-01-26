use std::env;
use dotenvy::dotenv;
use anyhow::Result;
use axum::{
    routing::get,
    Router,
};
use tower_http::trace::TraceLayer;
use news::news::news_list;
use news::app::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let uri = &env::var("DATABASE_URL")?;
    let address = &env::var("SERVER_ADDR")?;

    let state = AppState::build(uri)?;

    let app = Router::new()
        .route("/", get(news_list))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(address).await?;
    tracing::info!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
