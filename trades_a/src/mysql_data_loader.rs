// mysql_data_loader.rs
use crate::data_loader::DataLoader;
use crate::models::*;
use mysql::prelude::*;

pub struct MySqlDataLoader {
    pool: mysql::Pool,
}

impl MySqlDataLoader {
    pub fn new(url: String, user: String, password: String, db: String) -> Self {
        let url = format!("mysql://{}:{}@{}/{}", user, password, url, db);
        // println!("URL: {}", url);
        let opts = mysql::Opts::from_url(&url).unwrap();
        let pool = mysql::Pool::new(opts).unwrap();
        Self { pool }
    }
}

impl DataLoader for MySqlDataLoader {
    fn load_trades(&self) -> Vec<Trade> {
        let mut conn = self.pool.get_conn().unwrap();

        let mut trades: Vec<Trade> = conn.query_map(
            "SELECT TradeID, Symbol, OpenDate, CloseDate, BrokerID, ExchangeID, RealizedGain FROM trades",
            |(trade_id, symbol, open_date, close_date, broker_id, exchange_id, realized_gain)| Trade {
                trade_id,
                symbol,
                open_date,
                close_date,
                broker_id,
                exchange_id,
                realized_gain,
                executions: Vec::new(),
            },
        ).unwrap();

        let trade_executions: Vec<TradeExecution> = conn.query_map(
            "SELECT ExecutionID, TradeID, ExecutionDateTime, Spread, Quantity, PositionEffect, OrderPrice, FillPrice, Commission, Fees, ReferenceNumber FROM trade_executions",
            |(execution_id, trade_id, execution_date_time, spread, quantity, position_effect, order_price, fill_price, commission, fees, reference_number)| TradeExecution {
                execution_id,
                trade_id,
                execution_date_time,
                spread,
                quantity,
                position_effect,
                order_price,
                fill_price,
                commission,
                fees,
                reference_number,
                options: Vec::new(),
            },
        ).unwrap();

        let options_details: Vec<OptionDetail> = conn.query_map(
            "SELECT OptionID, ExecutionID, Expiration, Strike, Type, Quantity, Premium, Opra FROM options_details",
            |(option_id, execution_id, expiration, strike, option_type, quantity, premium, opra)| OptionDetail {
                option_id,
                execution_id,
                expiration,
                strike,
                option_type,
                quantity,
                premium,
                opra,
            },
        ).unwrap();

        for trade in &mut trades {
            trade.executions = trade_executions
                .iter()
                .filter(|te| te.trade_id == trade.trade_id)
                .cloned()
                .collect();

            for execution in &mut trade.executions {
                execution.options = options_details
                    .iter()
                    .filter(|od| od.execution_id == execution.execution_id)
                    .cloned()
                    .collect();
            }
        }

        trades
    }
}
