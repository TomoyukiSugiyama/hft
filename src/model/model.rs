pub use crate::capital::CapitalManager;
pub use crate::asset::AssetManager;
use crate::strategy::StrategyEngine;
use crate::strategy::TrendFollow;

pub struct Model{
    pub capital_manager: CapitalManager,
    pub asset_manager: AssetManager,
    pub strategy_engine: Box<dyn StrategyEngine>,
}

impl Model{
    pub fn new() -> Self{
        Self{
            capital_manager: CapitalManager::new(),
            asset_manager: AssetManager::new(),
            strategy_engine: Box::new(TrendFollow::new()),
        }
    }
}
