// mysql_data_loader.rs
use crate::data_loader::DataLoader;
use crate::models::*;
use mysql::prelude::*;

pub struct MysqlDataLoader {
    pool: mysql::Pool,
}

impl MysqlDataLoader {
    pub fn new(url: &str, user: &str, password: &str, db: &str) -> Self {
        let pool = mysql::Pool::new(format!("mysql://{}:{}@{}/{}", user, password, url, db)).unwrap();
        Self { pool }
    }
}

impl DataLoader for MysqlDataLoader {
    fn load_trades(&self) -> Vec<Trade> {
        let mut conn = self.pool.get_conn().unwrap();

        let trades: Vec<Trade> = conn
            .query_map(
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
            )
            .unwrap();

        let trade_executions: Vec<TradeExecution> = conn
            .query_map(
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
            )
            .unwrap();

        let options_details: Vec<OptionDetail> = conn
            .query_map(
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
            )
            .unwrap();

        let mut trades_map: std::collections::HashMap<i32, &mut Trade> = trades.iter_mut().map(|trade| (trade.trade_id, trade)).collect();

        for trade_execution in trade_executions {
            if let Some(trade) = trades_map.get_mut(&trade_execution.trade_id) {
                trade.executions.push(trade_execution);
            }
        }

        for trade in trades_map.values_mut() {
            for execution in &mut trade.executions {
                execution.options = options_details.iter().filter(|od| od.execution_id == execution.execution_id).cloned().collect();
            }
        }

        trades
    }
}
