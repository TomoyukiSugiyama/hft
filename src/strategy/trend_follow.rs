use crate::asset::stock::HistoricalPrices;
use crate::strategy::StrategyEngine;
use crate::strategy::engine::{Signal, TradeSignal};
use crate::trend::{Trend, TrendEngine};

pub struct TrendFollow {
    trend_engine: Box<dyn TrendEngine>,
}

impl TrendFollow {
    pub fn new(trend_engine: Box<dyn TrendEngine>) -> Self {
        Self { trend_engine }
    }
}

impl StrategyEngine for TrendFollow {
    fn name(&self) -> &str {
        "trend follow"
    }

    fn trend_engine(&self) -> &Box<dyn TrendEngine> {
        &self.trend_engine
    }

    fn calculate(&self, historical_prices: &HistoricalPrices) -> Vec<TradeSignal> {
        trend_follow(historical_prices, &self.trend_engine)
    }
}

fn trend_follow(
    historical_prices: &HistoricalPrices,
    trend_engine: &Box<dyn TrendEngine>,
) -> Vec<TradeSignal> {
    let tas = trend_engine.analyze(historical_prices);

    let mut tss = Vec::new();
    tas.iter().for_each(|ta| {
        tss.push(TradeSignal {
            timestamp: ta.timestamp,
            entry_price: ta.price,
            signal: match ta.trend {
                Trend::Uptrend => Signal::Buy,
                Trend::Downtrend => Signal::Sell,
                Trend::Neutral => Signal::Hold,
            },
        })
    });
    tss
}
