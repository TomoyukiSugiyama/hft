use chrono::prelude::*;
use chrono::offset::Local;
use serde::Deserialize;
use std::error::Error;

pub struct Stock {
    symbol: String,
    name: String,
    historical_prices: HistoricalPrices,
}

impl Stock {
    pub fn new(symbol: String, name: String, file_path: String) -> Self {
        let res = HistoricalPrices::load_from_csv(file_path);

        Self {
            symbol: symbol,
            name: name,
            historical_prices: res.unwrap(),
        }
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn head(&self, n: usize) -> String {
        self.historical_prices.head(n).to_string()
    }

    pub fn plot_data(&self) -> Vec<(DateTime<Local>, f32, f32, f32, f32)> {
        self.historical_prices.head(30).to_tupple()
    }
}

pub struct HistoricalPrices {
    data: Vec<HistoricalPrice>,
}

impl HistoricalPrices {
    pub fn load_from_csv(path: String) -> Result<HistoricalPrices, Box<dyn Error>> {
        let mut reader = csv::Reader::from_path(path)?;
        let mut historical_prices: Vec<HistoricalPrice> = vec![];
        for result in reader.deserialize() {
            match result {
                Ok(record) => historical_prices.push(record),
                Err(err) => println!("error reading CSV from file: {}", err),
            }
        }
        Ok(HistoricalPrices{data:historical_prices})
    }

    pub fn head(&self, n: usize) -> HistoricalPrices {
        HistoricalPrices {
            data: self.data[0..n].to_vec(),
        }
    }

    pub fn to_string(&self) -> String {
        self.data.iter().map(|hp| hp.to_string()).collect()
    }

    pub fn to_tupple(&self) -> Vec<(DateTime<Local>, f32, f32, f32, f32)> {
        self.data.iter().map(|hp| hp.to_tuple()).collect()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct HistoricalPrice {
    timestamp: i64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

impl HistoricalPrice {
    pub fn to_string(&self) -> String {
        format!(
            "[{}]:{}/{}/{}/{} ({})\n",
            self.parse_time().to_string(),
            self.open,
            self.high,
            self.low,
            self.close,
            self.volume
        )
    }

    pub fn to_tuple(&self) -> (DateTime<Local>, f32, f32, f32, f32) {
        (
            self.parse_time(),
            self.open as f32,
            self.high as f32,
            self.low as f32,
            self.close as f32,
        )
    }

    fn parse_time(&self) -> DateTime<Local> {
        Local.from_utc_datetime(&DateTime::from_timestamp_secs(self.timestamp)
        .unwrap()
        .naive_utc())
    }
}
