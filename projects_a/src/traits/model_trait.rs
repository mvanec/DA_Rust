use sqlx::PgPool;
use std::io;

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
        let model = f(record);
        model.create(pool).await.unwrap();
    }
    Ok(())
}
