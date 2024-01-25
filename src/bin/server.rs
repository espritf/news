use std::env;
use dotenvy::dotenv;
use anyhow::Result;
use axum::{
    routing::get,
    Router,
};
use news::news::news_list;
use news::app::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let uri = &env::var("DATABASE_URL")?;
    let address = &env::var("SERVER_ADDR")?;

    let state = AppState::build(uri)?;

    let app = Router::new()
        .route("/", get(news_list))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
