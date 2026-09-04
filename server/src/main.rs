pub mod app;
pub mod schema;
pub mod pool;
pub mod transfomer;
pub mod news;

use crate::news::handlers::ApiDoc;
use crate::news::repository::NewsRepositoryImpl;
use anyhow::Result;
use app::AppState;
use pool::Pool;
use std::env;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use transfomer::ollama::Model;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv_flow::dotenv_flow().ok();

    tracing_subscriber::fmt::init();

    let uri = &env::var("DATABASE_URL")?;
    let address = &env::var("SERVER_ADDR")?;

    let pool = Pool::new(uri)?;
    let repo = Arc::new(NewsRepositoryImpl::new(pool));

    let model = &env::var("EMBEDDING_MODEL")?;
    let ollama_url = &env::var("OLLAMA_URL")?;
    let model = Arc::new(Model::new(model, ollama_url));

    let max_chunk_chars = env::var("MAX_CHUNK_CHARS").expect("Expected to be set").parse()?;

    let state = AppState {
        repo,
        model,
        max_chunk_chars,
    };

    let cors = CorsLayer::new().allow_origin(Any);

    let token = env::var("NEWS_API_TOKEN")?;

    let app = news::handlers::routes(&token)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(address).await?;
    tracing::info!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
