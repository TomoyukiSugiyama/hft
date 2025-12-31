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
    pub entry_price: f64,
}

impl fmt::Display for TradeSignal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}]:signal:{} / enyty_price:{}",
            self.timestamp,
            self.signal.to_string(),
            self.entry_price
        )
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
