pub use crate::asset::Stock;
pub struct AssetManager {
    stock: Stock,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            stock: Stock::new("AAPL".to_string(), "Apple".to_string(), "historical_data/btcusd_bitstamp_1min_2012-2025.csv".to_string()),
        }
    }

    pub fn stock(&self) -> String {
        format!("symbol:{}/name:{}", self.stock.symbol(), self.stock.name())
    }

    pub fn head(&self) -> String {
        self.stock.head()
    }
}
