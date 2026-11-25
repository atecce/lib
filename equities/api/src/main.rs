//! The equities API, behind the tunnel and Cloudflare Access as
//! `api.atec.pub`: what the Optimates app reads, from Postgres.
//!
//!     ACCESS_TEAM=steep-snow-0aca ACCESS_AUD=<the Access app's AUD tag> equities-api
//!     equities-api --no-access        # local development: nothing checked
//!
//! Listens on 127.0.0.1:8787 (`--listen` for another), for cloudflared to
//! forward to. Every request but `/health` needs a valid Cloudflare Access
//! assertion (access.rs); the database credentials stay here (`STORE_URL`,
//! default `host=localhost dbname=financials`).
//!
//! - `GET /health`: `ok`.
//! - `GET /tickers`: the listings whose company filed in the last 400 days.
//! - `GET /dashboard?ticker=AAPL&items=Revenue,NetIncome&periods=3m,pit&market=market_cap`:
//!   each reported item for each period, as last reported (`listing_facts`),
//!   and each market field, in every currency the company reports in.

mod access;
mod rpc;

use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::{Query, Request, State};
use axum::http::{HeaderMap, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tokio_postgres::{Client, NoTls};

use access::Access;

/// Reported periods `listing_facts` knows.
const PERIODS: [&str; 5] = ["3m", "6m", "9m", "12m", "pit"];
/// The `market` table's series.
const MARKET_FIELDS: [&str; 5] = ["close", "market_cap", "price_to_book", "return_on_equity", "earnings_yield"];

struct App {
    store_url: String,
    /// One connection, reopened when Postgres restarts. (Queries pipeline
    /// over it; the app's few requests need no pool.)
    db: Mutex<Option<Arc<Client>>>,
    access: Option<Arc<Access>>,
}

impl App {
    async fn client(&self) -> Result<Arc<Client>, ApiError> {
        let mut db = self.db.lock().await;
        if let Some(c) = db.as_ref().filter(|c| !c.is_closed()) {
            return Ok(c.clone());
        }
        let (client, connection) = tokio_postgres::connect(&self.store_url, NoTls).await
            .map_err(|e| ApiError::internal(format!("Postgres: {}", e)))?;
        tokio::spawn(async move {
            if let Err(e) = connection.await { eprintln!("Postgres connection: {}", e) }
        });
        let client = Arc::new(client);
        *db = Some(client.clone());
        Ok(client)
    }
}

/// An error as the app sees it: a status and a message.
struct ApiError(StatusCode, String);

impl ApiError {
    fn bad(m: impl Into<String>) -> ApiError { ApiError(StatusCode::BAD_REQUEST, m.into()) }
    fn internal(m: impl Into<String>) -> ApiError { ApiError(StatusCode::INTERNAL_SERVER_ERROR, m.into()) }
}

impl From<tokio_postgres::Error> for ApiError {
    fn from(e: tokio_postgres::Error) -> ApiError { ApiError::internal(e.to_string()) }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        if self.0.is_server_error() { eprintln!("{}: {}", self.0, self.1) }
        (self.0, Json(serde_json::json!({ "error": self.1 }))).into_response()
    }
}

/// A refusal both kinds of client read: a Connect error (`code`, `message`)
/// and the JSON routes' `error`.
fn refuse(status: StatusCode, code: &str, message: &str) -> Response {
    (status, Json(serde_json::json!({ "code": code, "message": message, "error": message }))).into_response()
}

/// Refuse what Cloudflare Access didn't let through.
async fn require_access(State(app): State<Arc<App>>, headers: HeaderMap, req: Request, next: Next) -> Response {
    let Some(access) = &app.access else { return next.run(req).await };
    let Some(assertion) = headers.get("cf-access-jwt-assertion").and_then(|v| v.to_str().ok()) else {
        return refuse(StatusCode::UNAUTHORIZED, "unauthenticated", "no Cloudflare Access assertion");
    };
    match access.verify(assertion).await {
        Ok(who) => {
            eprintln!("{} {} ({})", req.method(), req.uri().path(), who);
            next.run(req).await
        }
        Err(why) => {
            eprintln!("refused {} {}: {}", req.method(), req.uri(), why);
            refuse(StatusCode::FORBIDDEN, "permission_denied", "Cloudflare Access assertion not valid")
        }
    }
}

/// The listings whose company filed in the last 400 days.
async fn load_tickers(db: &Client) -> Result<Vec<String>, tokio_postgres::Error> {
    let rows = db.query(
        "SELECT l.ticker FROM listing l WHERE EXISTS (
             SELECT 1 FROM filing g WHERE g.issuer_id = l.issuer_id AND g.filed_at > now() - interval '400 days')
         ORDER BY l.ticker", &[]).await?;
    Ok(rows.iter().map(|r| r.get(0)).collect())
}

async fn tickers(State(app): State<Arc<App>>) -> Result<Json<Vec<String>>, ApiError> {
    Ok(Json(load_tickers(&*app.client().await?).await?))
}

#[derive(Deserialize)]
struct DashboardQuery {
    ticker: String,
    #[serde(default)]
    items: String,
    #[serde(default)]
    periods: String,
    #[serde(default)]
    market: String,
}

#[derive(Serialize)]
struct Point {
    date: NaiveDate,
    value: f64,
}

/// A series of a dashboard: a reported `item` for a `period`, or a market
/// field with period `market`, in a `currency`.
#[derive(Serialize)]
struct TimeSeries {
    item: String,
    period: String,
    currency: String,
    points: Vec<Point>,
}

fn list(s: &str) -> Vec<String> {
    s.split(',').map(str::trim).filter(|x| !x.is_empty()).map(str::to_string).collect()
}

/// A company's dashboard: each series by (item or market field, period or
/// `market`, currency), its points in date order. `periods` and `fields`
/// must be among PERIODS and MARKET_FIELDS.
type Dashboard = BTreeMap<(String, String, String), Vec<(NaiveDate, f64)>>;

async fn load_dashboard(db: &Client, ticker: &str, items: &[String], periods: &[String], fields: &[String])
    -> Result<Dashboard, ApiError>
{
    if let Some(p) = periods.iter().find(|p| !PERIODS.contains(&p.as_str())) {
        return Err(ApiError::bad(format!("not a period: {}", p)));
    }
    if let Some(f) = fields.iter().find(|f| !MARKET_FIELDS.contains(&f.as_str())) {
        return Err(ApiError::bad(format!("not a market field: {}", f)));
    }
    let mut series = Dashboard::new();

    if !items.is_empty() && !periods.is_empty() {
        // Every currency the listing reports in (from its own regulator), each
        // period: listing_facts, as the Grafana dashboards draw them.
        let rows = db.query(
            // (Driven from the company's own facts: joining filings first,
            // the planner reads all of them, about a second.)
            "WITH c AS (
                 SELECT DISTINCT f.currency FROM fact f
                 WHERE f.issuer_id = (SELECT issuer_id FROM listing WHERE ticker = upper($1))
                   AND EXISTS (SELECT 1 FROM filing g WHERE g.accession = f.accession
                               AND g.source = listing_source(upper($1))))
             SELECT lf.item, p.period, c.currency::text, lf.period_end, lf.value
             FROM c CROSS JOIN unnest($3::text[]) AS p(period)
             CROSS JOIN LATERAL listing_facts($1, p.period, c.currency::text, $2::text[]) lf
             ORDER BY lf.period_end",
            &[&ticker, &items, &periods]).await?;
        for r in rows {
            series.entry((r.get(0), r.get(1), r.get(2))).or_default().push((r.get(3), r.get(4)));
        }
    }

    if !fields.is_empty() {
        let rows = db.query(
            "SELECT date, currency::text, close, market_cap, price_to_book, return_on_equity, earnings_yield
             FROM market WHERE ticker = upper($1) ORDER BY date", &[&ticker]).await?;
        for r in rows {
            let (date, currency): (NaiveDate, String) = (r.get(0), r.get(1));
            for f in fields {
                let column = 2 + MARKET_FIELDS.iter().position(|m| m == f).unwrap();
                if let Some(value) = r.get::<_, Option<f64>>(column) {
                    series.entry((f.clone(), "market".into(), currency.clone())).or_default().push((date, value));
                }
            }
        }
    }
    Ok(series)
}

async fn dashboard(State(app): State<Arc<App>>, Query(q): Query<DashboardQuery>) -> Result<Json<Vec<TimeSeries>>, ApiError> {
    let (items, periods, fields) = (list(&q.items), list(&q.periods), list(&q.market));
    let series = load_dashboard(&*app.client().await?, &q.ticker, &items, &periods, &fields).await?;
    Ok(Json(series.into_iter()
        .map(|((item, period, currency), points)| TimeSeries {
            item, period, currency,
            points: points.into_iter().map(|(date, value)| Point { date, value }).collect(),
        })
        .collect()))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let no_access = args.iter().any(|a| a == "--no-access");
    let listen: SocketAddr = match args.iter().position(|a| a == "--listen") {
        Some(i) => args.get(i + 1).ok_or("--listen needs an address")?.parse()?,
        None => "127.0.0.1:8787".parse()?,
    };
    let env = |k: &str| std::env::var(k).ok().filter(|v| !v.is_empty());
    let access = if no_access {
        eprintln!("--no-access: requests are not checked; for local development only");
        None
    } else {
        let team = env("ACCESS_TEAM").ok_or("ACCESS_TEAM is not set (or pass --no-access)")?;
        let aud = env("ACCESS_AUD").ok_or("ACCESS_AUD is not set (or pass --no-access)")?;
        Some(Access::new(team, aud))
    };
    let app = Arc::new(App {
        store_url: env("STORE_URL").unwrap_or_else(|| "host=localhost dbname=financials".into()),
        db: Mutex::new(None),
        access,
    });

    let checked = Router::new()
        .route("/tickers", get(tickers))
        .route("/dashboard", get(dashboard))
        .merge(rpc::routes())
        .layer(middleware::from_fn_with_state(app.clone(), require_access));
    let router = Router::new()
        .route("/health", get(|| async { "ok" }))
        .merge(checked)
        .with_state(app);

    let listener = tokio::net::TcpListener::bind(listen).await?;
    eprintln!("equities-api on http://{}", listen);
    axum::serve(listener, router)
        .with_graceful_shutdown(async { let _ = tokio::signal::ctrl_c().await; })
        .await?;
    Ok(())
}
