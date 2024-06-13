// csv_data_loader.rs
use crate::data_loader::{DataLoader, DataLoaderConfig, DataLoaderError};
use crate::models::*;
use csv::ReaderBuilder;

pub struct CsvDataLoader {
    trades_path: String,
    trade_executions_path: String,
    options_details_path: String,
}

impl CsvDataLoader {
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
        })
    }
}

impl DataLoader for CsvDataLoader {
    fn load_trades(&self) -> Result<Vec<Trade>, DataLoaderError> {
        let mut trades = Vec::new();

        let mut trades_reader = ReaderBuilder::new()
            .from_path(&self.trades_path)
            .map_err(|e| DataLoaderError::from(e))?;

        for trade in trades_reader.deserialize() {
            let trade: Trade = trade.map_err(|e| DataLoaderError::from(e))?;
            trades.push(trade);
        }

        let mut trade_executions_reader = ReaderBuilder::new()
            .from_path(&self.trade_executions_path)
            .map_err(|e| DataLoaderError::from(e))?;

        let mut trade_executions: Vec<TradeExecution> = Vec::new();

        for trade_execution in trade_executions_reader.deserialize() {
            let trade_execution: TradeExecution =
                trade_execution.map_err(|e| DataLoaderError::from(e))?;
            trade_executions.push(trade_execution);
        }

        let mut options_details_reader = ReaderBuilder::new()
            .from_path(&self.options_details_path)
            .map_err(|e| DataLoaderError::from(e))?;

        let mut options_details: Vec<OptionDetail> = Vec::new();

        for option_detail in options_details_reader.deserialize() {
            let option_detail: OptionDetail =
                option_detail.map_err(|e| DataLoaderError::from(e))?;
            options_details.push(option_detail);
        }

        for trade in &mut trades {
            trade.executions = trade_executions
                .iter()
                .filter(|te| te.trade_id == trade.trade_id)
                .cloned()
                .collect();

            for execution in &mut trade.executions {
                execution.options = options_details
                    .iter()
                    .filter(|od| od.execution_id == execution.execution_id)
                    .cloned()
                    .collect();
            }
        }

        Ok(trades)
    }
}
