use crate::data_loader::{DataLoader, DataLoaderConfig, DataLoaderError};
use crate::csv_data_loader::CsvDataLoader;
use crate::mysql_data_loader::MySqlDataLoader;


pub enum DataLoaderType {
    MySql,
    Csv,
    // Add more types as needed
}

pub struct TradeFactory {

}

impl TradeFactory {
    pub fn new(data_loader_type: DataLoaderType, config: DataLoaderConfig) -> Result<Box<dyn DataLoader>, DataLoaderError> {
        match data_loader_type {
            DataLoaderType::MySql => Ok(Box::new(MySqlDataLoader::new(config)?)),
            DataLoaderType::Csv => Ok(Box::new(CsvDataLoader::new(config)?)),
            // Add more cases as needed
        }
    }
}
