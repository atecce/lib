//! The Connect protocol (connectrpc.com) for proto/equities/v1/equities.proto:
//! each call a POST of one message to `/equities.v1.Equities/<Method>`, as
//! protobuf (`application/proto`) or JSON (`application/json`), answered in
//! kind; an error is a status and `{"code", "message"}` in JSON.
//!
//! Only unary calls, which is all the service has: no streaming, so no
//! tonic, and nothing Cloudflare's tunnel and Access don't already pass.

use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::State;
use axum::Extension;
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use chrono::NaiveDate;
use prost::Message;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::App;

/// The messages and their JSON form, generated from the .proto (build.rs).
pub mod pb {
    include!(concat!(env!("OUT_DIR"), "/equities.v1.rs"));
    include!(concat!(env!("OUT_DIR"), "/equities.v1.serde.rs"));
}

pub fn routes() -> Router<Arc<App>> {
    Router::new()
        .route("/equities.v1.Equities/Tickers", post(tickers))
        .route("/equities.v1.Equities/Dashboard", post(dashboard))
        .route("/equities.v1.Equities/Holdings", post(holdings))
        .route("/equities.v1.Equities/Macro", post(macro_series))
}

/// A Connect error: one of the protocol's codes, and a message.
pub struct ConnectError {
    code: &'static str,
    message: String,
}

impl ConnectError {
    pub fn new(code: &'static str, message: impl Into<String>) -> ConnectError {
        ConnectError { code, message: message.into() }
    }

    fn invalid(message: impl Into<String>) -> ConnectError {
        ConnectError::new("invalid_argument", message)
    }

    fn status(&self) -> StatusCode {
        match self.code {
            "invalid_argument" | "failed_precondition" | "out_of_range" => StatusCode::BAD_REQUEST,
            "unauthenticated" => StatusCode::UNAUTHORIZED,
            "permission_denied" => StatusCode::FORBIDDEN,
            "not_found" => StatusCode::NOT_FOUND,
            "unimplemented" => StatusCode::NOT_IMPLEMENTED,
            "unavailable" => StatusCode::SERVICE_UNAVAILABLE,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<tokio_postgres::Error> for ConnectError {
    fn from(e: tokio_postgres::Error) -> ConnectError {
        ConnectError::new("internal", e.to_string())
    }
}

impl From<crate::ApiError> for ConnectError {
    fn from(e: crate::ApiError) -> ConnectError {
        ConnectError::new(if e.0.is_server_error() { "internal" } else { "invalid_argument" }, e.1)
    }
}

impl IntoResponse for ConnectError {
    fn into_response(self) -> Response {
        let status = self.status();
        if status.is_server_error() { eprintln!("{}: {}", self.code, self.message) }
        (status, Json(serde_json::json!({ "code": self.code, "message": self.message }))).into_response()
    }
}

/// How a call's messages are written: its Content-Type.
#[derive(Clone, Copy)]
enum Codec {
    Proto,
    Json,
}

impl Codec {
    /// Or the protocol's answer to another: 415, Unsupported Media Type.
    fn of(headers: &HeaderMap) -> Result<Codec, Response> {
        let content_type = headers.get(header::CONTENT_TYPE).and_then(|v| v.to_str().ok()).unwrap_or("");
        match content_type.split(';').next().unwrap_or("").trim() {
            "application/proto" => Ok(Codec::Proto),
            "application/json" => Ok(Codec::Json),
            _ => Err((StatusCode::UNSUPPORTED_MEDIA_TYPE, [("accept-post", "application/proto, application/json")]).into_response()),
        }
    }

    fn decode<M: Message + Default + DeserializeOwned>(self, body: &[u8]) -> Result<M, ConnectError> {
        match self {
            Codec::Proto => M::decode(body).map_err(|e| ConnectError::invalid(e.to_string())),
            // An empty body is the empty message.
            Codec::Json if body.iter().all(u8::is_ascii_whitespace) => Ok(M::default()),
            Codec::Json => serde_json::from_slice(body).map_err(|e| ConnectError::invalid(e.to_string())),
        }
    }

    fn encode<M: Message + Serialize>(self, m: &M) -> Response {
        match self {
            Codec::Proto => ([(header::CONTENT_TYPE, "application/proto")], m.encode_to_vec()).into_response(),
            Codec::Json => Json(m).into_response(),
        }
    }
}

async fn tickers(State(app): State<Arc<App>>, headers: HeaderMap, body: Bytes) -> Result<Response, ConnectError> {
    let codec = match Codec::of(&headers) { Ok(c) => c, Err(refusal) => return Ok(refusal) };
    let _: pb::TickersRequest = codec.decode(&body)?;
    let tickers = crate::load_tickers(&*app.client().await?).await?;
    Ok(codec.encode(&pb::TickersResponse { tickers }))
}

async fn dashboard(State(app): State<Arc<App>>, headers: HeaderMap, body: Bytes) -> Result<Response, ConnectError> {
    let codec = match Codec::of(&headers) { Ok(c) => c, Err(refusal) => return Ok(refusal) };
    let req: pb::DashboardRequest = codec.decode(&body)?;
    if req.ticker.trim().is_empty() {
        return Err(ConnectError::invalid("no ticker"));
    }
    let items = req.items.iter().map(|&i| match pb::Item::try_from(i) {
        Ok(pb::Item::Unspecified) | Err(_) => Err(ConnectError::invalid(format!("not an item: {}", i))),
        // The prost variant's name is equities::Item's: TotalAssets.
        Ok(item) => Ok(format!("{:?}", item)),
    }).collect::<Result<Vec<_>, _>>()?;
    let periods = req.periods.iter().map(|&p| match pb::Period::try_from(p) {
        Ok(p) => period_name(p).ok_or_else(|| ConnectError::invalid(format!("not a period: {}", p.as_str_name()))),
        Err(_) => Err(ConnectError::invalid(format!("not a period: {}", p))),
    }).collect::<Result<Vec<_>, _>>()?;
    let fields = req.market.iter().map(|&f| match pb::MarketField::try_from(f) {
        Ok(f) => field_name(f).ok_or_else(|| ConnectError::invalid(format!("not a market field: {}", f.as_str_name()))),
        Err(_) => Err(ConnectError::invalid(format!("not a market field: {}", f))),
    }).collect::<Result<Vec<_>, _>>()?;

    let periods: Vec<String> = periods.into_iter().map(String::from).collect();
    let fields: Vec<String> = fields.into_iter().map(String::from).collect();
    let loaded = crate::load_dashboard(&*app.client().await?, &req.ticker, &items, &periods, &fields).await?;

    let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
    let series = loaded.into_iter().filter_map(|((name, period, currency), points)| {
        let of = if period == "market" {
            pb::series::Of::Market(field_of(&name)? as i32)
        } else {
            pb::series::Of::Reported(pb::Reported {
                item: pb::Item::from_str_name(&format!("ITEM_{}", screaming(&name)))? as i32,
                period: period_of(&period)? as i32,
            })
        };
        Some(pb::Series {
            of: Some(of),
            currency,
            points: points.into_iter()
                .map(|(date, value)| pb::Point { epoch_day: (date - epoch).num_days() as i32, value })
                .collect(),
        })
    }).collect();
    Ok(codec.encode(&pb::DashboardResponse { series }))
}

/// The most FRED series one Macro call may ask for.
const MACRO_MAX_SERIES: usize = 64;

async fn macro_series(State(app): State<Arc<App>>, headers: HeaderMap, body: Bytes) -> Result<Response, ConnectError> {
    let codec = match Codec::of(&headers) { Ok(c) => c, Err(refusal) => return Ok(refusal) };
    let req: pb::MacroRequest = codec.decode(&body)?;
    if req.series.len() > MACRO_MAX_SERIES {
        return Err(ConnectError::invalid(format!("at most {} series", MACRO_MAX_SERIES)));
    }
    let ids: Vec<String> = req.series.iter().map(|s| s.trim().to_uppercase()).collect();
    let rows = app.client().await?.query(
        "SELECT s.id, coalesce(s.title, ''), o.date, o.value
         FROM fred_series s JOIN fred_observation o ON o.series_id = s.id
         WHERE s.id = ANY($1) AND o.value IS NOT NULL
         ORDER BY s.id, o.date", &[&ids]).await?;
    let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
    let mut series: Vec<pb::MacroSeries> = Vec::new();
    for r in rows {
        let id: String = r.get(0);
        if series.last().is_none_or(|s| s.id != id) {
            series.push(pb::MacroSeries { id, title: r.get(1), points: vec![] });
        }
        let date: NaiveDate = r.get(2);
        series.last_mut().unwrap().points.push(pb::Point { epoch_day: (date - epoch).num_days() as i32, value: r.get(3) });
    }
    Ok(codec.encode(&pb::MacroResponse { series }))
}

/// How long a sync stays fresh: a Holdings call asking for a refresh within
/// it reads what's stored.
const SYNC_FRESH_SECONDS: i64 = 15 * 60;

/// What a short of `short` shares brought in: its newest short sales, as
/// (shares, proceeds), newest first. The older ones were covered first
/// (FIFO), so the newest are what's still open. None if the sales on record
/// (Plaid's two years) don't add up to the position.
pub fn short_proceeds(short: f64, sales_newest_first: &[(f64, f64)]) -> Option<f64> {
    let (mut left, mut proceeds) = (short, 0.0);
    for &(shares, amount) in sales_newest_first {
        if left <= 1e-9 { break }
        if shares <= 0.0 { continue }
        let take = shares.min(left);
        proceeds += amount * take / shares;
        left -= take;
    }
    (left <= 1e-6).then_some(proceeds)
}

/// The ticker positions are measured against: the portfolio's r.
pub const BASELINE: &str = "MINT";

/// The market, for over/under market: the S&P 500 (SPY, total return).
pub const MARKET: &str = "SPY";

/// A ticker's total return level by day (Tiingo's adjusted closes), oldest
/// first; empty, and logged, when it isn't in the cache.
fn total_return_index(ticker: &str) -> Vec<(NaiveDate, f64)> {
    tiingo::Cache::new(tiingo::Cache::default_dir()).prices(ticker)
        .map(|p| p.iter().filter_map(|p| Some((p.day()?, p.adj_close))).collect())
        .unwrap_or_else(|e| { eprintln!("{}: {}", ticker, e); Vec::new() })
}

/// What the dollars behind `held` shares would be worth had they gone into
/// the baseline instead, on the same days: (cost, value now). The buys are
/// (day, shares, dollars), newest first; the older ones were sold first
/// (FIFO), so the newest are what's held. `index` is the baseline's total
/// return level (adjusted closes) by day, oldest first; a buy is matched to
/// the last level on or before its day. None if the buys on record don't
/// account for every share held, or one predates the index.
pub fn baseline(held: f64, buys_newest_first: &[(NaiveDate, f64, f64)], index: &[(NaiveDate, f64)]) -> Option<Measured> {
    let now = index.last()?.1;
    let level = |day: NaiveDate| -> Option<f64> {
        let i = index.partition_point(|(d, _)| *d <= day);
        (i > 0).then(|| index[i - 1].1)
    };
    let (mut left, mut cost, mut value, mut since) = (held, 0.0, 0.0, None);
    for &(day, shares, dollars) in buys_newest_first {
        if left <= 1e-9 { break }
        if shares <= 0.0 { continue }
        let take = shares.min(left);
        let part = dollars * take / shares;
        cost += part;
        value += part * now / level(day)?;
        since = Some(day);
        left -= take;
    }
    (left <= 1e-6).then_some(Measured { cost, value, since: since? })
}

/// A position measured against the baseline: what its held shares' buys
/// cost, what those dollars would be worth in the baseline, and the day of
/// the oldest of those buys.
#[derive(Debug, PartialEq)]
pub struct Measured {
    pub cost: f64,
    pub value: f64,
    pub since: NaiveDate,
}

/// Dividends a position received from `since` on: cash in, positive (Plaid
/// writes money in as a negative amount; a short pays dividends out, which
/// count against it). `paid` is (day, amount as Plaid wrote it).
pub fn dividends_since(since: NaiveDate, paid: &[(NaiveDate, f64)]) -> f64 {
    paid.iter().filter(|(d, _)| *d >= since).map(|(_, amount)| -amount).sum()
}

async fn holdings(State(app): State<Arc<App>>, who: Option<Extension<crate::access::Identity>>, headers: HeaderMap, body: Bytes)
    -> Result<Response, ConnectError>
{
    let codec = match Codec::of(&headers) { Ok(c) => c, Err(refusal) => return Ok(refusal) };
    let req: pb::HoldingsRequest = codec.decode(&body)?;
    // The family may read the dashboards; the portfolio is the owner's.
    if app.access.is_some() {
        let allowed = match who.as_deref() {
            Some(crate::access::Identity::Person(email)) => app.portfolio_owners.contains(&email.to_lowercase()),
            Some(crate::access::Identity::ServiceToken(_)) => true,
            None => false,
        };
        if !allowed {
            return Err(ConnectError::new("permission_denied", "the portfolio is its owner's"));
        }
    }
    let db = app.client().await?;
    let synced = |db: Arc<tokio_postgres::Client>| async move {
        db.query_one("SELECT extract(epoch FROM max(synced_at))::bigint FROM portfolio_sync", &[]).await
            .map(|r| r.get::<_, Option<i64>>(0))
    };
    if req.refresh {
        let last = synced(db.clone()).await?.unwrap_or(0);
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;
        if now - last > SYNC_FRESH_SECONDS {
            let run = tokio::process::Command::new(&app.store_plaid).kill_on_drop(true).output();
            match tokio::time::timeout(std::time::Duration::from_secs(90), run).await {
                Ok(Ok(out)) if out.status.success() => {}
                Ok(Ok(out)) => eprintln!("store-plaid: {}", String::from_utf8_lossy(&out.stderr).trim()),
                Ok(Err(e)) => eprintln!("store-plaid: {}", e),
                Err(_) => eprintln!("store-plaid: over 90 s, stopped"),
            }
        }
    }

    // Short sales still on record, newest first, by (account, security).
    let mut sales: std::collections::HashMap<(String, String), Vec<(f64, f64)>> = Default::default();
    for r in db.query("SELECT account_id, security_id, -quantity, -amount FROM portfolio_transaction
                       WHERE name LIKE 'sell_short%' AND quantity < 0 ORDER BY date DESC, transaction_id", &[]).await? {
        sales.entry((r.get(0), r.get(1))).or_default().push((r.get(2), r.get(3)));
    }
    // Buys on record, newest first, by (account, security), and the
    // baseline's total return level by day (Tiingo's adjusted closes).
    let mut buys: std::collections::HashMap<(String, String), Vec<(NaiveDate, f64, f64)>> = Default::default();
    for r in db.query("SELECT account_id, security_id, date, quantity, amount FROM portfolio_transaction
                       WHERE name LIKE 'buy %' AND quantity > 0 AND amount > 0 ORDER BY date DESC, transaction_id", &[]).await? {
        buys.entry((r.get(0), r.get(1))).or_default().push((r.get(2), r.get(3), r.get(4)));
    }
    // Cash dividends, by (account, security): (day, amount as Plaid wrote it).
    // Plaid gives a dividend no security; Robinhood's description names its
    // ticker: "Cash dividend of $0.05 from CL - DIVIDEND".
    let mut paid: std::collections::HashMap<(String, String), Vec<(NaiveDate, f64)>> = Default::default();
    for r in db.query("SELECT t.account_id, s.security_id, t.date, t.amount
                       FROM portfolio_transaction t
                       JOIN portfolio_security s ON s.ticker = substring(t.name FROM 'from (\\S+) - DIVIDEND') AND s.type <> 'derivative'
                       WHERE t.type = 'cash' AND t.subtype = 'dividend' AND t.amount IS NOT NULL", &[]).await? {
        paid.entry((r.get(0), r.get(1))).or_default().push((r.get(2), r.get(3)));
    }
    let index = total_return_index(BASELINE);
    let market = total_return_index(MARKET);

    let rows = db.query(
        "SELECT a.account_id, coalesce(a.institution, ''), a.name, coalesce(a.subtype, ''), coalesce(a.balance, 0),
                s.security_id, coalesce(s.ticker, ''), coalesce(s.name, ''), coalesce(s.type, ''),
                h.quantity, coalesce(h.price, 0), coalesce(h.value, h.quantity * h.price, 0), h.cost_basis, h.price_as_of
         FROM portfolio_holding h JOIN portfolio_account a USING (account_id) JOIN portfolio_security s USING (security_id)
         WHERE h.as_of = (SELECT max(as_of) FROM portfolio_holding)
         ORDER BY a.balance DESC NULLS LAST, a.account_id, abs(coalesce(h.value, 0)) DESC", &[]).await?;
    let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
    let mut accounts: Vec<(String, pb::Account)> = Vec::new();
    for r in rows {
        let account_id: String = r.get(0);
        if accounts.last().is_none_or(|(id, _)| *id != account_id) {
            accounts.push((account_id.clone(), pb::Account {
                institution: r.get(1), name: r.get(2), subtype: r.get(3), balance: r.get(4), positions: vec![],
            }));
        }
        let (security_id, kind_name): (String, String) = (r.get(5), r.get(8));
        let quantity: f64 = r.get(9);
        let kind = match kind_name.as_str() {
            "derivative" => pb::PositionKind::Option,
            "cash" => pb::PositionKind::Cash,
            _ if quantity < 0.0 => pb::PositionKind::Short,
            _ => pb::PositionKind::Long,
        };
        let (cost, source) = match kind {
            pb::PositionKind::Long => match r.get::<_, Option<f64>>(12) {
                Some(c) => (Some(c), pb::CostSource::Institution),
                None => (None, pb::CostSource::Unspecified),
            },
            pb::PositionKind::Short => match sales.get(&(account_id.clone(), security_id.clone())).and_then(|s| short_proceeds(-quantity, s)) {
                Some(p) => (Some(-p), pb::CostSource::ShortSales),
                None => (None, pb::CostSource::Unspecified),
            },
            _ => (None, pb::CostSource::Unspecified),
        };
        let key = (account_id.clone(), security_id);
        let measured = match kind {
            pb::PositionKind::Long => buys.get(&key).and_then(|b| baseline(quantity, b, &index)),
            _ => None,
        };
        // The dividends that belong with the measured shares: those paid
        // since their oldest buy.
        let dividends = measured.as_ref().map(|m| paid.get(&key).map(|p| dividends_since(m.since, p)).unwrap_or(0.0));
        // The same dollars, on the same days, in the market instead.
        let in_market = match measured {
            Some(_) => buys.get(&key).and_then(|b| baseline(quantity, b, &market)).map(|m| m.value),
            None => None,
        };
        accounts.last_mut().unwrap().1.positions.push(pb::Position {
            ticker: r.get(6), name: r.get(7), kind: kind as i32, quantity, price: r.get(10), value: r.get(11),
            cost, cost_source: source as i32,
            price_epoch_day: r.get::<_, Option<NaiveDate>>(13).map(|d| (d - epoch).num_days() as i32).unwrap_or(0),
            baseline_cost: measured.as_ref().map(|m| m.cost),
            baseline_value: measured.as_ref().map(|m| m.value),
            dividends,
            market_value: in_market,
        });
    }
    let synced_unix = synced(db).await?.unwrap_or(0);
    let last_day = |i: &[(NaiveDate, f64)]| i.last().map(|(d, _)| (*d - epoch).num_days() as i32).unwrap_or(0);
    Ok(codec.encode(&pb::HoldingsResponse {
        accounts: accounts.into_iter().map(|(_, a)| a).collect(),
        synced_unix,
        baseline: BASELINE.into(),
        baseline_epoch_day: last_day(&index),
        market: MARKET.into(),
        market_epoch_day: last_day(&market),
    }))
}


/// `TotalAssets` → `TOTAL_ASSETS`, the .proto's spelling.
pub fn screaming(camel: &str) -> String {
    let mut s = String::new();
    for (i, c) in camel.chars().enumerate() {
        if c.is_ascii_uppercase() && i > 0 { s.push('_') }
        s.push(c.to_ascii_uppercase());
    }
    s
}

/// The store's names for periods (listing_facts) and market fields (the
/// `market` table's columns).
const PERIODS: [(pb::Period, &str); 5] = [
    (pb::Period::Quarter, "3m"),
    (pb::Period::HalfYear, "6m"),
    (pb::Period::NineMonths, "9m"),
    (pb::Period::Year, "12m"),
    (pb::Period::PointInTime, "pit"),
];
const FIELDS: [(pb::MarketField, &str); 5] = [
    (pb::MarketField::Close, "close"),
    (pb::MarketField::MarketCap, "market_cap"),
    (pb::MarketField::PriceToBook, "price_to_book"),
    (pb::MarketField::ReturnOnEquity, "return_on_equity"),
    (pb::MarketField::EarningsYield, "earnings_yield"),
];

fn period_name(p: pb::Period) -> Option<&'static str> { PERIODS.iter().find(|x| x.0 == p).map(|x| x.1) }
fn period_of(name: &str) -> Option<pb::Period> { PERIODS.iter().find(|x| x.1 == name).map(|x| x.0) }
fn field_name(f: pb::MarketField) -> Option<&'static str> { FIELDS.iter().find(|x| x.0 == f).map(|x| x.1) }
fn field_of(name: &str) -> Option<pb::MarketField> { FIELDS.iter().find(|x| x.1 == name).map(|x| x.0) }

#[cfg(test)]
mod tests {
    use super::*;

    /// The .proto's Item and equities::Item name the same items, and each
    /// converts to the other by name.
    #[test]
    fn items_agree() {
        let mut missing = Vec::new();
        for item in equities::item::Item::ALL {
            let name = format!("{:?}", item);
            match pb::Item::from_str_name(&format!("ITEM_{}", screaming(&name))) {
                Some(p) => assert_eq!(format!("{:?}", p), name, "the .proto's {} doesn't convert back", p.as_str_name()),
                None => missing.push(format!("ITEM_{}", screaming(&name))),
            }
        }
        assert!(missing.is_empty(), "add to the .proto's Item, numbered after the last: {:?}", missing);
        // And none the other way: the .proto has one more, UNSPECIFIED.
        let in_proto = (1..1000).filter(|&i| pb::Item::try_from(i).is_ok()).count();
        assert_eq!(in_proto, equities::item::Item::ALL.len(), "the .proto has items equities::Item hasn't");
    }

    /// The bytes both sides must agree on: the same hex is in optimates'
    /// Tests/OptimatesTests (ContractTests), decoded and encoded by the
    /// Swift the app compiles from this .proto. Changing either means a
    /// change to the wire format: one the app on the phone won't read.
    // ticker "gs", items [TotalAssets], periods [Quarter], market [Close, MarketCap]
    pub const GOLDEN_REQUEST: &str = "0a026773 120116 1a0101 22020102";
    // TotalAssets at a point in time in USD, 2026-03-31: 1.5e12; market cap in
    // JPY, 1970-01-01: -2 and 1970-01-02: 0.25
    pub const GOLDEN_RESPONSE: &str = "0a1a0a04081610051a03555344220d08bfa0011100008079efd37542 0a1f10021a034a505922091100000000000000c0220b080111000000000000d03f";

    fn golden() -> (pb::DashboardRequest, pb::DashboardResponse) {
        let request = pb::DashboardRequest {
            ticker: "gs".into(),
            items: vec![pb::Item::TotalAssets as i32],
            periods: vec![pb::Period::Quarter as i32],
            market: vec![pb::MarketField::Close as i32, pb::MarketField::MarketCap as i32],
        };
        let response = pb::DashboardResponse {
            series: vec![
                pb::Series {
                    of: Some(pb::series::Of::Reported(pb::Reported { item: pb::Item::TotalAssets as i32, period: pb::Period::PointInTime as i32 })),
                    currency: "USD".into(),
                    points: vec![pb::Point { epoch_day: 20543, value: 1.5e12 }],
                },
                pb::Series {
                    of: Some(pb::series::Of::Market(pb::MarketField::MarketCap as i32)),
                    currency: "JPY".into(),
                    points: vec![pb::Point { epoch_day: 0, value: -2.0 }, pb::Point { epoch_day: 1, value: 0.25 }],
                },
            ],
        };
        (request, response)
    }

    fn hex(bytes: &[u8]) -> String { bytes.iter().map(|b| format!("{:02x}", b)).collect() }

    #[test]
    fn golden_bytes() {
        let (request, response) = golden();
        let squash = |s: &str| s.replace(' ', "");
        assert_eq!(hex(&request.encode_to_vec()), squash(GOLDEN_REQUEST), "request");
        assert_eq!(hex(&response.encode_to_vec()), squash(GOLDEN_RESPONSE), "response");
    }

    #[test]
    fn shorts_cost_what_their_newest_sales_brought_in() {
        // Shorted 1 share at 60, covered, shorted 1 at 50; short 1 now.
        let sales = [(1.0, 50.0), (1.0, 60.0)];
        assert_eq!(short_proceeds(1.0, &sales), Some(50.0));
        assert_eq!(short_proceeds(2.0, &sales), Some(110.0));
        assert_eq!(short_proceeds(1.5, &sales), Some(80.0));
        // More short than the sales on record: not known.
        assert_eq!(short_proceeds(3.0, &sales), None);
        assert_eq!(short_proceeds(1.0, &[]), None);
    }

    #[test]
    fn the_same_dollars_in_the_baseline() {
        let d = |y, m, day| NaiveDate::from_ymd_opt(y, m, day).unwrap();
        // The baseline rose 2% from January to June, then 1% to now.
        let index = [(d(2026, 1, 2), 100.0), (d(2026, 6, 1), 102.0), (d(2026, 11, 25), 103.02)];
        // Held 3 shares: bought 2 in June ($200), 2 in January ($100), 1 sold since.
        let buys = [(d(2026, 6, 1), 2.0, 200.0), (d(2026, 1, 5), 2.0, 100.0)];
        let m = baseline(3.0, &buys, &index).unwrap();
        // June's $200 grew 1%; half of January's ($50), bought on a weekend
        // after the 2nd's level, grew 3.02%. The oldest held buy: January's.
        assert!((m.cost - 250.0).abs() < 1e-9);
        assert!((m.value - (200.0 * 1.01 + 50.0 * 1.0302)).abs() < 1e-9);
        assert_eq!(m.since, d(2026, 1, 5));
        // More held than bought on record: not known.
        assert_eq!(baseline(5.0, &buys, &index), None);
        // A buy before the index starts: not known.
        assert_eq!(baseline(1.0, &[(d(2025, 12, 1), 1.0, 10.0)], &index), None);
    }

    #[test]
    fn dividends_since_the_oldest_held_buy() {
        let d = |y, m, day| NaiveDate::from_ymd_opt(y, m, day).unwrap();
        // Plaid writes cash in as negative: $1.57 and $1.60 received, the
        // first before the shares held were bought; $0.35 paid out (a short).
        let paid = [(d(2025, 12, 1), -1.57), (d(2026, 3, 1), -1.60), (d(2026, 6, 1), 0.35)];
        assert!((dividends_since(d(2026, 1, 5), &paid) - (1.60 - 0.35)).abs() < 1e-9);
        assert_eq!(dividends_since(d(2027, 1, 1), &paid), 0.0);
    }

    #[test]
    fn periods_and_fields_cover_the_store() {
        assert_eq!(PERIODS.map(|x| x.1), crate::PERIODS);
        assert_eq!(FIELDS.map(|x| x.1), crate::MARKET_FIELDS);
    }
}
