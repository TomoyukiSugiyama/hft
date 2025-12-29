use crate::asset::stock::HistoricalPrices;

pub trait StrategyEngine {
    fn name(&self) -> &str;
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
