// models.rs
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Trade {
    pub trade_id: i32,
    pub symbol: String,
    pub open_date: String,
    pub close_date: Option<String>,
    pub broker_id: i32,
    pub exchange_id: i32,
    pub realized_gain: Option<f64>,
    pub executions: Vec<TradeExecution>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TradeExecution {
    pub execution_id: i32,
    pub trade_id: i32,
    pub execution_date_time: String,
    pub spread: String,
    pub quantity: i32,
    pub position_effect: String,
    pub order_price: f64,
    pub fill_price: f64,
    pub commission: f64,
    pub fees: f64,
    pub reference_number: String,
    pub options: Vec<OptionDetail>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OptionDetail {
    pub option_id: i32,
    pub execution_id: i32,
    pub expiration: String,
    pub strike: f64,
    pub option_type: String,
    pub quantity: i32,
    pub premium: f64,
    pub opra: String,
}
