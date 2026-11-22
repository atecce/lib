//! Which companies got filings since a time, from the cache alone: for a
//! reader to update just those, whichever run fetched them.
//!
//!     changed STAMP [--list FILE ...] [TICKER ...]
//!
//! STAMP is a file whose modification time is the cutoff (e.g. touched after
//! the last update); without it, every company counts as changed. Prints the
//! changed tickers one per line; tickers not in the ticker map are reported
//! on stderr. Tokyo tickers (7203.T) are EDINET's and skipped.

use std::error::Error;
use std::time::SystemTime;

use edgar::Cache;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    let stamp = args.next().ok_or("usage: changed STAMP [--list FILE ...] [TICKER ...]")?;
    let mut tickers = Vec::new();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--list" => tickers.extend(edgar::read_list(args.next().ok_or("--list needs a file")?)?),
            a if a.starts_with("--") => return Err(format!("unknown option {}", a).into()),
            t => tickers.push(t.to_string()),
        }
    }
    let since = std::fs::metadata(&stamp).and_then(|m| m.modified()).unwrap_or(SystemTime::UNIX_EPOCH);
    let cache = Cache::new(Cache::default_dir());
    for ticker in tickers.iter().filter(|t| !t.to_uppercase().ends_with(".T")) {
        match cache.cik(ticker) {
            Ok(cik) => if cache.changed_since(cik, since) { println!("{}", ticker) },
            Err(e) => eprintln!("{}: {}", ticker, e),
        }
    }
    Ok(())
}
