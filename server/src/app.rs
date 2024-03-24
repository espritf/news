use std::sync::Arc;
use crate::news::handlers::NewsRepository;
use anyhow::Result;
use deadpool_diesel::sqlite::{Manager, Pool, Runtime};

pub fn pool(uri: &String) -> Result<Pool> {
    let manager = Manager::new(uri, Runtime::Tokio1);
    let pool = Pool::builder(manager).build()?;
    Ok(pool)
}

#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<dyn NewsRepository>,
}