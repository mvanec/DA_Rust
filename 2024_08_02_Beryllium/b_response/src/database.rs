// database.rs

use sqlite3::{Pool, Connection, Error};
use futures::Future;
use async_std::task;
use config::Config;
use query::Query;

pub struct Database {
    conn: Pool,
}

impl Database {
    async fn new(config: Config) -> Result<Self, Error> {
        let conn = Pool::new(config.uri.as_str()).await?;
        Self { conn }
    }

    async fn insert_project(&mut self, project: models::Project) -> Result<(), Error> {
        let query = query::from_project(&project);
        let stmt = self.conn.prepare(&query.sql).await?;
        stmt.execute(&query.values).await?;
        Ok(())
    }
    // ... other database functions
}
