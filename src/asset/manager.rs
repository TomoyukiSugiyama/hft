pub use crate::asset::Stock;
use chrono::offset::Local;
use chrono::prelude::*;

pub struct AssetManager {
    stock: Stock,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            stock: Stock::new(
                "SMPL".to_string(),
                "Sample".to_string(),
                "historical_data/btcusd_bitstamp_1min_2012-2025.csv".to_string(),
            ),
        }
    }

    pub fn stock_summary(&self) -> String {
        let start = Local
            .with_ymd_and_hms(2024, 1, 1, 0, 0, 0)
            .single()
            .ok_or("Invalid start date")
            .unwrap();

        let end = Local
            .with_ymd_and_hms(2024, 1, 1, 0, 59, 59)
            .single()
            .ok_or("Invalid end date")
            .unwrap();

        format!(
            "symbol: {}\nname: {}\nhistorical_prices:\n{}",
            self.stock.symbol(),
            self.stock.name(),
            self.stock
                .historical_prices()
                .filter_by(start, end)
                .head(20)
                .to_string()
        )
    }

    pub fn stock(&self) -> &Stock {
        &self.stock
    }
}
