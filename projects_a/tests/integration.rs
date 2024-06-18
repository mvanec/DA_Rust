use ctor::ctor;
use sqlx::PgPool;
use sqlx::Row;
use std::env;
use tokio;

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

    let project_id: uuid::Uuid = retrieved_project.get("projectid");
    let project_name: String = retrieved_project.get("projectname");
    let project_start_date: chrono::NaiveDate = retrieved_project.get("projectstartdate");
    let project_end_date: chrono::NaiveDate = retrieved_project.get("projectenddate");
    let pay_rate: f64 = retrieved_project.get("payrate");

    assert_eq!(project_id, project.project_id);
    assert_eq!(project_name, project.project_name);
    assert_eq!(project_start_date, project.project_start_date);
    assert_eq!(project_end_date, project.project_end_date);
    assert_eq!(pay_rate, project.pay_rate);
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

#[ctor]
fn test_setup() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        let pool = create_test_pool().await.expect("Failed to create test pool");
        let mut tx = pool.begin().await.expect("Failed to start transaction");

        // Drop the database if it exists
        sqlx::query("DROP TABLE IF EXISTS Projects")
            .execute(&mut *tx)
            .await
            .expect("Failed to drop Projects table");

        // Create the table
        sqlx::query(
            "CREATE TABLE Projects (
            ProjectId UUID PRIMARY KEY,
            ProjectName VARCHAR(255),
            ProjectStartDate DATE,
            ProjectEndDate DATE,
            PayRate FLOAT
        )",
        )
        .execute(&mut *tx)
        .await
        .expect("Failed to create Projects table");

        tx.commit().await.expect("Failed to commit transaction");
    });
}

async fn create_test_pool() -> Result<PgPool, sqlx::Error> {
    dotenv::from_filename(".env.test").ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;
    Ok(pool)
}
