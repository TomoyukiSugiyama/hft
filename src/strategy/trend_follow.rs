
use crate::asset::stock::HistoricalPrices;
use crate::strategy::StrategyEngine;
use crate::strategy::engine::{Signal,TradeSignal};
use crate::trend::TrendEngine;
use crate::trend::PriceCross;

pub struct TrendFollow {
    name: String,
    trend_engine: Box<dyn TrendEngine>
}

impl TrendFollow {
    pub fn new() -> Self {
        Self {
            name: "trend follow".to_string(),
            trend_engine: Box::new(PriceCross::new())
        }
    }
}

impl StrategyEngine for TrendFollow {
    fn name(&self) -> &str {
        &self.name
    }

    fn trend_engine(&self) -> &Box<dyn TrendEngine> {
        &self.trend_engine
    }

    fn calculate(&self,historical_prices: &HistoricalPrices) -> TradeSignal {
        TradeSignal {
            signal: Signal::Hold,
        }
    }
}
