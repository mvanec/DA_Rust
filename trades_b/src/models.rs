// models.rs
use serde::Deserialize;

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
