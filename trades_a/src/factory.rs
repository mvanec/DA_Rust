// factory.rs
use crate::data_loader::DataLoader;
use crate::models::*;

pub struct TradeFactory<T: DataLoader> {
    data_loader: T,
}

impl<T: DataLoader> TradeFactory<T> {
    pub fn new(data_loader: T) -> Self {
        Self { data_loader }
    }

    pub async fn load_trades(&self) -> Vec<Trade> {
        self.data_loader.load_trades().await
    }
}
