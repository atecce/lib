//! Bring the Tiingo price cache up to date for some tickers.
//!
//!     tiingo-fetch [--list FILE ...] [TICKER ...]
//!
//! Needs a key in `TIINGO_API_KEY`. Paces itself to the free plan (50
//! requests an hour), so a long list takes hours: run it from a scheduler.
//! Exits non-zero if any ticker failed.

use std::error::Error;

use tiingo::{Cache, Client};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    let mut tickers = Vec::new();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--list" => {
                let path = args.next().ok_or("--list needs a file")?;
                for line in std::fs::read_to_string(&path)?.lines() {
                    let line = line.split('#').next().unwrap_or("");
                    tickers.extend(line.split([' ', '\t', ',']).filter(|t| !t.is_empty()).map(str::to_string));
                }
            },
            a if a.starts_with("--") => return Err(format!("unknown option {}", a).into()),
            t => tickers.push(t.to_string()),
        }
    }
    // Tokyo tickers (7203.T) have no Tiingo prices.
    tickers.retain(|t| !t.to_uppercase().ends_with(".T"));
    tickers.dedup();

    let key = std::env::var("TIINGO_API_KEY").map_err(|_| "TIINGO_API_KEY is not set")?;
    let client = Client::new(key, Cache::new(Cache::default_dir()))?;
    let mut failed = Vec::new();
    for t in &tickers {
        match client.fetch(t) {
            Ok(n) => eprintln!("{}: {} new days", t, n),
            Err(e) => {
                eprintln!("{}: {}", t, e);
                failed.push(t.as_str());
            },
        }
    }
    eprintln!("{} tickers, {} requests{}", tickers.len(), client.requests(),
        if failed.is_empty() { String::new() } else { format!("; failed: {}", failed.join(" ")) });
    if !failed.is_empty() {
        std::process::exit(1);
    }
    Ok(())
}
