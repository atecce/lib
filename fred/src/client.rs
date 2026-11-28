//! Downloading into the cache.
//!
//! A series is one request for its whole history (`fredgraph.csv`, which
//! needs no API key). Series are revised and extended, so a copy older than
//! `MAX_AGE` is fetched again; the file is replaced only once the new one
//! has parsed.

use std::error::Error;
use std::fs;
use std::time::{Duration, SystemTime};

use crate::{Cache, Series};

/// Monthly and daily series gain an observation at most daily.
const MAX_AGE: Duration = Duration::from_secs(20 * 3600);

const URL: &str = "https://fred.stlouisfed.org/graph/fredgraph.csv";

/// Generic, naming no one. FRED's edge resets HTTP/2 streams for some
/// User-Agents (`lib-fred`, a browser's) and not others; this one passes.
pub const USER_AGENT: &str = "lib-fetch";

pub struct Client {
    http: reqwest::blocking::Client,
    cache: Cache,
}

impl Client {
    pub fn new(cache: Cache) -> Result<Client, Box<dyn Error>> {
        Ok(Client {
            http: reqwest::blocking::Client::builder().user_agent(USER_AGENT).build()?,
            cache,
        })
    }

    pub fn cache(&self) -> &Cache {
        &self.cache
    }

    /// Fetch `id` unless the cached copy is recent (or `force`). Returns
    /// whether it was fetched.
    pub fn fetch(&self, id: &str, force: bool) -> Result<bool, Box<dyn Error>> {
        let path = self.cache.path(id);
        let fresh = fs::metadata(&path).and_then(|m| m.modified()).ok()
            .and_then(|t| SystemTime::now().duration_since(t).ok())
            .is_some_and(|age| age < MAX_AGE);
        if fresh && !force {
            return Ok(false);
        }
        let csv = self.http.get(format!("{}?id={}", URL, id)).send()?.error_for_status()?.text()?;
        let series = Series::parse(id, &csv)?;
        if series.observations.is_empty() {
            return Err(format!("{}: no observations (not a FRED series?)", id).into());
        }
        fs::create_dir_all(self.cache.dir())?;
        let tmp = path.with_extension("csv.part");
        fs::write(&tmp, csv)?;
        fs::rename(tmp, path)?;
        Ok(true)
    }
}
