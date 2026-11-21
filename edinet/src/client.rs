//! Downloading into the cache, through EDINET's API (v2).
//!
//! Day lists before the API's window (about ten years back) come back as
//! 404s and are saved empty, so they are not asked for again. Errors arrive
//! as JSON with an HTTP 200, so every response's own status is checked.

use std::cell::Cell;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::thread::sleep;
use std::time::{Duration, Instant};

use chrono::NaiveDate;
use serde_json::Value;

use crate::{Cache, Document, EdinetError};

const API: &str = "https://api.edinet-fsa.go.jp/api/v2";

/// EDINET publishes no rate limit; two requests a second is polite.
const MIN_INTERVAL: Duration = Duration::from_millis(500);

/// Day lists this recent are fetched again: filings are corrected and
/// withdrawn for a while after they appear.
const RECENT_DAYS: i64 = 7;

pub struct Client {
    http: reqwest::blocking::Client,
    key: String,
    cache: Cache,
    last_request: Cell<Option<Instant>>,
    requests: Cell<usize>,
}

impl Client {
    /// `key` is an EDINET API key (from the API key page,
    /// https://api.edinet-fsa.go.jp/api/auth/index.aspx?mode=1).
    pub fn new(key: impl Into<String>, cache: Cache) -> Result<Client, Box<dyn Error>> {
        Ok(Client {
            http: reqwest::blocking::Client::builder().timeout(Duration::from_secs(300)).build()?,
            key: key.into(),
            cache,
            last_request: Cell::new(None),
            requests: Cell::new(0),
        })
    }

    pub fn cache(&self) -> &Cache {
        &self.cache
    }

    /// How many requests this client has sent.
    pub fn requests(&self) -> usize {
        self.requests.get()
    }

    /// GET with a pause between requests, retrying server errors.
    fn get(&self, url: &str, query: &[(&str, &str)]) -> Result<reqwest::blocking::Response, Box<dyn Error>> {
        let mut attempt = 0;
        loop {
            if let Some(last) = self.last_request.get() {
                sleep(MIN_INTERVAL.saturating_sub(last.elapsed()));
            }
            self.last_request.set(Some(Instant::now()));
            self.requests.set(self.requests.get() + 1);
            // Dates, codes and a hex key: nothing that needs escaping.
            let mut full = format!("{}?Subscription-Key={}", url, self.key);
            for (k, v) in query {
                full.push_str(&format!("&{}={}", k, v));
            }
            let res = self.http.get(&full).send();
            let retry = match &res {
                Ok(r) => r.status().is_server_error(),
                Err(e) => e.is_timeout() || e.is_connect(),
            };
            attempt += 1;
            if !retry || attempt == 4 {
                // reqwest's errors quote the URL, which holds the API key.
                return Ok(res.and_then(|r| r.error_for_status()).map_err(|e| e.without_url())?);
            }
            sleep(Duration::from_secs(1 << attempt));
        }
    }

    /// Fetch the day lists from `from` to `to` not already on disk (and the
    /// last few days again). Returns how many were fetched.
    pub fn fetch_days(&self, from: NaiveDate, to: NaiveDate) -> Result<usize, Box<dyn Error>> {
        let recent = to - chrono::Duration::days(RECENT_DAYS);
        let mut fetched = 0;
        for date in from.iter_days().take_while(|d| *d <= to) {
            let path = self.cache.day_path(date);
            if path.exists() && date < recent {
                continue;
            }
            let bytes = self.get(&format!("{}/documents.json", API), &[("date", &date.to_string()), ("type", "2")])?.bytes().map_err(|e| e.without_url())?;
            let v: Value = serde_json::from_slice(&bytes)?;
            // {"metadata": {"status": "200", ...}, "results": [...]}, or
            // {"StatusCode": 401, "message": ...} for a bad key.
            let status = v.pointer("/metadata/status").and_then(Value::as_str).map(str::to_string)
                .or_else(|| v.get("StatusCode").map(|s| s.to_string()))
                .unwrap_or_default();
            let message = v.pointer("/metadata/message").or_else(|| v.get("message"))
                .and_then(Value::as_str).unwrap_or("").to_string();
            match status.as_str() {
                "200" => write_atomic(&path, &bytes)?,
                // Before the API's window: nothing to list.
                "404" => write_atomic(&path, br#"{"results": []}"#)?,
                _ => return Err(EdinetError::Api(status, message).into()),
            }
            fetched += 1;
        }
        Ok(fetched)
    }

    /// Download a document's facts (its XBRL as CSV) unless already on disk.
    /// Returns whether it was downloaded.
    pub fn fetch_document(&self, d: &Document) -> Result<bool, Box<dyn Error>> {
        let dir = self.cache.document_dir(d);
        if dir.join("XBRL_TO_CSV").exists() {
            return Ok(false);
        }
        let bytes = self.get(&format!("{}/documents/{}", API, d.doc_id), &[("type", "5")])?.bytes().map_err(|e| e.without_url())?;
        // A refusal is JSON, not a zip.
        if bytes.starts_with(b"{") {
            let v: Value = serde_json::from_slice(&bytes)?;
            let status = v.pointer("/metadata/status").map(|s| s.to_string()).unwrap_or_default();
            let message = v.pointer("/metadata/message").and_then(Value::as_str).unwrap_or("").to_string();
            return Err(EdinetError::Api(status, message).into());
        }
        // Unpack beside the final directory, then rename, so a half-written
        // document is never taken for a whole one.
        let tmp = dir.with_extension("partial");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp)?;
        zip::ZipArchive::new(std::io::Cursor::new(bytes))?.extract(&tmp)?;
        fs::rename(&tmp, &dir)?;
        Ok(true)
    }
}

fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    if let Some(dir) = path.parent() { fs::create_dir_all(dir)?; }
    let tmp = path.with_extension("part");
    fs::write(&tmp, bytes)?;
    fs::rename(&tmp, path)?;
    Ok(())
}
