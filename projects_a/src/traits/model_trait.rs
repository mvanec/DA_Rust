use async_trait::async_trait;
use sqlx::PgPool;
use std::{fmt, io};

#[async_trait(?Send)]
pub trait ModelTrait {
    async fn create(&self, pool: &PgPool) -> Result<(), sqlx::Error>;
    #[allow(dead_code)]
    async fn delete(&self, pool: &PgPool) -> Result<(), sqlx::Error>;
}

// Custom error type that wraps sqlx::Error
#[derive(Debug)]
pub struct DatabaseError {
    pub error: sqlx::Error,
}

impl DatabaseError {
    pub fn new(error: sqlx::Error) -> Self {
        DatabaseError { error }
    }
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.error {
            sqlx::Error::Database(e) => {
                writeln!(f, "Database Error {{")?;
                writeln!(f, "    message: {}", e.as_ref().message())?;
                writeln!(f, "    code: {:?}", e.as_ref().code())?;
                writeln!(f, "}}")?;
            }
            _ => {
            writeln!(f, "{:?}", self.error)?;
            }
        }
        Ok(())
    }
}

// Usage
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
            let error = DatabaseError::new(e);
            io::Error::new(io::ErrorKind::Other, format!("{}", error))
        })?;
    }
    Ok(())
}
