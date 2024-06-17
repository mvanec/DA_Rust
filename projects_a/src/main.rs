use sqlx::PgConnection;
use uuid::Uuid;
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};
use project::{Project};
use project_task::{ProjectTask};
use task_timing::{TaskTiming};

#[tokio::main]
async fn main() {
    let connection_string = "postgres://user:password@localhost/database";
    let mut connection = PgConnection::connect(connection_string)
        .await
        .expect("Failed to connect to database");

    let project = Project {
        project_id: Uuid::new_v4(),
        project_name: "Test Project".to_string(),
        project_start_date: NaiveDate::from_ymd(2022, 1, 1),
        project_end_date: NaiveDate::from_ymd(2022, 12, 31),
        pay_rate: 20.0,
        project_total_duration: NaiveTime::from_hms(0, 0, 0),
        project_total_pay: 0.0,
    };

    project.insert(&mut connection)
        .await
        .expect("Failed to insert project");

    let project_task = ProjectTask {
        task_id: Uuid::new_v4(),
        project_id: project.project_id,
        task_name: "Test Task".to_string(),
        task_total_duration: NaiveTime::from_hms(0, 0, 0),
    };

    project_task.insert(&mut connection)
        .await
        .expect("Failed to insert project task");

    let task_timing = TaskTiming {
        timing_id: 0,
        task_id: project_task.task_id,
        start_timestamp: NaiveDateTime::from_timestamp(1643723400, 0),
        end_timestamp: NaiveDateTime::from_timestamp(1643723400, 0),
    };

    task_timing.insert(&mut connection)
        .await
        .expect("Failed to insert task timing");
}
