use sqlx::PgPool;
use sqlx::types::Uuid;
use chrono::NaiveDateTime;

use crate::traits::model_trait::ModelTrait;

#[derive(Debug, sqlx::FromRow)]
pub struct Timing {
    pub timing_id: i32,
    pub task_id: Uuid,
    pub start_timestamp: NaiveDateTime,
    pub end_timestamp: NaiveDateTime,
}

impl Timing {
    pub fn new(task_id: Uuid, start_timestamp: NaiveDateTime, end_timestamp: NaiveDateTime) -> Self {
        Self {
            timing_id: 0,
            task_id,
            start_timestamp,
            end_timestamp,
        }
    }
}

#[async_trait]
impl ModelTrait for Timing {
    async fn create(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO TaskTimings (TaskId, StartTimestamp, EndTimestamp)
             VALUES ($1, $2, $3)",
        )
        .bind(&self.task_id)
        .bind(&self.start_timestamp)
        .bind(&self.end_timestamp)
        .execute(pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM TaskTimings WHERE TimingId = $1")
            .bind(&self.timing_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
