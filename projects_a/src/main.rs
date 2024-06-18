mod models;
mod traits;

use dotenv::dotenv;
use std::env;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{NaiveDate, NaiveDateTime};

use models::project::Project;
use models::task::Task;
use models::timing::Timing;
use traits::model_trait::{load_from_csv, ModelTrait};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let projects_csv = env::var("PROJECTS_CSV").expect("PROJECTS_CSV must be set");
    let tasks_csv = env::var("TASKS_CSV").expect("TASKS_CSV must be set");
    let timings_csv = env::var("TIMINGS_CSV").expect("TIMINGS_CSV must be set");

    let pool = PgPool::connect(&database_url).await?;

    // Example project
    let project = Project::new(
        Uuid::new_v4(),
        "Test Project".to_string(),
        NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
        NaiveDate::from_ymd_opt(2022, 12, 31).unwrap(),
        50.0,
    );
    project.create(&pool).await?;

    // Example task
    let task = Task::new(Uuid::new_v4(), project.project_id, "Test Task".to_string());
    task.create(&pool).await?;

    // Example timing
    let timing = Timing::new(
        task.task_id,
        NaiveDate::from_ymd_opt(2022, 1, 1).unwrap().and_hms_opt(8, 0, 0).unwrap(),
        NaiveDate::from_ymd_opt(2022, 1, 1).unwrap().and_hms_opt(16, 0, 0).unwrap(),
    );
    timing.create(&pool).await?;

    // Load projects from CSV
    load_from_csv(&projects_csv, &pool, |record| {
        Project::new(
            Uuid::parse_str(&record[0]).unwrap(),
            record[1].clone(),
            NaiveDate::parse_from_str(&record[2], "%Y-%m-%d").unwrap(),
            NaiveDate::parse_from_str(&record[3], "%Y-%m-%d").unwrap(),
            record[4].parse().unwrap(),
        )
    })
    .await
    .unwrap();

    // Load tasks from CSV
    load_from_csv(&tasks_csv, &pool, |record| {
        Task::new(
            Uuid::parse_str(&record[0]).unwrap(),
            Uuid::parse_str(&record[1]).unwrap(),
            record[2].clone(),
        )
    })
    .await
    .unwrap();

    // Load timings from CSV
    load_from_csv(&timings_csv, &pool, |record| {
        Timing::new(
            Uuid::parse_str(&record[1]).unwrap(),
            NaiveDateTime::parse_from_str(&record[2], "%Y-%m-%d %H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str(&record[3], "%Y-%m-%d %H:%M:%S").unwrap(),
        )
    })
    .await
    .unwrap();

    Ok(())
}
