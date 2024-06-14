// main.rs
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use std::process;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use log::SetLoggerError;
use log4rs::Handle;
use log::LevelFilter;

mod csv_data_loader;
mod data_loader;
mod factory;
mod models;
mod mysql_data_loader;

use data_loader::DataLoaderConfig;
use factory::*;

#[derive(Serialize, Deserialize)]
struct LoggingConfig {
    log_file: String,
}

#[derive(Serialize, Deserialize)]
struct AppConfig {
    logging: LoggingConfig,
    data_loaders: Vec<DataLoaderConfig>,
}

fn load_config_from_file(path: &str) -> Result<AppConfig, Box<dyn std::error::Error>> {
    let config_path = Path::new(path);
    if !config_path.exists() {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Config file not found")));
    }
    let config_str = fs::read_to_string(config_path)?;
    let config: AppConfig = serde_json::from_str(&config_str)?;
    Ok(config)
}

fn run_data_load(config: DataLoaderConfig) {
    let data_loader = match TradeFactory::new(config.data_loader_type, config) {
        Ok(loader) => loader,
        Err(err) => {
            log::error!("Error creating data loader with: {}", err);
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
            log::error!("Error loading trades: {}", err);
        }
    }
}

fn initialize_logging(log_file: &String) -> Config {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} {l} - {m}\n")))
        .build(log_file)
        .unwrap();

    let log_config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info)).unwrap();

    log_config
}

fn create_logger(log_config: Config) -> Result<crate::Handle, SetLoggerError> {
    log4rs::init_config(log_config)
}

fn main() {
    // Bootstrap logging
    let cfg = initialize_logging(&"bootstrap.log".to_string());
    let handle =  match create_logger(cfg) {
        Ok(handle) => handle,
        Err(err) => {
            eprintln!("Error bootstrapping logger: {:?}", err);
            process::exit(-1);
        }
    };

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} config_file", args[0]);
        process::exit(1);
    }
    let config_file = &args[1];

    let config = match load_config_from_file(config_file) {
        Ok(config) => config,
        Err(err) => {
            log::error!("Error loading config file: {}", err);
            process::exit(1);
        }
    };

    let cfg = initialize_logging(&config.logging.log_file);
    handle.set_config(cfg);

    for data_loader_config in config.data_loaders {
        run_data_load(data_loader_config);
    }
}
