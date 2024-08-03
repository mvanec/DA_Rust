// lib.rs

use database::Database;
use query::ToQuery;
use config::Config;

pub mod database;
pub mod models;
pub mod query;
pub mod config;

#[tokio::main]
async fn main() -> Result<(), sqlite::Error> {
    // Some sample data to insert into the database
    let project_data = models::Project::default();

    // Initialize the database
    let config = Config {
        uri: "file:test.db?mode=memory&cache=shared".to_string(),
    };
    let database = Database::new(&config).await?;

    // Insert data into the database
    database.insert(&project_data).await?;

    Ok(())
}
