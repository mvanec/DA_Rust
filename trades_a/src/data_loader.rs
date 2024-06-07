// data_loader.rs
use crate::models::*;

pub trait DataLoader {
    fn load_trades(&self) -> Vec<Trade>;
}

// csv_data_loader.rs
use crate::data_loader::DataLoader;
use crate::models::*;
use csv::ReaderBuilder;

pub struct CsvDataLoader {
    trades_path: String,
    trade_executions_path: String,
    options_details_path: String,
}

impl CsvDataLoader {
    pub fn new(trades_path: String, trade_executions_path: String, options_details_path: String) -> Self {
        Self {
            trades_path,
            trade_executions_path,
            options_details_path,
        }
    }
}

impl DataLoader for CsvDataLoader {
    fn load_trades(&self) -> Vec<Trade> {
        let mut trades = Vec::new();

        let mut trades_reader = ReaderBuilder::new()
            .from_path(&self.trades_path)
            .unwrap();

        for trade in trades_reader.deserialize() {
            let trade: Trade = trade.unwrap();
            trades.push(trade);
        }

        let mut trade_executions_reader = ReaderBuilder::new()
            .from_path(&self.trade_executions_path)
            .unwrap();

        let mut trade_executions: Vec<TradeExecution> = Vec::new();

        for trade_execution in trade_executions_reader.deserialize() {
            let trade_execution: TradeExecution = trade_execution.unwrap();
            trade_executions.push(trade_execution);
        }

        let mut options_details_reader = ReaderBuilder::new()
            .from_path(&self.options_details_path)
            .unwrap();

        let mut options_details: Vec<OptionDetail> = Vec::new();

        for option_detail in options_details_reader.deserialize() {
            let option_detail: OptionDetail = option_detail.unwrap();
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

        trades
    }
}
