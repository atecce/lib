//! Read the series the `lib` executable writes to InfluxDB 3 (database
//! `financials`), for apps: exported to Swift through uniffi.
//!
//! Measurements:
//!
//! - `reported_item_v15`: one field, `value`; tags `ticker` (lower case),
//!   `item` (an `equities::Item`, e.g. `TotalAssets`), `period` (`3m`, `12m`,
//!   or `pit` for balances at a point in time) and `currency`.
//! - `market_v1`: fields `close`, `market_cap`, `price_to_book`,
//!   `return_on_equity`, `earnings_yield`; tags `ticker` and `currency`.
//!
//! The server and token come from `INFLUXDB_URL` (default
//! `http://localhost:8181`) and `INFLUXDB3_AUTH_TOKEN`.

uniffi::setup_scaffolding!();

use std::time::{Duration, SystemTime};

const DEFAULT_URL: &str = "http://localhost:8181";
const DATABASE: &str = "financials";

#[derive(Debug, uniffi::Error)]
pub enum InfluxError {
    /// `INFLUXDB3_AUTH_TOKEN` is not set.
    NoToken,
    /// The request failed, or InfluxDB answered with an error.
    Query { message: String },
}

impl std::fmt::Display for InfluxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InfluxError::NoToken => write!(f, "INFLUXDB3_AUTH_TOKEN is not set"),
            InfluxError::Query { message } => write!(f, "{}", message),
        }
    }
}

impl std::error::Error for InfluxError {}

/// One value of a series: a `Date` in Swift.
#[derive(Clone, Debug, PartialEq, uniffi::Record)]
pub struct Point {
    pub time: SystemTime,
    pub value: f64,
}

/// The tickers with reported items.
#[uniffi::export]
pub fn tickers() -> Result<Vec<String>, InfluxError> {
    let rows = query("SELECT DISTINCT ticker FROM reported_item_v15 ORDER BY ticker")?;
    Ok(rows.iter().filter_map(|r| r.get("ticker")?.as_str().map(str::to_string)).collect())
}

/// The currencies a ticker reports in (a company that changed its reporting
/// currency has more than one).
#[uniffi::export]
pub fn currencies(ticker: String) -> Result<Vec<String>, InfluxError> {
    let rows = query(&format!(
        "SELECT DISTINCT currency FROM reported_item_v15 WHERE ticker = {} ORDER BY currency",
        literal(&ticker)))?;
    Ok(rows.iter().filter_map(|r| r.get("currency")?.as_str().map(str::to_string)).collect())
}

/// A reported item over time: `item` as in `equities::Item` (`TotalAssets`),
/// `period` one of `3m`, `12m`, `pit`.
#[uniffi::export]
pub fn reported(ticker: String, item: String, period: String, currency: String) -> Result<Vec<Point>, InfluxError> {
    points(&format!(
        "SELECT time, value FROM reported_item_v15 WHERE ticker = {} AND item = {} AND period = {} \
         AND currency = {} ORDER BY time",
        literal(&ticker), literal(&item), literal(&period), literal(&currency)), "value")
}

/// A market series: `field` one of `close`, `market_cap`, `price_to_book`,
/// `return_on_equity`, `earnings_yield`.
#[uniffi::export]
pub fn market(ticker: String, field: String, currency: String) -> Result<Vec<Point>, InfluxError> {
    if !field.chars().all(|c| c.is_ascii_lowercase() || c == '_') {
        return Err(InfluxError::Query { message: format!("not a field: {}", field) });
    }
    points(&format!(
        "SELECT time, \"{field}\" FROM market_v1 WHERE ticker = {} AND currency = {} \
         AND \"{field}\" IS NOT NULL ORDER BY time",
        literal(&ticker), literal(&currency)), &field)
}

fn points(sql: &str, field: &str) -> Result<Vec<Point>, InfluxError> {
    query(sql)?.iter().map(|row| {
        let time = row.get("time").and_then(|t| t.as_str()).and_then(parse_time);
        let value = row.get(field).and_then(|v| v.as_f64());
        match (time, value) {
            (Some(time), Some(value)) => Ok(Point { time, value }),
            _ => Err(InfluxError::Query { message: format!("unexpected row: {:?}", row) }),
        }
    }).collect()
}

/// InfluxDB's times are UTC without a zone: `2026-07-26T00:00:00`.
fn parse_time(s: &str) -> Option<SystemTime> {
    let t = chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f").ok()?.and_utc();
    let secs = u64::try_from(t.timestamp()).ok()?;
    Some(SystemTime::UNIX_EPOCH + Duration::new(secs, t.timestamp_subsec_nanos()))
}

/// A SQL string literal.
fn literal(s: &str) -> String {
    format!("'{}'", s.replace('\'', "''"))
}

type Row = serde_json::Map<String, serde_json::Value>;

fn query(sql: &str) -> Result<Vec<Row>, InfluxError> {
    let token = std::env::var("INFLUXDB3_AUTH_TOKEN").map_err(|_| InfluxError::NoToken)?;
    let url = std::env::var("INFLUXDB_URL").unwrap_or_else(|_| DEFAULT_URL.to_string());
    let err = |e: reqwest::Error| InfluxError::Query { message: e.to_string() };
    let res = reqwest::blocking::Client::new()
        .post(format!("{}/api/v3/query_sql", url))
        .bearer_auth(token)
        .header("Content-Type", "application/json")
        .body(serde_json::json!({ "db": DATABASE, "q": sql, "format": "json" }).to_string())
        .send().map_err(err)?;
    let status = res.status();
    let body = res.text().map_err(err)?;
    if !status.is_success() {
        return Err(InfluxError::Query { message: format!("{}: {}", status, body) });
    }
    serde_json::from_str(&body).map_err(|e| InfluxError::Query { message: e.to_string() })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn times() {
        let t = parse_time("2026-07-26T00:00:00").unwrap();
        assert_eq!(t.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(), 1_785_024_000);
        assert!(parse_time("2026-07-26").is_none());
    }

    #[test]
    fn literals() {
        assert_eq!(literal("o'brien"), "'o''brien'");
    }

    /// Against the local server: `cargo test -p influx -- --ignored`.
    #[test]
    #[ignore]
    fn live() {
        assert!(tickers().unwrap().contains(&"nvda".to_string()));
        let assets = reported("nvda".into(), "TotalAssets".into(), "pit".into(), "USD".into()).unwrap();
        assert!(assets.len() > 20 && assets.windows(2).all(|w| w[0].time < w[1].time));
        assert!(!market("nvda".into(), "market_cap".into(), "USD".into()).unwrap().is_empty());
        assert!(market("nvda".into(), "x; drop".into(), "USD".into()).is_err());
    }
}
