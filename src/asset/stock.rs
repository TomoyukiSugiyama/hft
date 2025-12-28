use chrono::offset::Local;
use chrono::prelude::*;
use core::f64;
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

    pub fn historical_prices(&self) -> &HistoricalPrices {
        &self.historical_prices
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
        Ok(HistoricalPrices {
            data: historical_prices,
        })
    }

    pub fn head(&self, n: usize) -> HistoricalPrices {
        HistoricalPrices {
            data: self.data[0..n].to_vec(),
        }
    }

    pub fn to_string(&self) -> String {
        self.data.iter().map(|hp| hp.to_string()).collect()
    }

    pub fn to_plot(&self) -> Vec<HistoricalPricePlot> {
        self.data.iter().map(|hp| hp.to_plot()).collect()
    }

    pub fn range(&self) -> (f64, f64) {
        let (max, min) = self.data.iter().fold(
            (f64::NEG_INFINITY, f64::INFINITY),
            |(max, min), hp| (max.max(hp.high), min.min(hp.low)),
        );
        (max, min)
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

pub struct HistoricalPricePlot {
    pub timestamp: DateTime<Local>,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
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

    pub fn to_plot(&self) -> HistoricalPricePlot {
        HistoricalPricePlot {
            timestamp: self.parse_time(),
            open: self.open as f32,
            high: self.high as f32,
            low: self.low as f32,
            close: self.close as f32,
        }
    }

    fn parse_time(&self) -> DateTime<Local> {
        Local.from_utc_datetime(
            &DateTime::from_timestamp_secs(self.timestamp)
                .unwrap()
                .naive_utc(),
        )
    }
}
