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

    #[test]
    fn periods_and_fields_cover_the_store() {
        assert_eq!(PERIODS.map(|x| x.1), crate::PERIODS);
        assert_eq!(FIELDS.map(|x| x.1), crate::MARKET_FIELDS);
    }
}
