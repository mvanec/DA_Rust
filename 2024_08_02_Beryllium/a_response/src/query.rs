// query.rs
use crate::models::*;
use sqlite::stmt::{Parameter, Query};
use chrono::NaiveDate;
use uuid::Uuid;

pub trait ToQuery {
    fn to_query(&self) -> Query;
}

impl ToQuery for Project {
    fn to_query(&self) -> Query {
        let query_str = "INSERT INTO projects (
            project_id,
            project_name,
            project_start_date,
            project_end_date,
            pay_rate,
            project_duration,
            project_total_pay
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7
        )";

        Self::new(query_str)
            .bind::<Uuid>(&self.project_id)?
            .bind::<String>(&self.project_name)?
            .bind::<NaiveDate>(&self.project_start_date)?
            .bind::<NaiveDate>(&self.project_end_date)?
            .bind::<f64>(&self.pay_rate)?
            .bind::<i32>(&self.project_duration)?
            .bind::<f64>(&self.project_total_pay)?
            .finish()
    }
}
