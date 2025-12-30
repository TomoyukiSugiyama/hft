use crate::asset::stock::{HistoricalPrice, HistoricalPrices};
use crate::indicator::indicator::{Indicator, IndicatorPoint, IndicatorSeries};

pub struct SimpleMovingAverage {
    period: usize,
}

impl SimpleMovingAverage {
    pub fn new(period: usize) -> Self {
        Self { period }
    }
}

impl Indicator for SimpleMovingAverage {
    fn name(&self) -> &str {
        "simple moving average"
    }

    fn calculate(&self, historical_prices: &HistoricalPrices) -> IndicatorSeries {
        let data = calculate_sma(historical_prices, self.period);
        IndicatorSeries { data }
    }
}

fn calculate_sma(prices: &[HistoricalPrice], period: usize) -> Vec<IndicatorPoint> {
    if prices.len() < period {
        return vec![];
    }

    let mut result = Vec::new();
    for i in (period - 1)..prices.len() {
        let window = &prices[i - (period - 1)..=i];
        let sum: f64 = window.iter().map(|p| p.close).sum();
        let sma_value = sum / period as f64;
        result.push(IndicatorPoint {
            timestamp: prices[i].timestamp,
            value: sma_value,
        });
    }

    result
}
