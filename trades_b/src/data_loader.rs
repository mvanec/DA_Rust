// data_loader.rs
use std::collections::HashMap;
use serde::Deserialize;
use crate::factory::DataLoaderType;
use crate::models::*;

#[derive(Deserialize)]
pub struct DataLoaderConfig {
    pub data_loader_type: DataLoaderType,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub dataset: String,
    #[serde(default)]
    pub options: HashMap<String, String>,
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

impl From<mysql::Error> for DataLoaderError {
    fn from(e: mysql::Error) -> Self {
        DataLoaderError::DatabaseError(e.to_string())
    }
}

impl From<csv::Error> for DataLoaderError {
    fn from(e: csv::Error) -> Self {
        DataLoaderError::FileError(e.to_string())
    }
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
