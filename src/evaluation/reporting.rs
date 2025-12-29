pub use crate::model::Model;
pub struct Reporting {

}

impl Reporting {
    pub fn ourput(model: &Model){
        println!("[capital]");
        println!("investment capital: {}", model.capital_manager.initial_investment_amount());
        println!("allowable drawdown percentage: {}", model.capital_manager.allowable_drawdown_percentage());
        println!("[asset]\n{}", model.asset_manager.stock_summary());
        println!("[strategy]\nengine: {}\nsignal: {:?}", model.strategy_engine.name(),model.strategy_engine.calculate(model.asset_manager.stock().historical_prices()).signal);
        println!("[trend]\nengine: {}", model.strategy_engine.trend_engine().name());
        println!("[trend]\nengine: {:?}", model.strategy_engine.trend_engine().analyze());
        println!("[indicator]\nname: {:?}", model.strategy_engine.trend_engine().indicator().name());
    }
}
