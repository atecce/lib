//! Bring FRED series in the cache up to date.
//!
//!     fred-fetch [--force] [--list FILE ...] [SERIES ...]   # e.g. fred-fetch GS10
//!
//! `--list` reads series IDs from a file, separated by spaces, commas or
//! newlines, with `#` starting a comment (`rates.txt`).
//! A series fetched in the last 20 hours is left alone unless `--force`.

use std::error::Error;

use fred::{Cache, Client};

fn main() -> Result<(), Box<dyn Error>> {
    let mut force = false;
    let mut ids = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--force" => force = true,
            "--list" => {
                let file = args.next().ok_or("--list needs a file")?;
                for line in std::fs::read_to_string(&file)?.lines() {
                    let line = line.split('#').next().unwrap_or("");
                    ids.extend(line.split(|c: char| c == ',' || c.is_whitespace()).filter(|s| !s.is_empty()).map(str::to_uppercase));
                }
            },
            a if a.starts_with("--") => return Err(format!("unknown option {}", a).into()),
            id => ids.push(id.to_uppercase()),
        }
    }
    if ids.is_empty() {
        return Err("usage: fred-fetch [--force] [--list FILE ...] [SERIES ...]".into());
    }
    let client = Client::new(Cache::new(Cache::default_dir()))?;
    let mut failed = 0;
    for id in ids {
        let fetched = match client.fetch(&id, force) {
            Ok(f) => f,
            Err(e) => { eprintln!("{}: {}", id, e); failed += 1; continue },
        };
        let series = client.cache().series(&id)?;
        let latest = series.latest().map(|o| format!("{} = {}", o.date, o.value.unwrap())).unwrap_or_default();
        eprintln!("{}: {} observations, latest {}{}", id, series.observations.len(), latest,
            if fetched { "" } else { " (cached)" });
    }
    if failed > 0 {
        return Err(format!("{} series failed", failed).into());
    }
    Ok(())
}
