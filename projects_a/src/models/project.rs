use uuid::Uuid;
use chrono::NaiveDate;
use sqlx::PgPool;
use async_trait::async_trait;

use crate::traits::model_trait::ModelTrait;

#[derive(Debug, sqlx::FromRow)]
pub struct Project {
    pub project_id: Uuid,
    pub project_name: String,
    pub project_start_date: NaiveDate,
    pub project_end_date: NaiveDate,
    pub pay_rate: f64,
    pub project_total_duration: String,
    pub project_total_pay: f64,
}

impl Project {
    pub fn new(
        project_id: Uuid,
        project_name: String,
        project_start_date: NaiveDate,
        project_end_date: NaiveDate,
        pay_rate: f64,
    ) -> Self {
        Self {
            project_id,
            project_name,
            project_start_date,
            project_end_date,
            pay_rate,
            project_total_duration: "00:00:00".to_string(),
            project_total_pay: 0.0,
        }
    }
}

#[async_trait(?Send)]
impl ModelTrait for Project {
    async fn create(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO Projects (ProjectId, ProjectName, ProjectStartDate, ProjectEndDate, PayRate)
             VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(&self.project_id)
        .bind(&self.project_name)
        .bind(&self.project_start_date)
        .bind(&self.project_end_date)
        .bind(&self.pay_rate)
        .execute(pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM Projects WHERE ProjectId = $1")
            .bind(&self.project_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    #[test]
    fn test_new_project() {
        let project_id = Uuid::new_v4();
        let project_name = "Test Project".to_string();
        let project_start_date = NaiveDate::from_ymd(2022, 1, 1);
        let project_end_date = NaiveDate::from_ymd(2022, 12, 31);
        let pay_rate = 100.0;

        let project = Project::new(project_id, project_name.clone(), project_start_date, project_end_date, pay_rate);

        assert_eq!(project.project_id, project_id);
        assert_eq!(project.project_name, project_name);
        assert_eq!(project.project_start_date, project_start_date);
        assert_eq!(project.project_end_date, project_end_date);
        assert_eq!(project.pay_rate, pay_rate);
    }

    #[sqlx::test]
    async fn test_project_round_trip(pool: PgPool) {
        let project_id = Uuid::new_v4();
        let project_name = "Test Project".to_string();
        let project_start_date = NaiveDate::from_ymd(2022, 1, 1);
        let project_end_date = NaiveDate::from_ymd(2022, 12, 31);
        let pay_rate = 100.0;

        let project = Project::new(project_id, project_name.clone(), project_start_date, project_end_date, pay_rate);

        // Insert data into the test database
        project.create(&pool).await.unwrap();

        // Retrieve the data from the test database
        let retrieved_project = sqlx::query_as("SELECT * FROM Projects WHERE ProjectId = $1")
            .bind(&project_id)
            .fetch_one(&pool)
            .await
            .unwrap();

        // Compare the retrieved data to the original data
        assert_eq!(retrieved_project.project_id, project_id);
        assert_eq!(retrieved_project.project_name, project_name);
        assert_eq!(retrieved_project.project_start_date, project_start_date);
        assert_eq!(retrieved_project.project_end_date, project_end_date);
        assert_eq!(retrieved_project.pay_rate, pay_rate);
    }
}
