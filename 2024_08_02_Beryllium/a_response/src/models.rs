// models.rs
use serde::{Deserialize, Serialize};
use serde_json::Value;
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug)]
pub struct Project {
    pub project_id: Uuid,
    pub project_name: String,
    pub project_start_date: NaiveDate,
    pub project_end_date: NaiveDate,
    pub pay_rate: f64,
    pub project_duration: i32,
    pub project_total_pay: f64,
}
