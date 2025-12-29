use crate::indicator::{Indicator, SimpleMovingAverage};
use crate::trend::engine::{OverboughtOversell, Trend, TrendAnalysis, TrendEngine};

pub struct PriceCross {
    name: String,
    indicator: Box<dyn Indicator>,
}

impl PriceCross {
    pub fn new() -> Self {
        Self {
            name: "price cross".to_string(),
            indicator: Box::new(SimpleMovingAverage::new()),
        }
    }
}

impl TrendEngine for PriceCross {
    fn name(&self) -> &str {
        &self.name
    }

    fn indicator(&self) -> &Box<dyn Indicator> {
        &self.indicator
    }

    fn analyze(&self) -> TrendAnalysis {
        TrendAnalysis {
            trend: Trend::Newtral,
            overbought_oversell: OverboughtOversell::Newtral,
        }
    }
}
