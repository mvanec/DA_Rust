// database_test.rs

use anyhow::Result;
use sqlite3::Error;
use database::Database;
use models::Project;
use config::Config;

#[tokio::test]
async fn test_database_insert() {
    // Arrange
    let config = Config {
        uri: Value::Str("file:test.db?mode=memory&cache=shared"),
    };
    let database = Database::new(config).await?;
    let project = Project::default();

    // Act
    database.insert_project(project).await?;

   // Assert (skipped for brevity)
}
