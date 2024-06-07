// models.rs
use std::collections::{VecDeque, HashMap};

#[derive(Debug, Clone)]
struct Trade {
    trade_id: i32,
    symbol: String,
    open_date: String,
    close_date: Option<String>,
    broker_id: i32,
    exchange_id: i32,
    realized_gain: Option<f64>,
    executions: VecDeque<TradeExecution>,
}

#[derive(Debug, Clone)]
struct TradeExecution {
    execution_id: i32,
    execution_date_time: String,
    spread: String,
    quantity: i32,
    position_effect: String,
    order_price: f64,
    fill_price: f64,
    commission: f64,
    fees: f64,
    reference_number: String,
    options: VecDeque<OptionDetails>,
}

#[derive(Debug, Clone)]
struct OptionDetails {
    option_id: i32,
    expiration: String,
    strike: f64,
    option_type: String,
    quantity: i32,
    premium: f64,
    opra: String,
}
