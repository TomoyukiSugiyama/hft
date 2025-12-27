pub use crate::asset::Stock;
use chrono::prelude::*;
pub struct AssetManager {
    stock: Stock,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            stock: Stock::new("SMPL".to_string(), "Sample".to_string(), "historical_data/btcusd_bitstamp_1min_2012-2025.csv".to_string()),
        }
    }

    pub fn stock(&self) -> String {
        format!("symbol:{}/name:{}", self.stock.symbol(), self.stock.name())
    }

    pub fn historical_prices_head(&self) -> String {
        self.stock.head(10)
    }

    pub fn plot_data(&self) -> Vec<(DateTime<Local>, f32, f32, f32, f32)> {
        self.stock.plot_data()
    }
}
