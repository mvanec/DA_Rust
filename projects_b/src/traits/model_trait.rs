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

// Implement Display trait for DatabaseError
impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.error {
            sqlx::Error::Database(e) => {
                writeln!(f, "Database(PgDatabaseError {{")?;
                writeln!(f, "    severity: {},", e.severity())?;
                writeln!(f, "    code: {},", e.code())?;
                writeln!(f, "    message: {},", e.message())?;
                if let Some(detail) = e.detail() {
                    writeln!(f, "    detail: Some({}),", detail)?;
                }
                if let Some(hint) = e.hint() {
                    writeln!(f, "    hint: Some({}),", hint)?;
                }
                if let Some(position) = e.position() {
                    writeln!(f, "    position: Some({}),", position)?;
                }
                if let Some(where_) = e.where_() {
                    writeln!(f, "    where: Some({}),", where_)?;
                }
                if let Some(schema) = e.schema() {
                    writeln!(f, "    schema: Some({}),", schema)?;
                }
                if let Some(table) = e.table() {
                    writeln!(f, "    table: Some({}),", table)?;
                }
                if let Some(column) = e.column() {
                    writeln!(f, "    column: Some({}),", column)?;
                }
                if let Some(data_type) = e.data_type() {
                    writeln!(f, "    data_type: Some({}),", data_type)?;
                }
                if let Some(constraint) = e.constraint() {
                    writeln!(f, "    constraint: Some({}),", constraint)?;
                }
                if let Some(file) = e.file() {
                    writeln!(f, "    file: Some({}),", file)?;
                }
                if let Some(line) = e.line() {
                    writeln!(f, "    line: Some({}),", line)?;
                }
                if let Some(routine) = e.routine() {
                    writeln!(f, "    routine: Some({}),", routine)?;
                }
                writeln!(f, "}})")?;
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
