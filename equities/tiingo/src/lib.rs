//! Tiingo's end-of-day stock prices as a source: each symbol's daily history,
//! kept under `cache/<SYMBOL>.json` as Tiingo returns it, oldest first.
//!
//! The free plan allows 500 distinct symbols a month, 50 requests an hour and
//! 1,000 a day; `Client` paces itself to that across runs (`cache/requests.log`).
//! The data is licensed for personal, internal use only.
//!
//! `Cache` reads prices and never touches the network. With the `fetch`
//! feature, `Client` downloads into it (with a key from `TIINGO_API_KEY`),
//! and `tiingo-fetch` runs it.

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[cfg(feature = "fetch")]
mod client;
#[cfg(feature = "fetch")]
pub use client::Client;

/// One trading day.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Price {
    /// "2021-10-01T00:00:00.000Z"
    pub date: String,
    /// As traded that day, not adjusted for later splits.
    pub close: f64,
    #[serde(rename = "adjClose")]
    pub adj_close: f64,
    /// Above 1 on the day a split takes effect: 3.0 for Tesla's 3-for-1.
    #[serde(rename = "splitFactor", default = "one")]
    pub split_factor: f64,
    #[serde(rename = "divCash", default)]
    pub dividend: f64,
}

fn one() -> f64 { 1.0 }

impl Price {
    pub fn day(&self) -> Option<NaiveDate> {
        NaiveDate::parse_from_str(self.date.get(..10)?, "%Y-%m-%d").ok()
    }
}

#[derive(Debug)]
pub enum TiingoError {
    /// Not in the cache: run `tiingo-fetch` first.
    NotFetched(PathBuf),
}

impl std::fmt::Display for TiingoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TiingoError::NotFetched(p) => write!(f, "no prices fetched yet: {}", p.display()),
        }
    }
}

impl Error for TiingoError {}

/// Tiingo's symbol for a ticker: lower case, with a dash for a share class
/// ("BF.B" is "bf-b"). Berkshire is priced by its Class B shares.
pub fn symbol(ticker: &str) -> String {
    match ticker.to_uppercase().as_str() {
        "BRK" => "brk-b".to_string(),
        t => t.replace('.', "-").to_lowercase(),
    }
}

/// The cache, read only.
#[derive(Debug, Clone)]
pub struct Cache {
    dir: PathBuf,
}

impl Cache {
    pub fn new(dir: impl Into<PathBuf>) -> Cache {
        Cache { dir: dir.into() }
    }

    /// `cache/` next to this crate's manifest; `TIINGO_CACHE` names another.
    pub fn default_dir() -> PathBuf {
        std::env::var_os("TIINGO_CACHE").map(PathBuf::from)
            .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("cache"))
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    fn path(&self, ticker: &str) -> PathBuf {
        self.dir.join(format!("{}.json", symbol(ticker)))
    }

    /// The ticker's daily prices, oldest first.
    pub fn prices(&self, ticker: &str) -> Result<Vec<Price>, Box<dyn Error>> {
        let path = self.path(ticker);
        match fs::read(&path) {
            Ok(bytes) => Ok(serde_json::from_slice(&bytes)?),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(TiingoError::NotFetched(path).into()),
            Err(e) => Err(e.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbols() {
        assert_eq!(symbol("TSLA"), "tsla");
        assert_eq!(symbol("BF.B"), "bf-b");
        assert_eq!(symbol("BRK"), "brk-b");
    }
}
