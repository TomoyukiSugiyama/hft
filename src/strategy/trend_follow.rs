use super::engine::{Signal, TradeSignal};
use crate::asset::stock::HistoricalPrices;
use crate::strategy::StrategyEngine;

pub struct TrendFollow {
    name: String,
}

impl TrendFollow {
    pub fn new() -> Self {
        Self {
            name: "trend follow".to_string(),
        }
    }
}

impl StrategyEngine for TrendFollow {
    fn name(&self) -> &str {
        &self.name
    }

    fn calculate(&self,historical_prices: &HistoricalPrices) -> TradeSignal {
        TradeSignal {
            signal: Signal::Hold,
        }
    }
}
