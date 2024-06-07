// main.rs
mod models;
mod factory;

use models::*;
use factory::*;

use std::fmt;

impl fmt::Display for Trade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "| {:^10} | {:^10} | {:^10} | {:^10} | {:^10} | {:^10} |",
                 self.trade_id, self.symbol, self.open_date, self.broker_id, self.exchange_id, self.realized_gain.unwrap_or(0.0))?;

        for execution in &self.executions {
            writeln!(f, "| {:^10} | {:^10} | {:^10} | {:^10} | {:^10} | {:^10} |",
                     " ", " ", " ", " ", " ", " ")?;

            writeln!(f, "| {:^10} | {:^10} | {:^10} | {:^10} | {:^10} | {:^10} |",
                     execution.execution_id, execution.execution_date_time, execution.quantity, execution.order_price, execution.fill_price, execution.commission)?;

            for option in &execution.options {
                writeln!(f, "| {:^10} | {:^10} | {:^10} | {:^10} | {:^10} | {:^10} |",
                         " ", " ", " ", " ", " ", " ")?;

                writeln!(f, "| {:^10} | {:^10} | {:^10} | {:^10} | {:^10} | {:^10} |",
                         option.option_id, option.expiration, option.quantity, option.strike, option.option_type, option.premium)?;
            }
        }

        Ok(())
    }
}

fn main() {
    let trades = load_test_data();

    for trade in trades {
        println!("{}", trade);
    }
}
