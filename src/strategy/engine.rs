use core::fmt;

use chrono::{DateTime, Local};

use crate::{asset::HistoricalPrices, trend::TrendEngine};

pub trait StrategyEngine {
    fn name(&self) -> &str;
    fn trend_engine(&self) -> &Box<dyn TrendEngine>;
    fn calculate(&self, historical_prices: &HistoricalPrices) -> Vec<TradeSignal>;
}

pub struct TradeSignal {
    pub timestamp: DateTime<Local>,
    pub signal: Signal,
    pub entry_price: f64,
    pub stop_loss: f64,
    pub take_profit: f64,
}

impl fmt::Display for TradeSignal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}]:signal:{} / enyty_price:{} / stop_loss: {} / take_profit: {}",
            self.timestamp,
            self.signal.to_string(),
            self.entry_price,
            self.stop_loss,
            self.take_profit
        )
    }
}

#[derive(Clone)]
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
