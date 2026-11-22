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
//! The server comes from `configure` (an app's settings), or else from the
//! environment: `INFLUXDB_URL` (default `http://localhost:8181`),
//! `INFLUXDB3_AUTH_TOKEN`, and, for a server behind Cloudflare Access
//! (`https://influx.atec.pub`, see tunnel/README.md), the service token in
//! `CF_ACCESS_CLIENT_ID` and `CF_ACCESS_CLIENT_SECRET`.

uniffi::setup_scaffolding!();

use std::sync::RwLock;
use std::time::{Duration, SystemTime};

const DEFAULT_URL: &str = "http://localhost:8181";
const DATABASE: &str = "financials";

#[derive(Debug, uniffi::Error)]
pub enum InfluxError {
    /// No token: not configured, and `INFLUXDB3_AUTH_TOKEN` is not set.
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

/// Where InfluxDB is and how to get in.
#[derive(Clone, Debug, PartialEq, uniffi::Record)]
pub struct Server {
    /// e.g. `http://localhost:8181`, or `https://influx.atec.pub`.
    pub url: String,
    /// InfluxDB's token.
    pub token: String,
    /// A Cloudflare Access service token, for a server behind Access.
    pub access_client_id: Option<String>,
    pub access_client_secret: Option<String>,
}

static SERVER: RwLock<Option<Server>> = RwLock::new(None);

/// Use this server from now on, rather than the environment's; `None` goes
/// back to the environment.
#[uniffi::export]
pub fn configure(server: Option<Server>) {
    *SERVER.write().unwrap() = server;
}

/// The configured server, else the environment's.
fn server() -> Result<Server, InfluxError> {
    if let Some(s) = SERVER.read().unwrap().clone() {
        return Ok(s);
    }
    let env = |k: &str| std::env::var(k).ok().filter(|v| !v.is_empty());
    Ok(Server {
        url: env("INFLUXDB_URL").unwrap_or_else(|| DEFAULT_URL.to_string()),
        token: env("INFLUXDB3_AUTH_TOKEN").ok_or(InfluxError::NoToken)?,
        access_client_id: env("CF_ACCESS_CLIENT_ID"),
        access_client_secret: env("CF_ACCESS_CLIENT_SECRET"),
    })
}

/// One value of a series: a `Date` in Swift.
#[derive(Clone, Debug, PartialEq, uniffi::Record)]
pub struct Point {
    pub time: SystemTime,
    pub value: f64,
}

/// The tickers with reported items in the last 400 days: companies still
/// filing. (Over all time the query reads every row, about 20 seconds; this
/// reads a year's partitions, about a tenth of a second.)
#[uniffi::export]
pub fn tickers() -> Result<Vec<String>, InfluxError> {
    let rows = query("SELECT DISTINCT ticker FROM reported_item_v15 \
        WHERE time > now() - INTERVAL '400 days' ORDER BY ticker")?;
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

/// A series of a company's dashboard: a reported `item` for a `period`
/// (`3m`, `12m`, `pit`), or a field of `market_v1` with period `market`, in
/// a `currency`.
#[derive(Clone, Debug, PartialEq, uniffi::Record)]
pub struct TimeSeries {
    pub item: String,
    pub period: String,
    pub currency: String,
    pub points: Vec<Point>,
}

/// Everything a company's dashboard draws, in one query: the reported
/// `items` for each of `periods`, and the `market` fields of `market_v1`, in
/// every currency the company reports in (more than one if it changed).
/// One scan of the company's files, rather than one per line (a series
/// costs about 0.3 s, the whole of a dashboard about 1 s).
#[uniffi::export]
pub fn dashboard(ticker: String, items: Vec<String>, periods: Vec<String>,
                 market: Vec<String>) -> Result<Vec<TimeSeries>, InfluxError> {
    let list = |v: &[String]| v.iter().map(|s| literal(s)).collect::<Vec<_>>().join(", ");
    let t = literal(&ticker);
    let mut parts = Vec::new();
    if !items.is_empty() && !periods.is_empty() {
        parts.push(format!(
            "SELECT time, item, period, currency, value FROM reported_item_v15 WHERE ticker = {t} \
             AND item IN ({}) AND period IN ({})", list(&items), list(&periods)));
    }
    for field in &market {
        if !field.chars().all(|ch| ch.is_ascii_lowercase() || ch == '_') {
            return Err(InfluxError::Query { message: format!("not a field: {}", field) });
        }
        parts.push(format!(
            "SELECT time, '{field}' AS item, 'market' AS period, currency, \"{field}\" AS value \
             FROM market_v1 WHERE ticker = {t} AND \"{field}\" IS NOT NULL"));
    }
    if parts.is_empty() {
        return Ok(Vec::new());
    }
    let mut by_series: std::collections::BTreeMap<(String, String, String), Vec<Point>> = Default::default();
    for row in query(&parts.join(" UNION ALL "))? {
        let get = |k: &str| row.get(k).and_then(|v| v.as_str()).map(str::to_string);
        let (Some(item), Some(period), Some(currency), Some(time), Some(value)) = (get("item"),
            get("period"), get("currency"), get("time").as_deref().and_then(parse_time),
            row.get("value").and_then(|v| v.as_f64()))
        else {
            return Err(InfluxError::Query { message: format!("unexpected row: {:?}", row) });
        };
        by_series.entry((item, period, currency)).or_default().push(Point { time, value });
    }
    Ok(by_series.into_iter().map(|((item, period, currency), mut points)| {
        points.sort_by(|a, b| a.time.cmp(&b.time));
        TimeSeries { item, period, currency, points }
    }).collect())
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
    let server = server()?;
    if server.token.is_empty() {
        return Err(InfluxError::NoToken);
    }
    let err = |e: reqwest::Error| InfluxError::Query { message: e.without_url().to_string() };
    let mut req = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(120)).build().map_err(err)?
        .post(format!("{}/api/v3/query_sql", server.url.trim_end_matches('/')))
        .bearer_auth(&server.token);
    if let (Some(id), Some(secret)) = (&server.access_client_id, &server.access_client_secret) {
        req = req.header("CF-Access-Client-Id", id).header("CF-Access-Client-Secret", secret);
    }
    let res = req
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

    #[test]
    #[ignore]
    fn live_dashboard() {
        let series = dashboard("nvda".into(),
            vec!["TotalAssets".into(), "Revenue".into()], vec!["pit".into(), "3m".into()],
            vec!["market_cap".into()]).unwrap();
        let find = |i: &str, p: &str| series.iter().find(|s| s.item == i && s.period == p && s.currency == "USD");
        let assets = find("TotalAssets", "pit").unwrap();
        assert_eq!(assets.points, reported("nvda".into(), "TotalAssets".into(), "pit".into(), "USD".into()).unwrap());
        assert!(find("Revenue", "3m").is_some() && find("market_cap", "market").is_some());
        assert!(find("Revenue", "12m").is_none());
    }

    /// Through Cloudflare Access: `source ~/.cloudflared/equities-app.env`,
    /// then `cargo test -p influx -- --ignored`.
    #[test]
    #[ignore]
    fn through_access() {
        let env = |k: &str| std::env::var(k).ok();
        let server = Server {
            url: "https://influx.atec.pub".into(),
            token: env("INFLUXDB3_AUTH_TOKEN").unwrap(),
            access_client_id: env("CF_ACCESS_CLIENT_ID"),
            access_client_secret: env("CF_ACCESS_CLIENT_SECRET"),
        };
        configure(Some(Server { access_client_id: None, ..server.clone() }));
        assert!(tickers().is_err(), "Access let a request without its service token through");
        configure(Some(server));
        assert!(tickers().unwrap().contains(&"net".to_string()));
        configure(None);
    }
}
