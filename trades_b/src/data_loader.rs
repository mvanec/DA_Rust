// data_loader.rs
use crate::models::*;

pub trait DataLoader {
    fn load_trades(&self) -> Vec<Trade>;
}
