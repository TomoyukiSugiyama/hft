use core::fmt;

use crate::{asset::stock::HistoricalPrices, trend::TrendEngine};

pub trait StrategyEngine {
    fn name(&self) -> &str;
    fn trend_engine(&self) -> &Box<dyn TrendEngine>;
    fn calculate(&self, historical_prices: &HistoricalPrices) -> TradeSignal;
}

pub struct TradeSignal {
    pub signal: Signal,
}

pub enum Signal {
    Buy,
    Sell,
    Hold,
}

impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Signal::Buy => write!(f, "buy"),
            Signal::Sell => write!(f, "sell"),
            Signal::Hold => write!(f, "hold"),
        }
    }
}
