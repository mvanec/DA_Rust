// database.rs

use sqlite::{Pool, Error};
use futures::Future;
use tokio::io::{self, AsyncReadExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::query::ToQuery;
use crate::Config;

pub struct Database {
    pool: Pool,
}

impl Database {
    pub async fn new(config: &Config) -> Result<Self, Error> {
        let pool = Pool::new(config.uri)?;
        Ok(Self { pool })
    }

    pub async fn insert<T: ToQuery>(&mut self, data: T) -> Result<(), Error> {
        let conn = self.pool.get().await?;
        let query = data.to_query();
        conn.execute(&query).await?;
        Ok(())
    }
}
