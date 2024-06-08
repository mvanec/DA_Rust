// data_loader.rs
use async_trait::async_trait;
use crate::models::*;

#[async_trait]
pub trait DataLoader {
    async fn load_trades(&self) -> Vec<Trade>;
}
