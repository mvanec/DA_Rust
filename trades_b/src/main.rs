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

// Modules for data loading and models
mod data_loader;
mod factory;
mod models;
mod mysql_data_loader;
mod csv_data_loader;
pub mod loadable;

// Import necessary types
use data_loader::DataLoaderConfig;
use models::Trade;
use factory::*;

// Struct to hold logging configuration
#[derive(Serialize, Deserialize)]
struct LoggingConfig {
    log_file: String,
    log_level: String,
}

// Struct to hold application configuration
#[derive(Serialize, Deserialize)]
struct AppConfig {
    logging: LoggingConfig,
    data_loaders: Vec<DataLoaderConfig>,
}

// Load application configuration from a file
fn load_config_from_file(path: &str) -> Result<AppConfig, Box<dyn std::error::Error>> {
    // Check if the file exists
    let config_path = Path::new(path);
    if !config_path.exists() {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Config file not found")));
    }
    // Read the file contents
    let config_str = fs::read_to_string(config_path)?;
    // Deserialize the configuration
    let config: AppConfig = serde_json::from_str(&config_str).unwrap();

    Ok(config)
}

// Run a data load using the given configuration
fn run_data_load(config: DataLoaderConfig) {
    // Create a data loader instance
    let data_loader = match TradeFactory::new::<Trade>(config.data_loader_type, config) {
        Ok(loader) => loader,
        Err(err) => {
            log::error!("Error creating data loader with: {}", err);
            return;
        }
    };

    // Load data using the data loader
    match data_loader.load_data() {
        Ok(trades) => {
            // Print the loaded trades
            for trade in trades {
                println!("{}", trade);
            }
        }
        Err(err) => {
            log::error!("Error loading trades: {}", err);
        }
    }
}

// Initialize logging using the given configuration
fn initialize_logging(logging_config: &LoggingConfig) -> Config {
    // Create a file appender
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} {l} - {m}\n")))
        .build(&logging_config.log_file)
        .unwrap();

    // Determine the log level
    let log_level_filter = match logging_config.log_level.to_lowercase().as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };

    // Create the logging configuration
    let log_config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(log_level_filter)).unwrap();

    log_config
}

// Create a logger instance using the given configuration
fn create_logger(log_config: Config) -> Result<crate::Handle, SetLoggerError> {
    log4rs::init_config(log_config)
}

fn main() {
    // Bootstrap logging
    let bootstrap_logging_config = LoggingConfig {
        log_file: "bootstrap.log".to_string(),
        log_level: "info".to_string(),
    };
    let cfg = initialize_logging(&bootstrap_logging_config);
    let handle =  match create_logger(cfg) {
        Ok(handle) => handle,
        Err(err) => {
            eprintln!("Error bootstrapping logger: {:?}", err);
            process::exit(-1);
        }
    };

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} config_file", args[0]);
        process::exit(1);
    }
    let config_file = &args[1];

    // Load application configuration
    let config = match load_config_from_file(config_file) {
        Ok(config) => config,
        Err(err) => {
            log::error!("Error loading config file: {}", err);
            process::exit(1);
        }
    };

    // Reconfigure logging using the loaded configuration
    let cfg = initialize_logging(&config.logging);
    handle.set_config(cfg);

    // Run data loads using the loaded configuration
    for data_loader_config in config.data_loaders {
        run_data_load(data_loader_config);
    }
}
