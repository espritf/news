use crate::news::handlers::NewsRepository;
use anyhow::Result;
use deadpool_diesel::sqlite::{Manager, Pool, Runtime};

fn pool(uri: &String) -> Result<Pool> {
    let manager = Manager::new(uri, Runtime::Tokio1);
    let pool = Pool::builder(manager).build()?;
    Ok(pool)
}

pub trait Repository {
    fn new(pool: Pool) -> Self;
}

#[derive(Clone)]
pub struct AppState {
    pool: Pool,
}

impl AppState {
    pub fn build(uri: &String) -> Result<Self> {
        let pool = pool(uri)?;
        Ok(AppState { pool })
    }

    pub fn get_repo<T>(&self) -> T
    where
        T: Repository + NewsRepository,
    {
        T::new(self.pool.clone())
    }
}
