use chrono::offset::Local;
use chrono::prelude::*;
use core::{f64, fmt};
use serde::Deserialize;
use std::error::Error;

pub struct Stock {
    symbol: String,
    name: String,
    historical_prices: HistoricalPrices,
}

impl fmt::Display for Stock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hps: String = self
            .historical_prices
            .iter()
            .map(|hp| hp.to_string())
            .collect();
        write!(
            f,
            "symbol: {}\nname: {}\nhistorical_prices:\n{}",
            self.symbol, self.name, hps
        )
    }
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

    pub fn to_plot(&self) -> Vec<HistoricalPricePlot> {
        self.data.iter().map(|hp| hp.to_plot()).collect()
    }

    pub fn range(&self) -> (f64, f64) {
        let (max, min) = self
            .data
            .iter()
            .fold((f64::NEG_INFINITY, f64::INFINITY), |(max, min), hp| {
                (max.max(hp.high), min.min(hp.low))
            });
        (max, min)
    }

    pub fn filter_by(&self, start: DateTime<Local>, end: DateTime<Local>) -> HistoricalPrices {
        HistoricalPrices {
            data: self
                .data
                .iter()
                .filter(|hp| hp.timestamp >= start && hp.timestamp <= end)
                .cloned()
                .collect(),
        }
    }
}

impl std::ops::Deref for HistoricalPrices {
    type Target = Vec<HistoricalPrice>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl fmt::Display for HistoricalPrices {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.data.iter().map(|hp| hp.to_string()).collect();
        write!(f, "{}", s)
    }
}

#[derive(Clone, Deserialize)]
pub struct HistoricalPrice {
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub timestamp: DateTime<Local>,
    open: f64,
    high: f64,
    low: f64,
    pub close: f64,
    volume: f64,
}

fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let timestamp = i64::deserialize(deserializer)?;
    DateTime::from_timestamp(timestamp, 0)
        .map(|dt| Local.from_utc_datetime(&dt.naive_utc()))
        .ok_or_else(|| serde::de::Error::custom("Invalid timestamp"))
}

pub struct HistoricalPricePlot {
    pub timestamp: DateTime<Local>,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
}

impl HistoricalPrice {
    pub fn to_plot(&self) -> HistoricalPricePlot {
        HistoricalPricePlot {
            timestamp: self.timestamp,
            open: self.open as f32,
            high: self.high as f32,
            low: self.low as f32,
            close: self.close as f32,
        }
    }
}

impl fmt::Display for HistoricalPrice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}]:{}/{}/{}/{} ({})\n",
            self.timestamp.to_string(),
            self.open,
            self.high,
            self.low,
            self.close,
            self.volume
        )
    }
}
