// main.rs
#![allow(unused_imports)]
#![allow(dead_code)]
mod csv_data_loader;
mod data_loader;
mod factory;
mod models;
mod mysql_data_loader;

use data_loader::{DataLoaderConfig, DataLoader, DataLoaderError};
use csv_data_loader::CsvDataLoader;
use factory::*;
use models::*;
use mysql_data_loader::MySqlDataLoader;

use std::fmt;

impl fmt::Display for Trade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "| {:^5} | {:^6} | {:^10} | {:^5} | {:^5} | {:^10} |",
            self.trade_id, self.symbol, self.open_date, self.broker_id, self.exchange_id, self.realized_gain.unwrap_or(0.0)
        )?;

        for execution in &self.executions {
            writeln!(
                f,
                "    | {:^5} | {:^19} | {:^6} | {:^6} | {:^10} | {:^10} |",
                execution.execution_id, execution.execution_date_time, execution.quantity, execution.order_price,
                execution.commission, execution.fees
            )?;

            for option in &execution.options {
                writeln!(
                    f,
                    "        | {:^5} | {:^10} | {:^6} | {:^10} | {:^5} | {:^10} | {:^20} |",
                    option.option_id, option.expiration, option.quantity, option.strike,
                    option.option_type, option.premium, option.opra
                )?;
            }
        }

        Ok(())
    }
}

fn run_csv_load() {
    let config = DataLoaderConfig {
        url: "W:\\DataAnnotation\\Rust\\test_data\\trades.csv".to_string(),
        user: "W:\\DataAnnotation\\Rust\\test_data\\trade_executions.csv".to_string(),
        password: "W:\\DataAnnotation\\Rust\\test_data\\options_details.csv".to_string(),
        db: "".to_string(),
    };

    let data_loader = match TradeFactory::new(DataLoaderType::Csv, config) {
        Ok(loader) => loader,
        Err(err) => {
            eprintln!("Error creating data loader with: {}", err);
            return;
        }
    };

    match data_loader.load_trades() {
        Ok(trades) => {
            for trade in trades {
                println!("{}", trade);
            }
        }
        Err(err) => {
            eprintln!("Error loading trades: {}", err);
        }
    }
}

fn run_mysql_load() {
    let config = DataLoaderConfig {
        url: "127.0.0.1".to_string(),
        user: "data".to_string(),
        password: "dataannotation".to_string(),
        db: "dataannotation".to_string(),
    };

    let data_loader = match TradeFactory::new(DataLoaderType::MySql, config) {
        Ok(loader) => loader,
        Err(err) => {
            eprintln!("Error creating data loader with: {}", err);
            return;
        }
    };

    match data_loader.load_trades() {
        Ok(trades) => {
            for trade in trades {
                println!("{}", trade);
            }
        }
        Err(err) => {
            eprintln!("Error loading trades: {}", err);
        }
    }
}

fn main() {
    run_mysql_load();
    run_csv_load();
}
