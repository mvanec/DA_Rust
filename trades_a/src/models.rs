// models.rs
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Trade {
    pub TradeID: i32,
    pub Symbol: String,
    pub OpenDate: String,
    pub CloseDate: Option<String>,
    pub BrokerID: i32,
    pub ExchangeID: i32,
    pub RealizedGain: Option<f64>,
    pub executions: Vec<TradeExecution>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TradeExecution {
    pub ExecutionID: i32,
    pub TradeID: i32,
    pub ExecutionDateTime: String,
    pub Spread: String,
    pub Quantity: i32,
    pub PositionEffect: String,
    pub OrderPrice: f64,
    pub FillPrice: f64,
    pub Commission: f64,
    pub Fees: f64,
    pub ReferenceNumber: String,
    pub options: Vec<OptionDetail>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OptionDetail {
    pub OptionID: i32,
    pub ExecutionID: i32,
    pub Expiration: String,
    pub Strike: f64,
    pub Type: String,
    pub Quantity: i32,
    pub Premium: f64,
    pub Opra: String,
}
