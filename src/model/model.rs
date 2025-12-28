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

    pub fn run(&self){
        println!("[capital]");
        println!("investment capital: {}", self.capital_manager.initial_investment_amount());
        println!("allowable drawdown percentage: {}", self.capital_manager.allowable_drawdown_percentage());
        println!("[asset]\n{}", self.asset_manager.stock_summary());
        println!("[strategy]\nengine: {}", self.strategy_engine.name());        
    }
}
