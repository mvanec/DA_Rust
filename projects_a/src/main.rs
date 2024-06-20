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
        let id = uuid::Uuid::parse_str(&record[0])
            .expect("Failed to parse project ID");
        let name = record[1].clone();
        let start_date = chrono::NaiveDate::parse_from_str(&record[2], "%Y-%m-%d")
            .expect("Failed to parse project start date");
        let end_date = chrono::NaiveDate::parse_from_str(&record[3], "%Y-%m-%d")
            .expect("Failed to parse project end date");
        let cost = record[4].parse()
            .expect("Failed to parse project cost");
        let stage = record[5].parse()
            .expect("Failed to parse project stage");
        let phase = record[6].parse()
            .expect("Failed to parse project phase");

        models::project::Project::new(id, name, start_date, end_date, cost, stage, phase)
    })
    .await
    .expect("Failed to load projects from CSV");

    // Load tasks from CSV
    load_from_csv(&tasks_csv, &pool, |record| {
        models::task::Task::new(
            uuid::Uuid::parse_str(&record[0]).unwrap(),
            uuid::Uuid::parse_str(&record[1]).unwrap(),
            record[2].clone(),
            record[3].parse().unwrap()
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
