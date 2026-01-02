use core::fmt;

use chrono::{DateTime, Local, TimeZone};

use crate::asset::Stock;
use crate::asset::HistoricalPrices;
use crate::capital::InvestmentCapital;
use crate::capital::manager::calculate_capital_history;
use crate::evaluation::reporting::reporting;
use crate::evaluation::visualization::plot_ccacpital;
use crate::evaluation::visualization::plot_price;
use crate::indicator::SimpleMovingAverage;
use crate::strategy::{Signal, StrategyEngine, TradeSignal, TrendFollow};
use crate::trend::PriceCross;

pub fn run_backtest() -> Result<(), Box<dyn std::error::Error>> {
    let capital = InvestmentCapital::new();

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

    let cs = calculate_capital_history(&hps, capital.initial_investment_capital_amount(), &pls);

    if let Err(err) = plot_price(&hps, &iss, &tss) {
        println!("{}", err);
    }

    if let Err(err) = plot_ccacpital(&cs) {
        println!("{}", err);
    }
    reporting(&capital, &cs, &stock, strategy_engine, &hps, &pls);

    Ok(())
}

pub struct ProfitLoss {
    pub timestamp: DateTime<Local>,
    entry_price: f64,
    exit_price: f64,
    pub profit_loss: f64,
    signal: Signal,
}

impl fmt::Display for ProfitLoss {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}]:entry: {} / exit: {} / profit loss: {} / signal: {}",
            self.timestamp,
            self.entry_price,
            self.exit_price,
            self.profit_loss,
            self.signal.to_string()
        )
    }
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
