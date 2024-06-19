// tests/integration/task_test.rs
use log::{error, info};
use sqlx::PgPool;
use sqlx::Row;
use tokio;

use projects::models::task::Task;
use projects::traits::model_trait::ModelTrait;

mod test_helpers;
//use crate::test_helpers::*;

// Create a test pool and a task
async fn setup_test_task() -> (PgPool, Task) {
    let pool = test_helpers::create_test_pool().await.unwrap();
    let project = test_helpers::setup_test_project().await.unwrap();
    project.create(&pool).await.unwrap();

    let task = Task::new(
        uuid::Uuid::new_v4(),
        project.project_id,
        "Test Task".to_string(),
    );
    (pool, task)
}

#[tokio::test]
async fn test_task_create() -> Result<(), sqlx::Error> {
    // Create a test pool and a task
    let (pool, task) = setup_test_task().await;

    // Create the task in the database
    task.create(&pool).await?;

    // Retrieve the task from the database
    let retrieved_task = sqlx::query("SELECT * FROM ProjectTasks WHERE TaskId = $1")
        .bind(&task.task_id)
        .fetch_one(&pool)
        .await?;

    // Check that the retrieved task matches the original task
    let task_id: uuid::Uuid = retrieved_task.get("taskid");
    let project_id: uuid::Uuid = retrieved_task.get("projectid");
    let task_name: String = retrieved_task.get("taskname");
    let task_total_duration: String = retrieved_task.get("tasktotalduration");

    assert_eq!(task_id, task.task_id);
    assert_eq!(project_id, task.project_id);
    assert_eq!(task_name, task.task_name);
    assert_eq!(task_total_duration, task.task_total_duration);

    Ok(())
}

#[tokio::test]
async fn test_task_delete() -> Result<(), sqlx::Error> {
    // Create a test pool and a task
    let (pool, task) = setup_test_task().await;

    // Create the task in the database
    task.create(&pool).await?;

    // Delete the task from the database
    task.delete(&pool).await?;

    // Check that the task was deleted
    let count = sqlx::query("SELECT COUNT(*) FROM ProjectTasks WHERE TaskId = $1")
        .bind(&task.task_id)
        .fetch_one(&pool)
        .await?;

    let count: i64 = count.get(0);
    assert_eq!(count, 0);

    Ok(())
}
