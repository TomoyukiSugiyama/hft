use crate::indicator::Indicator;

pub trait TrendEngine {
    fn name(&self) -> &str;
    fn indicator(&self) -> &Box<dyn Indicator>;
    fn analyze(&self) -> TrendAnalysis;
}

#[derive(Debug)]
pub struct TrendAnalysis {
    pub trend: Trend,
    pub overbought_oversell: OverboughtOversell,
}

#[derive(Debug)]
pub enum Trend{
    Uptrend,
    Downtrend,
    Newtral
}

#[derive(Debug)]
pub enum OverboughtOversell{
    Overbought,
    Oversell,
    Newtral
}

