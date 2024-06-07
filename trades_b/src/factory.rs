// factory.rs
use crate::data_loader::DataLoader;
use crate::models::*;

pub struct TradeFactory {
    data_loader: Box<dyn DataLoader>,
}

impl TradeFactory {
    pub fn new(data_loader: Box<dyn DataLoader>) -> Self {
        Self { data_loader }
    }

    pub fn load_trades(&self) -> Vec<Trade> {
        self.data_loader.load_trades()
    }
}
