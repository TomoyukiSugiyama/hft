pub use crate::capital::CapitalManager;
pub use crate::asset::AssetManager;
pub use crate::strategy::Engine;

pub struct Model{
    pub capital_manager: CapitalManager,
    pub asset_manager: AssetManager,
    pub strategy_engine: Engine,
}

impl Model{
    pub fn new() -> Self{
        Self{
            capital_manager: CapitalManager::new(),
            asset_manager: AssetManager::new(),
            strategy_engine: Engine::new(),
        }
    }
}
