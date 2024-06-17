use serde::{Serialize, Deserialize};
use crate::data_loader::{DataLoader, DataLoaderConfig, DataLoaderError};
use crate::loadable::Loadable;
use crate::mysql_data_loader::MySqlDataLoader;
use crate::csv_data_loader::CsvDataLoader;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum DataLoaderType {
    #[serde(rename = "Csv")]
    Csv,
    #[serde(rename = "MySql")]
    MySql,
}

pub struct TradeFactory {

}

impl TradeFactory {
    pub fn new<T>(data_loader_type: DataLoaderType, config: DataLoaderConfig) -> Result<Box<dyn DataLoader<T>>, DataLoaderError>
    where
        T: Loadable + 'static,
    {
        match data_loader_type {
            DataLoaderType::MySql => Ok(Box::new(MySqlDataLoader::<T>::new(config)?)),
            DataLoaderType::Csv => Ok(Box::new(CsvDataLoader::<T>::new(config)?)),
            // Add more cases as needed
        }
    }
}
