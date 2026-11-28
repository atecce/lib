//! Downloading into the cache, within the free plan's limits.
//!
//! Every request is logged with its time and symbol in `cache/requests.log`,
//! so separate runs share the limits: no more than `PER_HOUR` in any hour or
//! `PER_DAY` in any day (the client waits when it would exceed them), and a
//! warning past `SYMBOLS_PER_MONTH` distinct symbols in a calendar month.

use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

use chrono::{Datelike, Local, NaiveDate, NaiveDateTime};

use crate::{symbol, Cache, Price};

const PER_HOUR: usize = 50;
const PER_DAY: usize = 1000;
const SYMBOLS_PER_MONTH: usize = 500;

pub struct Client {
    http: reqwest::blocking::Client,
    key: String,
    cache: Cache,
    requests: std::cell::Cell<usize>,
}

impl Client {
    pub fn new(key: impl Into<String>, cache: Cache) -> Result<Client, Box<dyn Error>> {
        Ok(Client {
            http: reqwest::blocking::Client::builder().timeout(Duration::from_secs(120)).build()?,
            key: key.into(),
            cache,
            requests: std::cell::Cell::new(0),
        })
    }

    pub fn cache(&self) -> &Cache {
        &self.cache
    }

    /// Requests sent by this client.
    pub fn requests(&self) -> usize {
        self.requests.get()
    }

    fn log_path(&self) -> PathBuf {
        self.cache.dir().join("requests.log")
    }

    /// Past requests: (time, symbol), from the log.
    fn log(&self) -> Vec<(NaiveDateTime, String)> {
        fs::read_to_string(self.log_path()).unwrap_or_default().lines()
            .filter_map(|l| {
                let (t, s) = l.split_once(' ')?;
                Some((NaiveDateTime::parse_from_str(t, "%Y-%m-%dT%H:%M:%S").ok()?, s.to_string()))
            })
            .collect()
    }

    /// Wait until a request fits the hourly and daily limits, then log it.
    fn reserve(&self, sym: &str) -> Result<(), Box<dyn Error>> {
        loop {
            let now = Local::now().naive_local();
            let log = self.log();
            let in_hour: Vec<_> = log.iter().filter(|(t, _)| now - *t < chrono::Duration::hours(1)).collect();
            let in_day: Vec<_> = log.iter().filter(|(t, _)| now - *t < chrono::Duration::days(1)).collect();
            let wait = if in_hour.len() >= PER_HOUR {
                Some(in_hour.iter().map(|(t, _)| *t).min().unwrap() + chrono::Duration::hours(1) - now)
            } else if in_day.len() >= PER_DAY {
                Some(in_day.iter().map(|(t, _)| *t).min().unwrap() + chrono::Duration::days(1) - now)
            } else {
                None
            };
            match wait {
                Some(w) => {
                    let secs = w.num_seconds().max(1) as u64 + 1;
                    eprintln!("tiingo: at the request limit, waiting {}m{:02}s", secs / 60, secs % 60);
                    sleep(Duration::from_secs(secs));
                },
                None => {
                    let month = |t: &NaiveDateTime| (t.year(), t.month());
                    let mut symbols: Vec<&str> = log.iter().filter(|(t, _)| month(t) == month(&now)).map(|(_, s)| s.as_str()).collect();
                    symbols.push(sym);
                    symbols.sort();
                    symbols.dedup();
                    if symbols.len() > SYMBOLS_PER_MONTH {
                        eprintln!("tiingo: {} distinct symbols this month, past the free plan's {}", symbols.len(), SYMBOLS_PER_MONTH);
                    }
                    fs::create_dir_all(self.cache.dir())?;
                    let mut f = fs::OpenOptions::new().create(true).append(true).open(self.log_path())?;
                    writeln!(f, "{} {}", now.format("%Y-%m-%dT%H:%M:%S"), sym)?;
                    self.requests.set(self.requests.get() + 1);
                    return Ok(());
                },
            }
        }
    }

    /// Bring a ticker's prices up to date: its whole history the first time,
    /// then the days since the last one cached. Returns the days added.
    pub fn fetch(&self, ticker: &str) -> Result<usize, Box<dyn Error>> {
        self.fetch_from(ticker, false)
    }

    /// A ticker's whole history again, replacing what's cached: for tickers
    /// first fetched from 1990 (before `HISTORY_START`), and to make the
    /// adjusted closes consistent as of today. Returns the days now cached.
    pub fn fetch_full(&self, ticker: &str) -> Result<usize, Box<dyn Error>> {
        self.fetch_from(ticker, true)
    }

    fn fetch_from(&self, ticker: &str, full: bool) -> Result<usize, Box<dyn Error>> {
        let sym = symbol(ticker);
        let mut prices = if full { Vec::new() } else { self.cache.prices(ticker).unwrap_or_default() };
        let last = prices.last().and_then(Price::day);
        let today = Local::now().date_naive();
        // Closes are final the next day; nothing to ask for before then.
        if last.is_some_and(|d| d >= today - chrono::Duration::days(1)) {
            return Ok(0);
        }
        let start = last.map(|d| d + chrono::Duration::days(1)).unwrap_or(HISTORY_START);
        self.reserve(&sym)?;
        // The key goes in a header, never the URL (errors quote URLs).
        let res = self.http.get(format!("https://api.tiingo.com/tiingo/daily/{}/prices?startDate={}", sym, start))
            .header("Authorization", format!("Token {}", self.key))
            .send()
            .and_then(|r| r.error_for_status())?;
        let body = res.bytes()?;
        // Refusals come back as 200 with {"detail": "..."} (the monthly
        // symbol limit): say what Tiingo said, not that it isn't prices.
        let new: Vec<Price> = match serde_json::from_slice(&body) {
            Ok(prices) => prices,
            Err(e) => match serde_json::from_slice::<serde_json::Value>(&body).ok()
                .and_then(|v| v.get("detail").and_then(|d| d.as_str()).map(String::from)) {
                Some(detail) => return Err(format!("Tiingo: {}", detail).into()),
                None => return Err(e.into()),
            },
        };
        let added = new.iter().filter(|p| p.day().is_some_and(|d| last.is_none_or(|l| d > l))).count();
        prices.extend(new.into_iter().filter(|p| p.day().is_some_and(|d| last.is_none_or(|l| d > l))));
        let path = self.cache.dir().join(format!("{}.json", sym));
        let tmp = path.with_extension("part");
        fs::write(&tmp, serde_json::to_vec(&prices)?)?;
        fs::rename(&tmp, &path)?;
        Ok(if full { prices.len() } else { added })
    }
}

/// Where a first fetch starts: before Tiingo's earliest US prices (1962), so
/// it gets a ticker's whole history. (Until 2026-11-23 it was 1990.)
pub const HISTORY_START: NaiveDate = NaiveDate::from_ymd_opt(1962, 1, 1).unwrap();
