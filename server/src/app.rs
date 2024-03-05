use anyhow::Result;
use deadpool_diesel::sqlite::{Manager, Pool, Runtime};
use crate::news::model::{NewsRepository};

#[derive(Clone)]
pub struct AppState {
    pool: Pool,
}

impl AppState {
    pub fn build(uri: &String) -> Result<Self> {
        let manager = Manager::new(uri, Runtime::Tokio1);
        let pool = Pool::builder(manager).build()?;

        Ok(AppState { pool })
    }

    pub fn get_repo<T: NewsRepository>(&self) -> T {
        T::new(self.pool.clone())
    }
}
