// main.rs
// #![allow(unused_imports)]
// #![allow(dead_code)]
use std::collections::HashMap;
mod csv_data_loader;
mod data_loader;
mod factory;
mod models;
mod mysql_data_loader;

use data_loader::DataLoaderConfig;
use factory::*;

fn run_csv_load() {
    let mut config = DataLoaderConfig {
        source: "".to_string(),
        username: "".to_string(),
        password: "".to_string(),
        dataset: "".to_string(),
        options: HashMap::new(),
    };
    config.options.insert(
        "trades_file".to_string(),
        "W:\\DataAnnotation\\Rust\\test_data\\trades.csv".to_string(),
    );
    config.options.insert(
        "trade_executions_file".to_string(),
        "W:\\DataAnnotation\\Rust\\test_data\\trade_executions.csv".to_string(),
    );
    config.options.insert(
        "options_details_file".to_string(),
        "W:\\DataAnnotation\\Rust\\test_data\\options_details.csv".to_string(),
    );
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
        source: "127.0.0.1".to_string(),
        username: "data".to_string(),
        password: "dataannotation".to_string(),
        dataset: "dataannotation".to_string(),
        options: HashMap::new(),
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
