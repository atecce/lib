//! Bring FRED series in the cache up to date.
//!
//!     fred-fetch [--force] SERIES ...       # e.g. fred-fetch GS10
//!
//! A series fetched in the last 20 hours is left alone unless `--force`.

use std::error::Error;

use fred::{Cache, Client};

fn main() -> Result<(), Box<dyn Error>> {
    let mut force = false;
    let mut ids = Vec::new();
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--force" => force = true,
            a if a.starts_with("--") => return Err(format!("unknown option {}", a).into()),
            id => ids.push(id.to_uppercase()),
        }
    }
    if ids.is_empty() {
        return Err("usage: fred-fetch [--force] SERIES ...".into());
    }
    let client = Client::new(Cache::new(Cache::default_dir()))?;
    for id in ids {
        let fetched = client.fetch(&id, force)?;
        let series = client.cache().series(&id)?;
        let latest = series.latest().map(|o| format!("{} = {}", o.date, o.value.unwrap())).unwrap_or_default();
        eprintln!("{}: {} observations, latest {}{}", id, series.observations.len(), latest,
            if fetched { "" } else { " (cached)" });
    }
    Ok(())
}
