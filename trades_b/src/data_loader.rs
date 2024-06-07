// data_loader.rs
use crate::models::*;
use std::error::Error;

pub trait DataLoader {
    fn load_trades(&self) -> Result<Vec<Trade>, Box<dyn Error>>;
    fn load_trade_executions(&self) -> Result<Vec<TradeExecution>, Box<dyn Error>>;
    fn load_option_details(&self) -> Result<Vec<OptionDetail>, Box<dyn Error>>;
}

// csv_data_loader.rs
use crate::data_loader::DataLoader;
use crate::models::*;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;
use csv::ReaderBuilder;

pub struct CsvDataLoader {
    trades_path: String,
    trade_executions_path: String,
    option_details_path: String,
}

impl CsvDataLoader {
    pub fn new(trades_path: String, trade_executions_path: String, option_details_path: String) -> Self {
        Self {
            trades_path,
            trade_executions_path,
            option_details_path,
        }
    }
}

impl DataLoader for CsvDataLoader {
    fn load_trades(&self) -> Result<Vec<Trade>, Box<dyn Error>> {
        let file = File::open(Path::new(&self.trades_path))?;
        let reader = ReaderBuilder::new().has_headers(true).from_reader(BufReader::new(file));
        let mut trades: Vec<Trade> = Vec::new();

        for result in reader.records() {
            let record = result?;
            trades.push(Trade {
                trade_id: record[0].parse()?,
                symbol: record[1].to_string(),
                open_date: record[2].to_string(),
                close_date: if record[3].is_empty() { None } else { Some(record[3].to_string()) },
                broker_id: record[4].parse()?,
                exchange_id: record[5].parse()?,
                realized_gain: if record[6].is_empty() { None } else { Some(record[6].parse()?) },
                executions: Vec::new(),
            });
        }

        Ok(trades)
    }

    fn load_trade_executions(&self) -> Result<Vec<TradeExecution>, Box<dyn Error>> {
        let file = File::open(Path::new(&self.trade_executions_path))?;
        let reader = ReaderBuilder::new().has_headers(true).from_reader(BufReader::new(file));
        let mut trade_executions: Vec<TradeExecution> = Vec::new();

        for result in reader.records() {
            let record = result?;
            trade_executions.push(TradeExecution {
                execution_id: record[0].parse()?,
                trade_id: record[1].parse()?,
                execution_date_time: record[2].to_string(),
                spread: record[3].to_string(),
                quantity: record[4].parse()?,
                position_effect: record[5].to_string(),
                order_price: record[6].parse()?,
                fill_price: record[7].parse()?,
                commission: record[8].parse()?,
                fees: record[9].parse()?,
                reference_number: record[10].to_string(),
                options: Vec::new(),
            });
        }

        Ok(trade_executions)
    }

    fn load_option_details(&self) -> Result<Vec<OptionDetail>, Box<dyn Error>> {
        let file = File::open(Path::new(&self.option_details_path))?;
        let reader = ReaderBuilder::new().has_headers(true).from_reader(BufReader::new(file));
        let mut option_details: Vec<OptionDetail> = Vec::new();

        for result in reader.records() {
            let record = result?;
            option_details.push(OptionDetail {
                option_id: record[0].parse()?,
                execution_id: record[1].parse()?,
                expiration: record[2].to_string(),
                strike: record[3].parse()?,
                option_type: record[4].to_string(),
                quantity: record[5].parse()?,
                premium: record[6].parse()?,
                opra: record[7].to_string(),
            });
        }

        Ok(option_details)
    }
}
