use async_trait::async_trait;
use sqlx::PgPool;
use std::io;

#[async_trait(?Send)]
pub trait ModelTrait {
    async fn create(&self, pool: &PgPool) -> Result<(), sqlx::Error>;
    async fn delete(&self, pool: &PgPool) -> Result<(), sqlx::Error>;
}

pub async fn load_from_csv<T, F>(path: &str, pool: &PgPool, f: F) -> Result<(), io::Error>
where
    F: Fn(Vec<String>) -> T,
    T: ModelTrait + Send + 'static,
{
    let mut rdr = csv::Reader::from_path(path)?;
    for result in rdr.records() {
        let record = result?;
        let record: Vec<String> = record.into_iter().map(|s| s.to_string()).collect();
        let model = f(record);
        model.create(pool).await.unwrap();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    use uuid::Uuid;
    use chrono::NaiveDate;
    use crate::Project;

    #[sqlx::test]
    async fn test_load_from_csv_round_trip(pool: PgPool) {
        let project_id = Uuid::new_v4();
        let project_name = "Test Project".to_string();
        let project_start_date = NaiveDate::from_ymd(2022, 1, 1);
        let project_end_date = NaiveDate::from_ymd(2022, 12, 31);
        let pay_rate = 100.0;

        let project = Project::new(project_id, project_name.clone(), project_start_date, project_end_date, pay_rate);

        // Create a CSV file
        let mut csv_file = tempfile::NamedTempFile::new().unwrap();
        csv_file.write_all(format!("ProjectId,ProjectName,ProjectStartDate,ProjectEndDate,PayRate\n{}, {}, {}, {}, {}\n", project_id, project_name, project_start_date, project_end_date, pay_rate).as_bytes()).unwrap();

        // Load the CSV file into the test database
        load_from_csv(csv_file.path().to_str().unwrap(), &pool, |record| {
            Project::new(
                Uuid::parse_str(&record[0]).unwrap(),
                record[1].clone(),
                NaiveDate::parse_from_str(&record[2], "%Y-%m-%d").unwrap(),
                NaiveDate::parse_from_str(&record[3], "%Y-%m-%d").unwrap(),
                record[4].parse::<f64>().unwrap(),
            )
        })
        .await
        .unwrap();

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