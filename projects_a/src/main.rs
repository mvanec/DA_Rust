mod models;
mod traits;
mod config;

use models::{Project, Task, Timing};
use traits::model_trait::{load_from_csv, ModelTrait};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::NaiveDate;
use config::Config;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let config = Config::new();
    let pool = PgPool::connect(config.database_url()).await?;

    // Example project
    let project = Project::new(
        Uuid::new_v4(),
        "Test Project".to_string(),
        NaiveDate::from_ymd(2022, 1, 1),
        NaiveDate::from_ymd(2022, 12, 31),
        50.0,
    );
    project.create(&pool).await?;

    // Example task
    let task = Task::new(Uuid::new_v4(), project.project_id, "Test Task".to_string());
    task.create(&pool).await?;

    // Example timing
    let timing = Timing::new(
        task.task_id,
        NaiveDate::from_ymd(2022, 1, 1).and_hms(8, 0, 0),
        NaiveDate::from_ymd(2022, 1, 1).and_hms(16, 0, 0),
    );
    timing.create(&pool).await?;

    // Load projects from CSV
    load_from_csv(config.projects_csv(), &pool, |record| {
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
    load_from_csv(config.tasks_csv(), &pool, |record| {
        Task::new(
            Uuid::parse_str(&record[0]).unwrap(),
            Uuid::parse_str(&record[1]).unwrap(),
            record[2].clone(),
        )
    })
    .await
    .unwrap();

    // Load timings from CSV
    load_from_csv(config.timings_csv(), &pool, |record| {
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
