pub struct CapitalManager{
    initial_investment_amount: f64,
    allowable_drawdown_percentage: f64,
    
}

impl CapitalManager{
    pub fn new() -> Self{
        Self{
            initial_investment_amount: 1000000.0,
            allowable_drawdown_percentage: 0.2,
        }
    }

    pub fn initial_investment_amount(&self) -> f64{
        self.initial_investment_amount
    }

    pub fn allowable_drawdown_percentage(&self) -> f64{
        self.allowable_drawdown_percentage
    }
}