use sqlx::PgPool;
use tokio;

mod project;

#[tokio::test]
async fn test_project_create_delete() {
    let pool = create_test_pool().await.unwrap();
    let project = project::Project::new(
        uuid::Uuid::new_v4(),
        "Test Project".to_string(),
        chrono::NaiveDate::from_ymd(2022, 1, 1),
        chrono::NaiveDate::from_ymd(2022, 12, 31),
        100.0,
    );

    project.create(&pool).await.unwrap();

    let retrieved_project = sqlx::query("SELECT * FROM Projects WHERE ProjectId = $1")
        .bind(&project.project_id)
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(retrieved_project.project_id, project.project_id);
    assert_eq!(retrieved_project.project_name, project.project_name);
    assert_eq!(retrieved_project.project_start_date, project.project_start_date);
    assert_eq!(retrieved_project.project_end_date, project.project_end_date);
    assert_eq!(retrieved_project.pay_rate, project.pay_rate);

    project.delete(&pool).await.unwrap();

    let count = sqlx::query("SELECT COUNT(*) FROM Projects WHERE ProjectId = $1")
        .bind(&project.project_id)
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
