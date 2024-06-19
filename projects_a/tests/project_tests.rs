mod utils;
use crate::utils::create_test_pool;

use sqlx::PgPool;
use sqlx::Row;
use tokio;
use ctor::ctor;

use projects::models::project::Project;
use projects::traits::model_trait::ModelTrait;

#[ctor]
fn setup() {
    eprintln!("******************************************************");
    utils::test_setup();
}

// Create a test pool and a project
async fn setup_test_project() -> (PgPool, Project) {
    let pool = create_test_pool().await.unwrap();
    let project = Project::new(
        uuid::Uuid::new_v4(),
        "Test Project".to_string(),
        chrono::NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
        chrono::NaiveDate::from_ymd_opt(2022, 12, 31).unwrap(),
        100.0,
    );
    (pool, project)
}

#[tokio::test]
async fn test_project_create() -> Result<(), sqlx::Error> {
    // Create a test pool and a project
    let (pool, project) = setup_test_project().await;

    // Create the project in the database
    project.create(&pool).await?;

    // Retrieve the project from the database
    let retrieved_project = sqlx::query("SELECT * FROM Projects WHERE ProjectId = $1")
        .bind(&project.project_id)
        .fetch_one(&pool)
        .await?;

    // Check that the retrieved project matches the original project
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

    Ok(())
}

#[tokio::test]
async fn test_project_delete() -> Result<(), sqlx::Error> {
    // Create a test pool and a project
    let (pool, project) = setup_test_project().await;

    // Create the project in the database
    project.create(&pool).await?;

    // Delete the project from the database
    project.delete(&pool).await?;

    // Check that the project was deleted
    let count = sqlx::query("SELECT COUNT(*) FROM Projects WHERE ProjectId = $1")
        .bind(&project.project_id)
        .fetch_one(&pool)
        .await?;

    let count: i64 = count.get(0);
    assert_eq!(count, 0);

    Ok(())
}
