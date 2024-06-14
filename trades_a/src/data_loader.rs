// data_loader.rs
use std::collections::HashMap;
use mysql::Pool;
use serde::{Serialize, Deserialize};
use serde_with::{serde_as, DisplayFromStr};
use secrecy::SecretString;

use crate::factory::DataLoaderType;

pub trait DataLoader<T> {
    fn get_pool(&self) -> &Pool;
    fn load_data(&self) -> Result<Vec<T>, DataLoaderError>;
    fn get_type(&self) -> String;
    fn get_options(&self) -> HashMap<String, String>;
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct DataLoaderConfig {
    pub data_loader_type: DataLoaderType,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub username: String,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(skip_serializing)]
    pub password: SecretString,
    #[serde(default)]
    pub dataset: String,
    #[serde(default)]
    pub options: HashMap<String, String>,
}

#[derive(Debug)]
pub enum DataLoaderError {
    DatabaseError(String),
    FileError(String),
    UnknownSourceError(String),
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
            DataLoaderError::UnknownSourceError(msg) => write!(f, "Error occurred: {}", msg),
            // Add more error types as needed
        }
    }
}
