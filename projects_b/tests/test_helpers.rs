// tests/test_helpers.rs
use ctor::ctor;
use log::{error, info};
use sqlx::PgPool;
use std::env;
use tokio;

use projects::models::project::Project;

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
                match sqlx::query("DROP TABLE IF EXISTS TaskTimings")
                    .execute(&mut *tx)
                    .await
                {
                    Ok(_) => info!("Dropped TaskTimings table"),
                    Err(e) => {
                        error!("Failed to drop TaskTimings table: {}", e);
                        panic!("Failed to drop TaskTimings table");
                    }
                }

                match sqlx::query("DROP TABLE IF EXISTS ProjectTasks")
                    .execute(&mut *tx)
                    .await
                {
                    Ok(_) => info!("Dropped ProjectTasks table"),
                    Err(e) => {
                        error!("Failed to drop ProjectTasks table: {}", e);
                        panic!("Failed to drop ProjectTasks table");
                    }
                }

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

                // Create the tables
                match sqlx::query(
                    "CREATE TABLE Projects (
                    ProjectId UUID PRIMARY KEY,
                    ProjectName VARCHAR(100) NOT NULL,
                    ProjectStartDate DATE NOT NULL,
                    ProjectEndDate DATE NOT NULL,
                    PayRate DECIMAL(10, 2) NOT NULL,
                    ProjectTotalDuration TIME NOT NULL DEFAULT '00:00:00',
                    ProjectTotalPay DECIMAL(10, 2) NOT NULL DEFAULT '0.00'
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

                match sqlx::query(
                    "CREATE TABLE ProjectTasks (
                    TaskId UUID PRIMARY KEY,
                    ProjectId UUID NOT NULL,
                    TaskName VARCHAR(100) NOT NULL,
                    TaskTotalDuration TIME NOT NULL DEFAULT '00:00:00',
                    FOREIGN KEY (ProjectId) REFERENCES Projects(ProjectId) ON DELETE CASCADE
                )",
                )
                .execute(&mut *tx)
                .await
                {
                    Ok(_) => info!("Created ProjectTasks table"),
                    Err(e) => {
                        error!("Failed to create ProjectTasks table: {}", e);
                        panic!("Failed to create ProjectTasks table");
                    }
                }

                match sqlx::query(
                    "CREATE TABLE TaskTimings (
                    TimingId SERIAL UNIQUE NOT NULL,
                    TaskId UUID NOT NULL,
                    StartTimestamp TIMESTAMP NOT NULL,
                    EndTimestamp TIMESTAMP NOT NULL,
                    PRIMARY KEY (TimingId),
                    FOREIGN KEY (TaskId) REFERENCES ProjectTasks(TaskId) ON DELETE CASCADE
                )",
                )
                .execute(&mut *tx)
                .await
                {
                    Ok(_) => info!("Created TaskTimings table"),
                    Err(e) => {
                        error!("Failed to create TaskTimings table: {}", e);
                        panic!("Failed to create TaskTimings table");
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

pub async fn create_test_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;
    Ok(pool)
}

pub async fn setup_test_project() -> Result<Project, sqlx::Error> {
    let project = Project::new(
        uuid::Uuid::new_v4(),
        "Test Project".to_string(),
        chrono::NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
        chrono::NaiveDate::from_ymd_opt(2022, 12, 31).unwrap(),
        100.0,
    );
    Ok(project)
}
