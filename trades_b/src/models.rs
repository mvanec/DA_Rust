// models.rs
use serde::Deserialize;
use std::fmt;
use mysql::prelude::Queryable;
use csv::ReaderBuilder;

use crate::loadable::Loadable;
use crate::data_loader::{DataLoader, DataLoaderError};

#[derive(Debug, Deserialize, Clone)]
pub struct Trade {
    #[serde(rename = "TradeID")]
    pub trade_id: i32,
    #[serde(rename = "Symbol")]
    pub symbol: String,
    #[serde(rename = "OpenDate")]
    pub open_date: String,
    #[serde(rename = "CloseDate")]
    pub close_date: Option<String>,
    #[serde(rename = "BrokerID")]
    pub broker_id: i32,
    #[serde(rename = "ExchangeID")]
    pub exchange_id: i32,
    #[serde(rename = "RealizedGain")]
    pub realized_gain: Option<f64>,
    #[serde(default)]
    pub executions: Vec<TradeExecution>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TradeExecution {
    #[serde(rename = "ExecutionID")]
    pub execution_id: i32,
    #[serde(rename = "TradeID")]
    pub trade_id: i32,
    #[serde(rename = "ExecutionDateTime")]
    pub execution_date_time: String,
    #[serde(rename = "Spread")]
    pub spread: String,
    #[serde(rename = "Quantity")]
    pub quantity: i32,
    #[serde(rename = "PositionEffect")]
    pub position_effect: String,
    #[serde(rename = "OrderPrice")]
    pub order_price: f64,
    #[serde(rename = "FillPrice")]
    pub fill_price: f64,
    #[serde(rename = "Commission")]
    pub commission: f64,
    #[serde(rename = "Fees")]
    pub fees: f64,
    #[serde(rename = "ReferenceNumber")]
    pub reference_number: String,
    #[serde(default)]
    pub options: Vec<OptionDetail>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OptionDetail {
    #[serde(rename = "OptionID")]
    pub option_id: i32,
    #[serde(rename = "ExecutionID")]
    pub execution_id: i32,
    #[serde(rename = "Expiration")]
    pub expiration: String,
    #[serde(rename = "Strike")]
    pub strike: f64,
    #[serde(rename = "Type")]
    pub option_type: String,
    #[serde(rename = "Quantity")]
    pub quantity: i32,
    #[serde(rename = "Premium")]
    pub premium: f64,
    #[serde(rename = "Opra")]
    pub opra: String,
}

impl fmt::Display for Trade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "| {:^5} | {:^6} | {:^10} | {:^5} | {:^5} | {:^10} |",
            self.trade_id, self.symbol, self.open_date, self.broker_id,
            self.exchange_id, self.realized_gain.unwrap_or(0.0)
        )?;

        for execution in &self.executions {
            writeln!(
                f,
                "    | {:^5} | {:^19} | {:^6} | {:^6} | {:^10} | {:^10} |",
                execution.execution_id, execution.execution_date_time, execution.quantity,
                execution.order_price, execution.commission, execution.fees
            )?;

            for option in &execution.options {
                writeln!(
                    f,
                    "        | {:^5} | {:^10} | {:^6} | {:^10} | {:^5} | {:^10} | {:^20} |",
                    option.option_id, option.expiration, option.quantity, option.strike,
                    option.option_type, option.premium,option.opra
                )?;
            }
        }

        Ok(())
    }
}

impl Loadable for Trade {
    fn load<T>(loader: &dyn DataLoader<T>) -> Result<Vec<Self>, DataLoaderError> {
        if loader.get_type() == "MySQL" {
           return load_mysql_data(loader);
        }
        else if loader.get_type() == "CSV" {
            return load_csv_data(loader);
        }
        Err(DataLoaderError::UnknownSourceError("Unknown DataLoader Type: {loader.get_type()}".to_string()))
    }
}

fn load_mysql_data<T>(loader: &dyn DataLoader<T>)  -> Result<Vec<Trade>, DataLoaderError> {
    let pool = loader.get_pool();
    let mut conn = pool.get_conn()?;

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

fn load_csv_data<T>(loader: &dyn DataLoader<T>)  -> Result<Vec<Trade>, DataLoaderError> {
    let mut trades: Vec<Trade> = Vec::new();
    let options = loader.get_options();

    let temp = "".to_string();
    let trades_path = options.get(&"trades_path".to_string()).unwrap_or(&temp);
    let trade_executions_path = options.get(&"trade_executions_path".to_string()).unwrap_or(&temp);
    let options_details_path = options.get(&"options_details_path".to_string()).unwrap_or(&temp);

    log::debug!("Trades path = {}", trades_path);
    log::debug!("Execs path  = {}", trade_executions_path);
    log::debug!("Opts path   = {}", options_details_path);

    let mut trades_reader = ReaderBuilder::new()
        .from_path(&trades_path)
        .map_err(|e| DataLoaderError::from(e))?;

    for trade in trades_reader.deserialize() {
        let trade: Trade = trade.map_err(|e| DataLoaderError::from(e))?;
        trades.push(trade);
    }

    let mut trade_executions_reader = ReaderBuilder::new()
        .from_path(&trade_executions_path)
        .map_err(|e| DataLoaderError::from(e))?;

    let mut trade_executions: Vec<TradeExecution> = Vec::new();

    for trade_execution in trade_executions_reader.deserialize() {
        let trade_execution: TradeExecution =
            trade_execution.map_err(|e| DataLoaderError::from(e))?;
        trade_executions.push(trade_execution);
    }

    let mut options_details_reader = ReaderBuilder::new()
        .from_path(&options_details_path)
        .map_err(|e| DataLoaderError::from(e))?;

    let mut options_details: Vec<OptionDetail> = Vec::new();

    for option_detail in options_details_reader.deserialize() {
        let option_detail: OptionDetail =
            option_detail.map_err(|e| DataLoaderError::from(e))?;
        options_details.push(option_detail);
    }

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
