use crate::{
    asset::{Stock, stock::HistoricalPrices},
    capital::Capital,
    strategy::StrategyEngine,
};

pub fn reporting(
    capital: &Capital,
    stock: &Stock,
    strategy_engine: &Box<dyn StrategyEngine>,
    hps: &HistoricalPrices,
) {
    println!("[capital]");
    println!(
        "investment capital: {}",
        capital.initial_investment_amount()
    );
    println!(
        "allowable drawdown percentage: {}",
        capital.allowable_drawdown_percentage()
    );
    println!(
        "[asset]\nsymbpl:{}\nname:{}\nhistorical_prices:{}",
        stock.symbol(),
        stock.name(),
        hps.to_string()
    );

    println!(
        "[strategy]\nengine: {}\nsignal: {}",
        strategy_engine.name(),
        strategy_engine.calculate(hps).signal.to_string()
    );
    println!("[trend]\nengine: {}", strategy_engine.trend_engine().name());
    println!(
        "[trend]\nengine: {}",
        strategy_engine.trend_engine().analyze(hps).to_string()
    );
    println!(
        "[indicator]\nname: {}",
        strategy_engine.trend_engine().indicator().name()
    );
    println!(
        "sma:\n{}",
        strategy_engine
            .trend_engine()
            .indicator()
            .calculate(hps)
            .to_string()
    );
}
