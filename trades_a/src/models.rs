// models.rs
use serde::Deserialize;
use std::fmt;

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
