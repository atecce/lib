//! Tests of trading strategies on history, starting where the efficient market
//! hypothesis did (equities/FORECASTING.md):
//!
//! - [`random_walk`]: does a price's past predict its future? Fama (1965):
//!   serial correlation of price changes, and runs of rises and falls.
//! - [`filter`]: Alexander's filter rules (1961, 1964), as Fama and Blume
//!   (1966) tested them against buying and holding.
//!
//! Prices come from the tiingo cache; returns are daily log changes of the
//! adjusted close (splits and dividends folded in), as Fama used price changes
//! with dividends added back.

use std::error::Error;

use chrono::NaiveDate;

pub mod filter;
pub mod random_walk;

/// A stock's daily adjusted closes.
#[derive(Clone, Debug)]
pub struct Prices {
    pub ticker: String,
    pub dates: Vec<NaiveDate>,
    /// Adjusted for splits and dividends: comparable across the whole history.
    pub adjusted: Vec<f64>,
}

impl Prices {
    /// From the tiingo cache, between two dates (inclusive), if given.
    pub fn load(cache: &tiingo::Cache, ticker: &str, from: Option<NaiveDate>, to: Option<NaiveDate>)
        -> Result<Prices, Box<dyn Error>> {
        let mut dates = Vec::new();
        let mut adjusted = Vec::new();
        for p in cache.prices(ticker)? {
            let date = NaiveDate::parse_from_str(&p.date[..10], "%Y-%m-%d")?;
            if from.is_some_and(|f| date < f) || to.is_some_and(|t| date > t) || p.adj_close <= 0.0 {
                continue;
            }
            dates.push(date);
            adjusted.push(p.adj_close);
        }
        Ok(Prices { ticker: ticker.to_string(), dates, adjusted })
    }

    /// Daily log changes: ln(P_t / P_{t-1}).
    pub fn log_returns(&self) -> Vec<f64> {
        self.adjusted.windows(2).map(|w| (w[1] / w[0]).ln()).collect()
    }
}

pub fn mean(x: &[f64]) -> f64 {
    if x.is_empty() { f64::NAN } else { x.iter().sum::<f64>() / x.len() as f64 }
}
