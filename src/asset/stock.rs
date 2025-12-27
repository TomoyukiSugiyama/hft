use std::{
    error::Error
};
use serde::Deserialize;

pub struct Stock {
    symbol: String,
    name: String,
    historical_prices: Vec<HistoricalPrice>,
}

impl Stock {
    pub fn new(symbol: String, name: String, file_path: String) -> Self {
        let res = HistoricalPrice::load_from_csv(file_path);

        Self{
            symbol: symbol,
            name: name,
            historical_prices: res.unwrap()
        }
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn head(&self) -> String{
        self.historical_prices.iter().take(10).map(|hp| hp.to_string()).collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct HistoricalPrice {
    timestamp: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

impl HistoricalPrice {
    pub fn load_from_csv(path: String) -> Result<Vec<HistoricalPrice>,Box<dyn Error>> {
        let mut reader = csv::Reader::from_path(path)?;
        let mut historical_prices: Vec<HistoricalPrice>  = vec![];
        for result in reader.deserialize(){
            match result {
                Ok(record) => historical_prices.push(record),
                Err(err) => println!("error reading CSV from file: {}", err),
            }

        }
        Ok(historical_prices)
    }

    pub fn to_string(&self) -> String{
        format!("[{}]:{}/{}/{}/{} ({})\n",self.timestamp,self.open,self.high,self.low,self.close,self.volume)
    }
}
