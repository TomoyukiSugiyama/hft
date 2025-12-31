use crate::asset::stock::{HistoricalPrice, HistoricalPrices};
use crate::indicator::{Indicator, IndicatorPoint};
use crate::trend::engine::{OverboughtOversell, Trend, TrendAnalysis, TrendEngine};

pub struct PriceCross {
    indicator: Box<dyn Indicator>,
}

impl PriceCross {
    pub fn new(indicator: Box<dyn Indicator>) -> Self {
        Self { indicator }
    }
}

impl TrendEngine for PriceCross {
    fn name(&self) -> &str {
        "price cross"
    }

    fn indicator(&self) -> &Box<dyn Indicator> {
        &self.indicator
    }

    fn analyze(&self, historical_prices: &HistoricalPrices) -> Vec<TrendAnalysis> {
        price_cross(
            historical_prices,
            &self.indicator.calculate(historical_prices),
        )
    }
}

fn price_cross(prices: &[HistoricalPrice], indicators: &[IndicatorPoint]) -> Vec<TrendAnalysis> {
    if prices.len() < 2 || indicators.len() < 2 || prices.len() < indicators.len(){
        return vec![];
    }
    let mut ta:Vec<TrendAnalysis> = Vec::new();

    let offset = prices.len() - indicators.len();
    for i in 0..indicators.len() - 2 {
        ta.push(
            TrendAnalysis{
                timestamp: indicators[i].timestamp,
                trend: check_trend(&prices[i+offset..i+offset+2], &indicators[i..i+2]),
                overbought_oversell: OverboughtOversell::Neutral
            }
        );
    }

    ta
}

fn check_trend(prices: &[HistoricalPrice], indicators: &[IndicatorPoint]) -> Trend {
    if prices.len() < 2 {
        println!("prices: {}",prices.len());
        return Trend::Neutral;
    }
    if indicators.len() < 2 {
        println!("indi: {}",indicators.len());
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

    match (prev_price, current_price) {
        (Position::High, Position::Low) => Trend::Downtrend,
        (Position::Low, Position::High) => Trend::Uptrend,
        _ => Trend::Neutral,
    }
}
