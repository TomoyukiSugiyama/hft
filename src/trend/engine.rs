use core::fmt;

use chrono::{DateTime, Local};

use crate::{asset::stock::HistoricalPrices, indicator::Indicator};

pub trait TrendEngine {
    fn name(&self) -> &str;
    fn indicator(&self) -> &Box<dyn Indicator>;
    fn analyze(&self, historical_prices: &HistoricalPrices) -> Vec<TrendAnalysis>;
}

pub struct TrendAnalysis {
    pub timestamp: DateTime<Local>,
    pub trend: Trend,
    pub overbought_oversell: OverboughtOversell,
}

impl fmt::Display for TrendAnalysis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}]:trend: {} / overbought_oversell: {}",
            self.timestamp, self.trend, self.overbought_oversell
        )
    }
}

pub enum Trend {
    Uptrend,
    Downtrend,
    Neutral,
}

impl fmt::Display for Trend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Trend::Uptrend => write!(f, "uptrend"),
            Trend::Downtrend => write!(f, "downtrend"),
            Trend::Neutral => write!(f, "neutral"),
        }
    }
}

pub enum OverboughtOversell {
    Overbought,
    Oversell,
    Neutral,
}

impl fmt::Display for OverboughtOversell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OverboughtOversell::Overbought => write!(f, "overbought"),
            OverboughtOversell::Oversell => write!(f, "oversell"),
            OverboughtOversell::Neutral => write!(f, "neutral"),
        }
    }
}
