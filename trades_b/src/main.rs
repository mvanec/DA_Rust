// main.rs
#![allow(unused_imports)]
#![allow(dead_code)]
mod models;
mod factory;
mod data_loader;
mod csv_data_loader;
mod mysql_data_loader;

use models::*;
use factory::*;
use csv_data_loader::CsvDataLoader;
use mysql_data_loader::MySqlDataLoader;

use std::fmt;

impl fmt::Display for Trade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "| {:^5} | {:^6} | {:^10} | {:^5} | {:^5} | {:^10} |",
                 self.trade_id, self.symbol, self.open_date, self.broker_id, self.exchange_id, self.realized_gain.unwrap_or(0.0))?;

        for execution in &self.executions {
            writeln!(f, "    | {:^5} | {:^19} | {:^6} | {:^6} | {:^10} | {:^10} |",
                     execution.execution_id, execution.execution_date_time, execution.quantity, execution.order_price, execution.commission, execution.fees)?;

            for option in &execution.options {
                writeln!(f, "        | {:^5} | {:^10} | {:^6} | {:^10} | {:^5} | {:^10} | {:^20} |",
                         option.option_id, option.expiration, option.quantity, option.strike, option.option_type, option.premium, option.opra)?;
            }
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    // let csv_data_loader = CsvDataLoader::new(
    //     "W:\\DataAnnotation\\Rust\\test_data\\trades.csv".to_string(),
    //     "W:\\DataAnnotation\\Rust\\test_data\\trade_executions.csv".to_string(),
    //     "W:\\DataAnnotation\\Rust\\test_data\\options_details.csv".to_string()
    // );
    // let trade_factory = TradeFactory::new(Box::new(csv_data_loader));

    let mysql_data_loader = MySqlDataLoader::new(
        "localhost:3306".to_string(),
        "data".to_string(),
        "dataannotation".to_string(),
        "dataannotation".to_string(),
    ).await;

    let trade_factory = TradeFactory::new(mysql_data_loader);

    let trades = trade_factory.load_trades().await;

    for trade in trades {
        println!("{}", trade);
    }
}
