use anyhow::{Result, Error};
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::deadpool::{Object, Pool as BasePool};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;

pub struct Pool {
    pub pool: BasePool<AsyncPgConnection>,
}

impl Pool {
    pub fn new(uri: &String) -> Result<Self> {
        let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(uri);
        let pool: BasePool<AsyncPgConnection>  = BasePool::builder(manager).build()?;
        Ok(Self { pool })
    }
    
    pub async fn get(&self) -> Result<Object<AsyncPgConnection>> {
        self.pool.get().await.map_err(Error::msg)
    }
}
