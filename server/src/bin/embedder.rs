use anyhow::Result;
use news::embedder::repository::EmbeddingJobRepositoryImpl;
use news::embedder::worker::EmbeddingWorker;
use news::pool::Pool;
use news::transfomer::ollama::Model;
use std::env;
use std::sync::Arc;

/// Background worker that chunks and embeds published news, see `news::embedder::worker`.
#[tokio::main]
async fn main() -> Result<()> {
    dotenv_flow::dotenv_flow().ok();

    tracing_subscriber::fmt::init();

    let uri = &env::var("DATABASE_URL")?;
    let jobs = Arc::new(EmbeddingJobRepositoryImpl::new(Pool::new(uri)?));

    let model = &env::var("EMBEDDING_MODEL")?;
    let ollama_url = &env::var("OLLAMA_URL")?;
    let model = Arc::new(Model::new(model, ollama_url));

    let max_chunk_chars = env::var("MAX_CHUNK_CHARS")?.parse()?;

    tracing::info!("embedding worker started");

    EmbeddingWorker::new(jobs, model, max_chunk_chars).run().await;

    Ok(())
}
