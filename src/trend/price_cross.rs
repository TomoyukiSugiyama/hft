use crate::trend::engine::TrendAnalysis;
use crate::trend::engine::{OverboughtOversell, Trend, TrendEngine};

pub struct PriceCross {
    name: String,
}

impl PriceCross {
    pub fn new() -> Self {
        Self {
            name: "price cross".to_string(),
        }
    }
}

impl TrendEngine for PriceCross {
    fn name(&self) -> &str {
        &self.name
    }

    fn analyze(&self) -> TrendAnalysis {
        TrendAnalysis {
            trend: Trend::Newtral,
            overbought_oversell: OverboughtOversell::Newtral,
        }
    }
}
