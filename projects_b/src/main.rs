mod models;
mod traits;

use dotenv::dotenv;
use std::env;
use sqlx::PgPool;

use traits::model_trait::load_from_csv;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let projects_csv = env::var("PROJECTS_CSV").expect("PROJECTS_CSV must be set");
    let tasks_csv = env::var("TASKS_CSV").expect("TASKS_CSV must be set");
    let timings_csv = env::var("TIMINGS_CSV").expect("TIMINGS_CSV must be set");

    let pool = PgPool::connect(&database_url).await?;

    // Load projects from CSV
    load_from_csv(&projects_csv, &pool, |record| {
        models::project::Project::new(
            uuid::Uuid::parse_str(&record[0]).unwrap(),
            record[1].clone(),
            chrono::NaiveDate::parse_from_str(&record[2], "%Y-%m-%d").unwrap(),
            chrono::NaiveDate::parse_from_str(&record[3], "%Y-%m-%d").unwrap(),
            record[4].parse().unwrap(),
        )
    })
    .await
    .unwrap();

    // Load tasks from CSV
    load_from_csv(&tasks_csv, &pool, |record| {
        models::task::Task::new(
            uuid::Uuid::parse_str(&record[0]).unwrap(),
            uuid::Uuid::parse_str(&record[1]).unwrap(),
            record[2].clone(),
        )
    })
    .await
    .unwrap();

    // Load timings from CSV
    load_from_csv(&timings_csv, &pool, |record| {
        models::timing::Timing::new(
            uuid::Uuid::parse_str(&record[1]).unwrap(),
            chrono::NaiveDateTime::parse_from_str(&record[2], "%Y-%m-%d %H:%M:%S").unwrap(),
            chrono::NaiveDateTime::parse_from_str(&record[3], "%Y-%m-%d %H:%M:%S").unwrap(),
        )
    })
    .await
    .unwrap();

    Ok(())
}
