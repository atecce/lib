//! Plaid (plaid.com) as a source: a brokerage's holdings and investment
//! transactions, read-only, for lib's portfolio.
//!
//! Plaid connects to an institution through the institution's own login (for
//! Robinhood, OAuth); what it gives back is an access token for that one
//! connection, an "Item". The keys (`~/.plaid/env`) and the Items' tokens
//! (`~/.plaid/items/`) stay on this Mac, readable by its user only, and out
//! of the repository; their copies are in Keybase (etc/plaid).

use std::error::Error;
use std::fs;
use std::io::Write;
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::PathBuf;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// `~/.plaid`.
pub fn dir() -> PathBuf {
    PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".into())).join(".plaid")
}

/// Plaid's error: its code and message, as the API returns them.
#[derive(Debug)]
pub struct PlaidError {
    pub code: String,
    pub message: String,
}

impl std::fmt::Display for PlaidError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Plaid {}: {}", self.code, self.message)
    }
}

impl Error for PlaidError {}

pub struct Client {
    http: reqwest::blocking::Client,
    base: &'static str,
    client_id: String,
    secret: String,
}

impl Client {
    /// From `~/.plaid/env`: PLAID_CLIENT_ID, PLAID_ENV (sandbox or
    /// production) and that environment's secret (PLAID_SANDBOX_SECRET or
    /// PLAID_SECRET).
    pub fn from_env_file() -> Result<Client, Box<dyn Error>> {
        let path = dir().join("env");
        let text = fs::read_to_string(&path).map_err(|e| format!("{}: {}", path.display(), e))?;
        let get = |k: &str| text.lines()
            .filter_map(|l| l.split_once('='))
            .find(|(key, _)| key.trim() == k)
            .map(|(_, v)| v.trim().to_string())
            .filter(|v| !v.is_empty());
        let client_id = get("PLAID_CLIENT_ID").ok_or("PLAID_CLIENT_ID is not set in ~/.plaid/env")?;
        let (base, secret) = match get("PLAID_ENV").as_deref().unwrap_or("sandbox") {
            "production" => ("https://production.plaid.com", get("PLAID_SECRET").ok_or("PLAID_SECRET is not set in ~/.plaid/env")?),
            "sandbox" => ("https://sandbox.plaid.com", get("PLAID_SANDBOX_SECRET").ok_or("PLAID_SANDBOX_SECRET is not set in ~/.plaid/env")?),
            other => return Err(format!("PLAID_ENV {:?}: sandbox or production", other).into()),
        };
        let http = reqwest::blocking::Client::builder().timeout(Duration::from_secs(60)).build()?;
        Ok(Client { http, base, client_id, secret })
    }

    pub fn is_production(&self) -> bool { self.base.contains("production") }

    /// POST `body` (with the keys added) to `path`.
    pub fn post(&self, path: &str, mut body: Value) -> Result<Value, Box<dyn Error>> {
        body["client_id"] = json!(self.client_id);
        body["secret"] = json!(self.secret);
        let response = self.http.post(format!("{}{}", self.base, path)).json(&body).send()?;
        let ok = response.status().is_success();
        let value: Value = response.json()?;
        if !ok || value.get("error_code").is_some_and(|c| !c.is_null()) {
            return Err(Box::new(PlaidError {
                code: value["error_code"].as_str().unwrap_or("UNKNOWN").into(),
                message: value["display_message"].as_str().or(value["error_message"].as_str()).unwrap_or("").into(),
            }));
        }
        Ok(value)
    }

    /// A token for Plaid Link (its login widget), for Investments.
    pub fn link_token(&self) -> Result<String, Box<dyn Error>> {
        let v = self.post("/link/token/create", json!({
            "client_name": "lib",
            "user": { "client_user_id": "lib-portfolio" },
            "products": ["investments"],
            "country_codes": ["US"],
            "language": "en",
        }))?;
        Ok(v["link_token"].as_str().ok_or("no link_token")?.to_string())
    }

    /// Link's public token for an Item's access token.
    pub fn exchange(&self, public_token: &str) -> Result<(String, String), Box<dyn Error>> {
        let v = self.post("/item/public_token/exchange", json!({ "public_token": public_token }))?;
        Ok((v["access_token"].as_str().ok_or("no access_token")?.into(), v["item_id"].as_str().ok_or("no item_id")?.into()))
    }

    /// The Item's accounts, holdings and securities (`/investments/holdings/get`).
    pub fn holdings(&self, item: &Item) -> Result<Value, Box<dyn Error>> {
        self.post("/investments/holdings/get", json!({ "access_token": item.access_token }))
    }

    /// The Item's investment transactions from `start` to `end` (dates,
    /// YYYY-MM-DD; Plaid keeps 24 months), every page, and the securities
    /// they name.
    pub fn investment_transactions(&self, item: &Item, start: &str, end: &str) -> Result<(Vec<Value>, Vec<Value>), Box<dyn Error>> {
        let (mut transactions, mut securities) = (Vec::new(), Vec::new());
        loop {
            let v = self.post("/investments/transactions/get", json!({
                "access_token": item.access_token,
                "start_date": start,
                "end_date": end,
                "options": { "count": 500, "offset": transactions.len() },
            }))?;
            let page = v["investment_transactions"].as_array().cloned().unwrap_or_default();
            securities.extend(v["securities"].as_array().cloned().unwrap_or_default());
            let total = v["total_investment_transactions"].as_u64().unwrap_or(0) as usize;
            let empty = page.is_empty();
            transactions.extend(page);
            if empty || transactions.len() >= total { break }
        }
        Ok((transactions, securities))
    }
}

/// A connection to an institution: its access token is a credential.
#[derive(Serialize, Deserialize)]
pub struct Item {
    pub item_id: String,
    pub access_token: String,
    pub institution: Option<String>,
}

impl Item {
    fn items_dir() -> PathBuf { dir().join("items") }

    /// Every Item connected so far.
    pub fn all() -> Result<Vec<Item>, Box<dyn Error>> {
        let mut items = Vec::new();
        let Ok(entries) = fs::read_dir(Self::items_dir()) else { return Ok(items) };
        for entry in entries {
            let path = entry?.path();
            if path.extension().is_some_and(|e| e == "json") {
                items.push(serde_json::from_slice(&fs::read(&path)?)?);
            }
        }
        Ok(items)
    }

    /// Kept as `~/.plaid/items/<item_id>.json`, mode 0600.
    pub fn save(&self) -> Result<PathBuf, Box<dyn Error>> {
        let d = Self::items_dir();
        fs::create_dir_all(&d)?;
        fs::set_permissions(&d, fs::Permissions::from_mode(0o700))?;
        let path = d.join(format!("{}.json", self.item_id));
        let mut f = fs::OpenOptions::new().write(true).create(true).truncate(true).mode(0o600).open(&path)?;
        f.write_all(&serde_json::to_vec_pretty(self)?)?;
        Ok(path)
    }
}
