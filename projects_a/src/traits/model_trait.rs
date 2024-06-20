use async_trait::async_trait;
use sqlx::{error::DatabaseError, PgPool};
use std::{fmt, io};

#[async_trait(?Send)]
pub trait ModelTrait {
    async fn create(&self, pool: &PgPool) -> Result<(), sqlx::Error>;
    #[allow(dead_code)]
    async fn delete(&self, pool: &PgPool) -> Result<(), sqlx::Error>;
}

pub async fn load_from_csv<T, F>(path: &str, pool: &PgPool, f: F) -> Result<(), io::Error>
where
    F: Fn(Vec<String>) -> T,
    T: ModelTrait + Send + 'static,
{
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)?;
    for result in rdr.records() {
        let record = result?;
        let record: Vec<String> = record.into_iter().map(|s| s.to_string()).collect();
        let model = f(record);
        model.create(pool).await.map_err(|e| {
            let error_message = format!("{}", e);
            io::Error::new(io::ErrorKind::Other, error_message)
        })?;
    }
    Ok(())
}

// Implement Display trait for sqlx::Error
impl fmt::Display for sqlx::Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            sqlx::Error::Database(e) => {
                writeln!(f, "Database({:?})", e)?;
            }
            _ => {
                writeln!(f, "{:?}", self)?;
            }
        }
        Ok(())
    }
}
