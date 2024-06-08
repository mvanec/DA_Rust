// data_loader.rs
use crate::models::*;

#[async_trait::async_trait]
pub trait DataLoader {
    async fn load_trades(&self) -> Vec<Trade>;
}
