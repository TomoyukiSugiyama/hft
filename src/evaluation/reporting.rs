use crate::{
    asset::{Stock, stock::HistoricalPrices},
    capital::Capital,
    evaluation::runner::ProfitLoss,
    strategy::StrategyEngine,
};

pub fn reporting(
    capital: &Capital,
    stock: &Stock,
    strategy_engine: Box<dyn StrategyEngine>,
    hps: &HistoricalPrices,
    pls: &Vec<ProfitLoss>,
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
        "[asset]\nsymbpl:{}\nname:{}\nhistorical_prices:\n{}",
        stock.symbol(),
        stock.name(),
        hps.to_string()
    );

    println!(
        "[indicator]\nname: {}",
        strategy_engine.trend_engine().indicator().name()
    );
    println!(
        "indicator series:\n{}",
        strategy_engine
            .trend_engine()
            .indicator()
            .calculate(hps)
            .to_string()
    );

    let tas: String = strategy_engine
        .trend_engine()
        .analyze(hps)
        .iter()
        .map(|ta| format!("{}\n", ta.to_string()))
        .collect();

    println!(
        "[trend]\nengine: {}\ntrend analysis:\n{}",
        strategy_engine.trend_engine().name(),
        tas
    );

    let tss: String = strategy_engine
        .calculate(hps)
        .iter()
        .map(|ts| format!("{}\n", ts.to_string()))
        .collect();
    println!(
        "[strategy]\nengine: {}\ntrade signal:\n{}",
        strategy_engine.name(),
        tss
    );

    let mut total_profit: f64 = 0.0;
    pls.iter().for_each(|pl| total_profit += pl.profit_loss);
    println!("[profit loss]\ntotal_profit: {}", total_profit);

    let pls: String = pls
        .iter()
        .map(|pl| format!("{}\n", pl.to_string()))
        .collect();
    println!("profit loss:\n{}", pls);



}
