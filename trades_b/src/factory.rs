use crate::data_loader::{DataLoader, DataLoaderConfig, DataLoaderError};
use crate::models::*;
use crate::csv_data_loader::CsvDataLoader;
use crate::mysql_data_loader::MySqlDataLoader;


pub enum DataLoaderType {
    MySql,
    Csv,
    // Add more types as needed
}

pub struct TradeFactory {
    data_loader: Box<dyn DataLoader>,
}

impl TradeFactory {
    pub fn new(data_loader_type: DataLoaderType, config: DataLoaderConfig) -> Result<Self, DataLoaderError> {
        let data_loader: Box<dyn DataLoader> = match data_loader_type {
            DataLoaderType::MySql => Box::new(MySqlDataLoader::new(config)?),
            DataLoaderType::Csv => Box::new(CsvDataLoader::new(config)?),
            // Add more cases as needed
        };

        Ok(Self { data_loader })
    }

    pub fn load_trades(&self) -> Result<Vec<Trade>, DataLoaderError> {
        self.data_loader.load_trades()
    }
}