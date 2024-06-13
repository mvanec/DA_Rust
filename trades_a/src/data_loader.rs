// data_loader.rs
use std::error::Error;
use crate::models::*;

pub struct DataLoaderConfig {
    pub url: String,
    pub user: String,
    pub password: String,
    pub db: String,
}

pub trait DataLoader {
    fn load_trades(&self) -> Result<Vec<Trade>, DataLoaderError>;
}

#[derive(Debug)]
pub enum DataLoaderError {
    DatabaseError(String),
    FileError(String),
    // Add more error types as needed
}

impl std::error::Error for DataLoaderError {}

impl std::fmt::Display for DataLoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DataLoaderError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            DataLoaderError::FileError(msg) => write!(f, "File error: {}", msg),
            // Add more error types as needed
        }
    }
}
