// main.rs
mod models;
mod factory;

use models::*;
use factory::*;

use std::fmt;

// main.rs
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

fn main() {
    let trades = load_test_data();

    for trade in trades {
        println!("{}", trade);
    }
}
