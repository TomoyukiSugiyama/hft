pub use crate::asset::Stock;
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
        format!(
            "symbol: {}\nname: {}\nhistorical_prices:\n{}",
            self.stock.symbol(),
            self.stock.name(),
            self.stock.historical_prices().head(10).to_string()
        )
    }

    pub fn stock(&self) -> &Stock {
        &self.stock
    }
}
