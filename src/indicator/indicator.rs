use chrono::{DateTime, Local};

use crate::asset::stock::HistoricalPrices;

pub trait Indicator {
    fn name(&self) -> &str;
    fn calculate(&self,historical_prices:&HistoricalPrices) -> IndicatorSeries;
}

#[derive(Debug)]
pub struct IndicatorSeries {
    pub data: Vec<IndicatorPoint>
}

#[derive(Debug)]
pub struct IndicatorPoint{
    pub timestamp: DateTime<Local>,
    pub value: f64
}