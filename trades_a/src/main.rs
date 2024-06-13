// main.rs
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

mod csv_data_loader;
mod data_loader;
mod factory;
mod models;
mod mysql_data_loader;

use data_loader::DataLoaderConfig;
use factory::*;

fn load_config_from_file(path: &str) -> Result<Vec<DataLoaderConfig>, std::io::Error> {
    let mut file = File::open(Path::new(path))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let configs: Vec<DataLoaderConfig> = match serde_json::from_str(&contents) {
        Ok(configs) => configs,
        Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, err)),
    };
    Ok(configs)
}

fn run_data_load(config: DataLoaderConfig) {
    let data_loader = match TradeFactory::new(config.data_loader_type, config) {
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
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} config_file", args[0]);
        return;
    }

    let config_file = &args[1];
    let configs = match load_config_from_file(config_file) {
        Ok(configs) => configs,
        Err(err) => {
            eprintln!("Failed to load config: {}", err);
            return;
        }
    };

    for config in configs {
        run_data_load(config);
    }
}
