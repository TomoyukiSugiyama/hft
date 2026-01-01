use chrono::{DateTime, Local, TimeZone};

use crate::asset::Stock;
use crate::asset::stock::HistoricalPrices;
use crate::capital::Capital;
use crate::evaluation::reporting::reporting;
use crate::evaluation::visualization::plot_price;
use crate::indicator::SimpleMovingAverage;
use crate::strategy::{Signal, StrategyEngine, TradeSignal, TrendFollow};
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
        .with_ymd_and_hms(2024, 1, 1, 1, 59, 59)
        .single()
        .ok_or("Invalid end date")?;
    let hps = stock.historical_prices().filter_by(start, end);

    let indicator = Box::new(SimpleMovingAverage::new(14));
    let trend_engine = Box::new(PriceCross::new(indicator));
    let strategy_engine = Box::new(TrendFollow::new(trend_engine));

    let tss = strategy_engine.calculate(&hps);
    // TODO: indicator を直接呼べるようにする。
    let iss = strategy_engine.trend_engine().indicator().calculate(&hps);

    let pls = calculate_profit_loss(&tss, &hps);
    let mut total_profit: f64 = 0.0;
    pls.iter().for_each(|pl| {
        total_profit += pl.profit_loss;
        println!(
            "[{}]:entry: {} / exit: {} / profit loss: {} / signal: {}",
            pl.timestamp,
            pl.entry_price,
            pl.exit_price,
            pl.profit_loss,
            pl.signal.to_string()
        )
    });
    println!("total_profit {}", total_profit);

    if let Err(err) = plot_price(&hps, &iss, &tss) {
        println!("{}", err);
    }

    reporting(&capital, &stock, strategy_engine, &hps);

    Ok(())
}

struct ProfitLoss {
    timestamp: DateTime<Local>,
    entry_price: f64,
    exit_price: f64,
    profit_loss: f64,
    signal: Signal,
}

fn calculate_profit_loss(
    trade_signal: &Vec<TradeSignal>,
    historical_prices: &HistoricalPrices,
) -> Vec<ProfitLoss> {
    let mut pls: Vec<ProfitLoss> = Vec::new();
    for ts in trade_signal {
        if matches!(ts.signal, Signal::Hold) {
            continue;
        }

        let exit_point = historical_prices
            .iter()
            .skip_while(|hp| hp.timestamp <= ts.timestamp)
            .find(|hp| match ts.signal {
                Signal::Buy => hp.close <= ts.stop_loss || hp.close >= ts.take_profit,
                Signal::Sell => hp.close >= ts.stop_loss || hp.close <= ts.take_profit,
                _ => false,
            });

        if let Some(hp) = exit_point {
            let profit_loss = match ts.signal {
                Signal::Buy => hp.close - ts.entry_price,
                Signal::Sell => ts.entry_price - hp.close,
                _ => 0.0,
            };
            pls.push(ProfitLoss {
                timestamp: hp.timestamp,
                entry_price: ts.entry_price,
                exit_price: hp.close,
                profit_loss,
                signal: ts.signal.clone(),
            });
        }
    }
    pls
}
