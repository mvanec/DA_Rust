use ctor::ctor;
use log::{error, info};
use sqlx::PgPool;
use sqlx::Row;
use std::env;
use tokio;

use projects::models::project::Project;
use projects::traits::model_trait::ModelTrait;

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

#[ctor]
fn test_setup() {
    dotenv::from_filename(".env.test").ok();
    std::env::set_var(
        "RUST_LOG",
        env::var("RUST_LOG").unwrap_or(String::from("info")),
    );
    env_logger::init();

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    info!("Commencing the test run");
    rt.block_on(async {
        match create_test_pool().await {
            Ok(pool) => {
                let mut tx = match pool.begin().await {
                    Ok(tx) => tx,
                    Err(e) => {
                        error!("Failed to start transaction: {}", e);
                        panic!("Failed to start transaction");
                    }
                };

                // Drop the database if it exists
                match sqlx::query("DROP TABLE IF EXISTS Projects")
                    .execute(&mut *tx)
                    .await
                {
                    Ok(_) => info!("Dropped Projects table"),
                    Err(e) => {
                        error!("Failed to drop Projects table: {}", e);
                        panic!("Failed to drop Projects table");
                    }
                }

                // Create the table
                match sqlx::query(
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
                {
                    Ok(_) => info!("Created Projects table"),
                    Err(e) => {
                        error!("Failed to create Projects table: {}", e);
                        panic!("Failed to create Projects table");
                    }
                }

                match tx.commit().await {
                    Ok(_) => info!("Committed transaction"),
                    Err(e) => {
                        error!("Failed to commit transaction: {}", e);
                        panic!("Failed to commit transaction");
                    }
                }
            }
            Err(e) => {
                error!("Failed to create test pool: {}", e);
                panic!("Failed to create test pool");
            }
        }
    });
}

async fn create_test_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;
    Ok(pool)
}
