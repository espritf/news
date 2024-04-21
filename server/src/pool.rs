use anyhow::{Result, Error};
use deadpool_diesel::postgres::{Manager, Object, Pool as DeadPool, Runtime};

pub struct Pool {
    pub pool: DeadPool,
}

impl Pool {
    pub fn new(uri: &String) -> Result<Self> {
        let manager = Manager::new(uri, Runtime::Tokio1);
        let pool = DeadPool::builder(manager).build()?;
        Ok(Self { pool })
    }

    pub async fn get(&self) -> Result<Object> {
        let p = self.pool.get();
        p.await.map_err(Error::msg)
    }
}
