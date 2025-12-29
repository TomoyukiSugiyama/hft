use crate::{asset::stock::HistoricalPrices, trend::TrendEngine};

pub trait StrategyEngine {
    fn name(&self) -> &str;
    fn trend_engine(&self) -> &Box<dyn TrendEngine>;
    fn calculate(&self,historical_prices:&HistoricalPrices) -> TradeSignal;
}

pub struct TradeSignal {
    pub signal: Signal,
}

#[derive(Debug)]
pub enum Signal {
    Buy,
    Sell,
    Hold,
}
