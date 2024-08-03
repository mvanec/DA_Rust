// tests/test.rs

use database::Database;
use models::Project;
use query::ToQuery;
use config::Config;
use sqlite::Error;
use tokio::test::{test, Test};

#[cfg(test)]
mod tests {
    #[test]
    async fn test_database_insert() {
        let config = Config {
            uri: "file:test.db?mode=memory&cache=shared",
        };
        let database = Database::new(&config).await?;
        let project = Project::default();
        database.insert(&project).await?;
    }
}
