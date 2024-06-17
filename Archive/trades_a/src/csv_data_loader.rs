// csv_data_loader.rs
use std::collections::HashMap;
use std::marker::PhantomData;

use crate::data_loader::{DataLoader, DataLoaderConfig, DataLoaderError};
use crate::loadable::Loadable;

pub struct CsvDataLoader<T> {
    trades_path: String,
    trade_executions_path: String,
    options_details_path: String,
    _marker: PhantomData<T>,
}

impl<T> CsvDataLoader<T> {
    pub fn new(config: DataLoaderConfig) -> Result<Self, DataLoaderError> {
        Ok(Self {
            trades_path: config
                .options
                .get("trades_file")
                .unwrap_or(&"".to_string())
                .clone(),
            trade_executions_path: config
                .options
                .get("trade_executions_file")
                .unwrap_or(&"".to_string())
                .clone(),
            options_details_path: config
                .options
                .get("options_details_file")
                .unwrap_or(&"".to_string())
                .clone(),
            _marker: PhantomData
        })
    }
}

impl<T> DataLoader<T> for CsvDataLoader<T>
where
    T: Loadable,
{

    fn get_pool(&self) -> &mysql::Pool {
        todo!()
    }

    fn load_data(&self) -> Result<Vec<T>, DataLoaderError> {
        T::load(self)
    }

    fn get_type(&self) -> String {
        return "CSV".to_string();
    }

    fn get_options(&self) -> HashMap<String, String> {
        let mut options: HashMap<String, String> = HashMap::new();
        options.insert("trades_path".to_string(), self.trades_path.clone());
        options.insert("trade_executions_path".to_string(), self.trade_executions_path.clone());
        options.insert("options_details_path".to_string(), self.options_details_path.clone());
        options
    }
}
