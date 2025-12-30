use core::fmt;

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

#[derive(Debug)]
pub struct IndicatorPoint{
    pub timestamp: DateTime<Local>,
    pub value: f64
}

impl fmt::Display for IndicatorPoint {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "[{}]:{}\n", self.timestamp,self.value)
    }
}