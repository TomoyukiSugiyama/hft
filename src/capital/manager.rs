use core::fmt;

use chrono::{DateTime, Local};

use crate::{asset::HistoricalPrices, evaluation::ProfitLoss};

pub struct InvestmentCapital {
    initial_investment_capital_amount: f64,
    allowable_drawdown_percentage: f64,
}

impl InvestmentCapital {
    pub fn new() -> Self {
        Self {
            initial_investment_capital_amount: 1000000.0,
            allowable_drawdown_percentage: 0.2,
        }
    }

    pub fn initial_investment_capital_amount(&self) -> f64 {
        self.initial_investment_capital_amount
    }

    pub fn allowable_drawdown_percentage(&self) -> f64 {
        self.allowable_drawdown_percentage
    }
}

pub struct Capital {
    pub timestamp: DateTime<Local>,
    pub value: f64,
}

impl fmt::Display for Capital {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]:capital: {}", self.timestamp, self.value,)
    }
}

pub fn calculate_capital_history(
    historical_prices: &HistoricalPrices,
    initial_investiment_capital: f64,
    profit_loss_history: &Vec<ProfitLoss>,
) -> Vec<Capital> {
    let mut cs = Vec::new();

    let mut current = initial_investiment_capital;

    historical_prices.iter().for_each(|hp| {
        for pl in profit_loss_history {
            if hp.timestamp.eq(&pl.timestamp) {
                current += pl.profit_loss;
            };
        }
        cs.push(Capital {
            timestamp: hp.timestamp,
            value: current,
        });
    });

    cs
}
