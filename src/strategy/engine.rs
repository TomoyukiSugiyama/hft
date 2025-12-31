use core::fmt;

use chrono::{DateTime, Local};

use crate::{asset::stock::HistoricalPrices, trend::TrendEngine};

pub trait StrategyEngine {
    fn name(&self) -> &str;
    fn trend_engine(&self) -> &Box<dyn TrendEngine>;
    fn calculate(&self, historical_prices: &HistoricalPrices) -> Vec<TradeSignal>;
}

pub struct TradeSignal {
    pub timestamp: DateTime<Local>,
    pub signal: Signal,
}

impl fmt::Display for TradeSignal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]:{}",self.timestamp,self.signal.to_string())
    }
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
