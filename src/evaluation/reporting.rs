use chrono::{Local, TimeZone};

pub use crate::model::Model;
pub struct Reporting {}

impl Reporting {
    pub fn output(model: &Model) {
        println!("[capital]");
        println!(
            "investment capital: {}",
            model.capital_manager.initial_investment_amount()
        );
        println!(
            "allowable drawdown percentage: {}",
            model.capital_manager.allowable_drawdown_percentage()
        );
        println!("[asset]\n{}", model.asset_manager.stock_summary());
        let start = Local
            .with_ymd_and_hms(2024, 1, 1, 0, 0, 0)
            .single()
            .ok_or("Invalid start date")
            .unwrap();

        let end = Local
            .with_ymd_and_hms(2024, 1, 1, 0, 59, 59)
            .single()
            .ok_or("Invalid end date")
            .unwrap();
        let hp = &model
            .asset_manager
            .stock()
            .historical_prices()
            .filter_by(start, end);
        println!(
            "[strategy]\nengine: {}\nsignal: {:?}",
            model.strategy_engine.name(),
            model.strategy_engine.calculate(hp).signal.to_string()
        );
        println!(
            "[trend]\nengine: {}",
            model.strategy_engine.trend_engine().name()
        );
        println!(
            "[trend]\nengine: {:?}",
            model.strategy_engine.trend_engine().analyze(hp).to_string()
        );
        println!(
            "[indicator]\nname: {:?}",
            model.strategy_engine.trend_engine().indicator().name()
        );
        println!(
            "sma:\n{}",
            model
                .strategy_engine
                .trend_engine()
                .indicator()
                .calculate(hp)
                .to_string()
        );
    }
}
