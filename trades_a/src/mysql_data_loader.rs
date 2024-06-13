// mysql_data_loader.rs
use crate::data_loader::{DataLoader, DataLoaderConfig, DataLoaderError};
use crate::models::*;
use mysql::prelude::*;
use mysql::{Opts, OptsBuilder, Pool};

pub struct MySqlDataLoader {
    pool: Pool,
}

impl MySqlDataLoader {
    pub fn new(config: DataLoaderConfig) -> Result<Self, DataLoaderError> {
        let opts: Opts = OptsBuilder::new()
            .ip_or_hostname(Some(config.source))
            .user(Some(config.username))
            .pass(Some(config.password))
            .db_name(Some(config.dataset))
            .into();

        let pool = Pool::new(opts)?;
        Ok(Self { pool })
    }
}

impl DataLoader for MySqlDataLoader {
    fn load_trades(&self) -> Result<Vec<Trade>, DataLoaderError> {
        let mut conn = self.pool.get_conn()?;

        let mut trades: Vec<Trade> = conn.query_map(
            "SELECT TradeID, Symbol, OpenDate, CloseDate, BrokerID, ExchangeID, RealizedGain FROM trades",
            |(trade_id, symbol, open_date, close_date, broker_id, exchange_id, realized_gain)| Trade {
                trade_id, symbol, open_date, close_date, broker_id, exchange_id, realized_gain, executions: Vec::new(),
            },
        )?;

        let trade_executions: Vec<TradeExecution> = conn.query_map(
            "SELECT ExecutionID, TradeID, ExecutionDateTime, Spread, Quantity, PositionEffect, OrderPrice, FillPrice, Commission, Fees, ReferenceNumber FROM tradeexecutions",
            |(execution_id, trade_id, execution_date_time, spread, quantity, position_effect, order_price, fill_price, commission, fees, reference_number)| TradeExecution {
                execution_id, trade_id, execution_date_time, spread, quantity, position_effect, order_price,
                fill_price, commission, fees, reference_number, options: Vec::new(),
            },
        )?;

        let options_details: Vec<OptionDetail> = conn.query_map(
            "SELECT OptionID, ExecutionID, Expiration, Strike, Type, Quantity, Premium, Opra FROM optionsdetails",
            |(option_id, execution_id, expiration, strike, option_type, quantity, premium, opra)| OptionDetail {
                option_id,  execution_id, expiration, strike, option_type, quantity, premium, opra,
            },
        )?;

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

        Ok(trades)
    }
}
