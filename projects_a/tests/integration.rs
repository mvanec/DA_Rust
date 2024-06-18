use sqlx::PgPool;
use tokio;
use uuid::Uuid;
use chrono::NaiveDate;

mod project;

#[tokio::test]
async fn test_project_create_delete() {
    let pool = create_test_pool().await.unwrap();
    let project_id = Uuid::new_v4();
    let project_name = "Test Project".to_string();
    let project_start_date = NaiveDate::from_ymd_opt(2022, 1, 1).unwrap();
    let project_end_date = NaiveDate::from_ymd_opt(2022, 12, 31).unwrap();
    let pay_rate = 100.0;

    let project = project::Project::new(
        project_id,
        project_name.clone(),
        project_start_date,
        project_end_date,
        pay_rate,
    );

    project.create(&pool).await.unwrap();

    let retrieved_project = sqlx::query_as!(
        project::Project,
        "SELECT * FROM Projects WHERE ProjectId = $1",
        project_id
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(retrieved_project.project_id, project.project_id);
    assert_eq!(retrieved_project.project_name, project.project_name);
    assert_eq!(retrieved_project.project_start_date, project.project_start_date);
    assert_eq!(retrieved_project.project_end_date, project.project_end_date);
    assert_eq!(retrieved_project.pay_rate, project.pay_rate);

    project.delete(&pool).await.unwrap();

    let count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM Projects WHERE ProjectId = $1",
        project_id
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(count, 0);
}

async fn create_test_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = "postgres://localhost/test_database";
    let pool = PgPool::connect(&database_url).await?;

    sqlx::query("CREATE TABLE IF NOT EXISTS Projects (
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
