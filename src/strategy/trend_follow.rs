use crate::asset::stock::HistoricalPrices;
use crate::strategy::StrategyEngine;
use crate::strategy::engine::{Signal, TradeSignal};
use crate::trend::PriceCross;
use crate::trend::{Trend, TrendEngine};

pub struct TrendFollow {
    trend_engine: Box<dyn TrendEngine>,
}

impl TrendFollow {
    pub fn new() -> Self {
        Self {
            trend_engine: Box::new(PriceCross::new()),
        }
    }
}

impl StrategyEngine for TrendFollow {
    fn name(&self) -> &str {
        "trend follow"
    }

    fn trend_engine(&self) -> &Box<dyn TrendEngine> {
        &self.trend_engine
    }

    fn calculate(&self, historical_prices: &HistoricalPrices) -> TradeSignal {
        TradeSignal {
            signal: trend_follow(historical_prices, &self.trend_engine),
        }
    }
}

fn trend_follow(
    historical_prices: &HistoricalPrices,
    trend_engine: &Box<dyn TrendEngine>,
) -> Signal {
    let ta = trend_engine.analyze(historical_prices);

    match ta.trend {
        Trend::Uptrend => Signal::Buy,
        Trend::Downtrend => Signal::Sell,
        Trend::Neutral => Signal::Hold,
    }
}
