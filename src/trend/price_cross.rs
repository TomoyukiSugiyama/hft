use crate::asset::stock::{HistoricalPrice, HistoricalPrices};
use crate::indicator::{Indicator, IndicatorPoint, SimpleMovingAverage};
use crate::trend::engine::{OverboughtOversell, Trend, TrendAnalysis, TrendEngine};

pub struct PriceCross {
    name: String,
    indicator: Box<dyn Indicator>,
}

impl PriceCross {
    pub fn new() -> Self {
        Self {
            name: "price cross".to_string(),
            indicator: Box::new(SimpleMovingAverage::new(14)),
        }
    }
}

impl TrendEngine for PriceCross {
    fn name(&self) -> &str {
        &self.name
    }

    fn indicator(&self) -> &Box<dyn Indicator> {
        &self.indicator
    }

    fn analyze(&self, historical_prices: &HistoricalPrices) -> TrendAnalysis {
        TrendAnalysis {
            trend: price_cross(historical_prices,&self.indicator.calculate(historical_prices)),
            overbought_oversell: OverboughtOversell::Neutral,
        }
    }
}

fn price_cross(prices: &[HistoricalPrice], indicators: &[IndicatorPoint]) -> Trend {

    if prices.len() < 2 {
        return Trend::Neutral;
    }
    if indicators.len() < 2 {
        return Trend::Neutral;
    }
    enum Position {
        High,
        Low,
    }
    let prev_price = if prices[prices.len() - 2].close > indicators[indicators.len() - 2].value {
        Position::High
    } else {
        Position::Low
    };

    let current_price = if prices[prices.len() - 1].close > indicators[indicators.len() - 1].value {
        Position::High
    } else {
        Position::Low
    };

    if (matches!(prev_price, Position::High) && matches!(current_price, Position::Low))
    {
        Trend::Downtrend
    }
    else if (matches!(prev_price, Position::Low) && matches!(current_price, Position::High))
    {
        Trend::Uptrend
    } else {
        Trend::Neutral
    }
}
