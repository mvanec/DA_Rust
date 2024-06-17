use crate::model::{Model, Project};
use sqlx::{PgConnection, Error};

impl Model for Project {
    fn insert(&self, connection: &mut PgConnection) -> Result<(), Error> {
        sqlx::query("INSERT INTO Projects (ProjectId, ProjectName, ProjectStartDate, ProjectEndDate, PayRate, ProjectTotalDuration, ProjectTotalPay) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(self.project_id)
            .bind(&self.project_name)
            .bind(self.project_start_date)
            .bind(self.project_end_date)
            .bind(self.pay_rate)
            .bind(self.project_total_duration)
            .bind(self.project_total_pay)
            .execute(connection)
            .map(|_| ())
    }

    fn delete(&self, connection: &mut PgConnection) -> Result<(), Error> {
        sqlx::query("DELETE FROM Projects WHERE ProjectId = $1")
            .bind(self.project_id)
            .execute(connection)
            .map(|_| ())
    }
}
