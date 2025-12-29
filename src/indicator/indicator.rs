use chrono::{DateTime, Local};

use crate::asset::stock::HistoricalPrices;

pub trait Indicator {
    fn name(&self) -> &str;
    fn calculate(&self,historical_prices:&HistoricalPrices) -> IndicatorSeries;
}

pub struct IndicatorSeries {
    data: Vec<IndicatorPoint>
}

pub struct IndicatorPoint{
    timestamp: DateTime<Local>
    
}