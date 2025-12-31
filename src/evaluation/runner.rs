use chrono::{Local, TimeZone};

use crate::asset::Stock;
use crate::capital::Capital;
use crate::evaluation::reporting::reporting;
use crate::evaluation::visualization::plot_price;
use crate::strategy::{StrategyEngine, TrendFollow};

pub fn run_backtest() -> Result<(), Box<dyn std::error::Error>> {
    let capital = Capital::new();
    // let asset_manager = AssetManager::new();
    let strategy_engine = Box::new(TrendFollow::new());

    let stock = Stock::new(
        "SMPL".to_string(),
        "Sample".to_string(),
        "historical_data/btcusd_bitstamp_1min_2012-2025.csv".to_string(),
    );
    let start = Local
        .with_ymd_and_hms(2024, 1, 1, 0, 0, 0)
        .single()
        .ok_or("Invalid start date")?;

    let end = Local
        .with_ymd_and_hms(2024, 1, 1, 0, 59, 59)
        .single()
        .ok_or("Invalid end date")?;
    let hps = stock.historical_prices().filter_by(start, end);

    let iss = strategy_engine.trend_engine().indicator().calculate(&hps);

    // plot_price(&hps,&iss);
    if let Err(err) = plot_price(&hps, &iss) {
        println!("{}", err);
    }
    // Reporting::output(model);
    reporting(
        &capital,
        &stock,
        &(strategy_engine as Box<dyn StrategyEngine>),
        &hps,
    );

    Ok(())
}
