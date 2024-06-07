// main.rs
mod models;
mod factory;
mod data_loader;
mod csv_data_loader;

use models::*;
use factory::*;
use csv_data_loader::CsvDataLoader;

use std::fmt;

impl fmt::Display for Trade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "| {:^5} | {:^6} | {:^10} | {:^5} | {:^5} | {:^10} |",
                 self.TradeID, self.Symbol, self.OpenDate, self.BrokerID, self.ExchangeID, self.RealizedGain.unwrap_or(0.0))?;

        for execution in &self.executions {
            writeln!(f, "    | {:^5} | {:^19} | {:^6} | {:^6} | {:^10} | {:^10} |",
                     execution.ExecutionID, execution.ExecutionDateTime, execution.Quantity, execution.OrderPrice, execution.Commission, execution.Fees)?;

            for option in &execution.options {
                writeln!(f, "        | {:^5} | {:^10} | {:^6} | {:^10} | {:^5} | {:^10} | {:^20} |",
                         option.OptionID, option.Expiration, option.Quantity, option.Strike, option.Type, option.Premium, option.Opra)?;
            }
        }

        Ok(())
    }
}

fn main() {
    let csv_data_loader = CsvDataLoader::new(
        "W:\\DataAnnotation\\Rust\\test_data\\trades.csv".to_string(),
        "W:\\DataAnnotation\\Rust\\test_data\\trade_executions.csv".to_string(),
        "W:\\DataAnnotation\\Rust\\test_data\\options_details.csv".to_string()
    );

    let trade_factory = TradeFactory::new(Box::new(csv_data_loader));

    let trades = trade_factory.load_trades();

    for trade in trades {
        println!("{}", trade);
    }
}
