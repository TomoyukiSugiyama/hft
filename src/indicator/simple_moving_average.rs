use crate::indicator::indicator::{Indicator, IndicatorSeries};
use crate::asset::stock::HistoricalPrices;

pub struct SimpleMovingAverage {
    name: String
}

impl SimpleMovingAverage {
    pub fn new() -> Self{
        Self { name: "simple moving average".to_string() }
    }
}

impl Indicator for SimpleMovingAverage{
    fn name(&self) -> &str {
        &self.name
    }
    
    fn calculate(&self,historical_prices:&HistoricalPrices) -> IndicatorSeries {
        todo!()
    }    
}