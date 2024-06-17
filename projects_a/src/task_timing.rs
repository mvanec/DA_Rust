use crate::model::{Model, TaskTiming};
use sqlx::{PgConnection, Error};

impl Model for TaskTiming {
    fn insert(&self, connection: &mut PgConnection) -> Result<(), Error> {
        sqlx::query("INSERT INTO TaskTimings (TaskId, StartTimestamp, EndTimestamp) VALUES ($1, $2, $3)")
            .bind(self.task_id)
            .bind(self.start_timestamp)
            .bind(self.end_timestamp)
            .execute(connection)
            .map(|_| ())
    }

    fn delete(&self, connection: &mut PgConnection) -> Result<(), Error> {
        sqlx::query("DELETE FROM TaskTimings WHERE TimingId = $1")
            .bind(self.timing_id)
            .execute(connection)
            .map(|_| ())
    }
}
