use uuid::Uuid;
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};
use sqlx::{PgConnection, FromRow, Error};

pub trait Model {
    fn insert(&self, connection: &mut PgConnection) -> Result<(), Error>;
    fn delete(&self, connection: &mut PgConnection) -> Result<(), Error>;
}

#[derive(Debug, FromRow)]
pub struct Project {
    pub project_id: Uuid,
    pub project_name: String,
    pub project_start_date: NaiveDate,
    pub project_end_date: NaiveDate,
    pub pay_rate: f64,
    pub project_total_duration: NaiveTime,
    pub project_total_pay: f64,
}

#[derive(Debug, FromRow)]
pub struct ProjectTask {
    pub task_id: Uuid,
    pub project_id: Uuid,
    pub task_name: String,
    pub task_total_duration: NaiveTime,
}

#[derive(Debug, FromRow)]
pub struct TaskTiming {
    pub timing_id: i32,
    pub task_id: Uuid,
    pub start_timestamp: NaiveDateTime,
    pub end_timestamp: NaiveDateTime,
}
