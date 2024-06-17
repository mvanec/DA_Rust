use crate::model::{Model, ProjectTask};
use sqlx::{PgConnection, Error};

impl Model for ProjectTask {
    fn insert(&self, connection: &mut PgConnection) -> Result<(), Error> {
        sqlx::query("INSERT INTO ProjectTasks (TaskId, ProjectId, TaskName, TaskTotalDuration) VALUES ($1, $2, $3, $4)")
            .bind(self.task_id)
            .bind(self.project_id)
            .bind(&self.task_name)
            .bind(self.task_total_duration)
            .execute(connection)
            .map(|_| ())
    }

    fn delete(&self, connection: &mut PgConnection) -> Result<(), Error> {
        sqlx::query("DELETE FROM ProjectTasks WHERE TaskId = $1")
            .bind(self.task_id)
            .execute(connection)
            .map(|_| ())
    }
}
