// option_removal.rs
use chrono::{NaiveDate, NaiveTime};

#[derive(Clone)] // Add this line
pub struct OptionRemoval {
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub quantity: f64,
    pub symbol: String,
    pub expiration: NaiveDate,
    pub strike_price: f64,
    pub side: String,
}

impl OptionRemoval {
    pub fn new(date: NaiveDate, time: NaiveTime, quantity: f64, symbol: String, expiration: NaiveDate, strike_price: f64, side: String) -> Self {
        Self {
            date,
            time,
            quantity,
            symbol,
            expiration,
            strike_price,
            side,
        }
    }
}
