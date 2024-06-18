use sqlx::PgPool;
use sqlx::Row;
use tokio;
use std::env;

use projects::models::project::Project;
use projects::traits::model_trait::ModelTrait;

#[tokio::test]
async fn test_project_create() {
    let pool = create_test_pool().await.unwrap();

    let project = Project::new(
        uuid::Uuid::new_v4(),
        "Test Project".to_string(),
        chrono::NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
        chrono::NaiveDate::from_ymd_opt(2022, 12, 31).unwrap(),
        100.0,
    );

    project.create(&pool).await.unwrap();

    let retrieved_project = sqlx::query("SELECT * FROM Projects WHERE ProjectId = $1")
        .bind(&project.project_id)
        .fetch_one(&pool)
        .await
        .unwrap();

    // ... assert that the project was created correctly ...
}

#[tokio::test]
async fn test_project_delete() {
    let pool = create_test_pool().await.unwrap();

    let project = Project::new(
        uuid::Uuid::new_v4(),
        "Test Project".to_string(),
        chrono::NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
        chrono::NaiveDate::from_ymd_opt(2022, 12, 31).unwrap(),
        100.0,
    );

    project.create(&pool).await.unwrap();

    project.delete(&pool).await.unwrap();

    let count = sqlx::query("SELECT COUNT(*) FROM Projects WHERE ProjectId = $1")
        .bind(&project.project_id)
        .fetch_one(&pool)
        .await
        .unwrap();

    let count: i64 = count.get(0);
    assert_eq!(count, 0);
}

async fn create_test_pool() -> Result<PgPool, sqlx::Error> {
    dotenv::from_filename(".env.test").ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url).await?;

    // Drop the database if it exists
    sqlx::query("DROP TABLE IF EXISTS Projects")
        .execute(&pool)
        .await?;

    // Create the table
    sqlx::query("CREATE TABLE Projects (
        ProjectId UUID PRIMARY KEY,
        ProjectName VARCHAR(255),
        ProjectStartDate DATE,
        ProjectEndDate DATE,
        PayRate FLOAT
    )")
    .execute(&pool)
    .await?;

    Ok(pool)
}
