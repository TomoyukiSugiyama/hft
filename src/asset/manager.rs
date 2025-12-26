pub use crate::asset::Stock;
pub struct AssetManager{
    stock: Stock,
}

impl AssetManager{
    pub fn new() -> Self{
        Self{ stock: Stock::new("AAPL".to_string(), "Apple".to_string()) }
    }

    pub fn stock(&self) -> String {
        format!("symbol:{}:name:{}",self.stock.symbol(),self.stock.name())
    }
}