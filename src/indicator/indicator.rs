use core::fmt;

use chrono::{DateTime, Local};

use crate::asset::stock::HistoricalPrices;

pub trait Indicator {
    fn name(&self) -> &str;
    fn calculate(&self, historical_prices: &HistoricalPrices) -> IndicatorSeries;
}

pub struct IndicatorSeries {
    pub data: Vec<IndicatorPoint>,
}

impl std::ops::Deref for IndicatorSeries {
    type Target = Vec<IndicatorPoint>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl fmt::Display for IndicatorSeries {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.data.iter().map(|is| is.to_string()).collect();
        write!(f, "{}", s)
    }
}

pub struct IndicatorPoint {
    pub timestamp: DateTime<Local>,
    pub value: f64,
}

impl fmt::Display for IndicatorPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]:{}\n", self.timestamp, self.value)
    }
}
