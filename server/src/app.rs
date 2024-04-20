use std::sync::Arc;
use crate::news::handlers::NewsRepository;
use anyhow::{Result, Error};
use deadpool_diesel::sqlite::{Manager, Pool as DeadPool, Runtime};
use deadpool::managed::Object;

pub struct Pool {
    pub pool: DeadPool,
}

impl Pool {
    pub fn new(uri: &String) -> Result<Self> {
        let manager = Manager::new(uri, Runtime::Tokio1);
        let pool = DeadPool::builder(manager).build()?;
        Ok(Self { pool })
    }
    
    pub async fn get(&self) -> Result<Object<Manager>> {
        self.pool.get().await.map_err(Error::msg)
    }
}

#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<dyn NewsRepository>,
}