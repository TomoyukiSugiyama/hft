pub use crate::capital::CapitalManager;
pub use crate::asset::AssetManager;
use crate::evaluation::Evaluation;
pub use crate::strategy::Engine;

pub struct Model{
    pub capital_manager: CapitalManager,
    pub asset_manager: AssetManager,
    pub strategy_engine: Engine,
    pub evaluation: Evaluation
}

impl Model{
    pub fn new() -> Self{
        Self{
            capital_manager: CapitalManager::new(),
            asset_manager: AssetManager::new(),
            strategy_engine: Engine::new(),
            evaluation: Evaluation::new()
        }
    }

    pub fn run(&self){
        println!("investment capital: {}", self.capital_manager.initial_investment_amount());
        println!("allowable drawdown percentage: {}", self.capital_manager.allowable_drawdown_percentage());
        println!("stock:\n{}", self.asset_manager.stock());
        println!("historical_prices:\n{}", self.asset_manager.historical_prices_head());
        println!("engine: {}", self.strategy_engine.name());
        println!("reporting: {}", self.evaluation.reporting());
        
    }
}
