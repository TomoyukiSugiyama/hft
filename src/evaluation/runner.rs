use chrono::{Local, TimeZone};

use crate::asset::Stock;
use crate::capital::Capital;
use crate::evaluation::reporting::reporting;
use crate::evaluation::visualization::plot_price;
use crate::indicator::SimpleMovingAverage;
use crate::strategy::{StrategyEngine, TrendFollow};
use crate::trend::PriceCross;

pub fn run_backtest() -> Result<(), Box<dyn std::error::Error>> {
    let capital = Capital::new();
    // let asset_manager = AssetManager::new();

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

    let indicator = Box::new(SimpleMovingAverage::new(14));
    let trend_engine = Box::new(PriceCross::new(indicator));
    let strategy_engine = Box::new(TrendFollow::new(trend_engine));

    // TODO: indicator を直接呼べるようにする。
    let iss = strategy_engine.trend_engine().indicator().calculate(&hps);

    if let Err(err) = plot_price(&hps, &iss) {
        println!("{}", err);
    }

    reporting(&capital, &stock, strategy_engine, &hps);

    Ok(())
}
