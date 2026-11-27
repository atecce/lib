//! Bring the EDGAR cache up to date for some companies: the ticker map,
//! their filing lists, and the cover and statement pages of any 10-Q, 10-K
//! or 20-F not yet on disk.
//!
//!     fetch [--bulk] [--list FILE ...] [TICKER ...]
//!
//! `--list` reads tickers from a file, separated by spaces, commas or
//! newlines, with `#` starting a comment. `--bulk` reads filing lists from
//! EDGAR's nightly submissions.zip (see `edgar::Client::bulk_submissions`).
//!
//! Prints, one per line on stdout, the tickers that got filings not on disk
//! before, for a reader to update just those; progress goes to stderr.
//! Exits non-zero if a company could not be looked up or listed at all, so a
//! scheduler can retry. A single filing that fails (EDGAR's occasional 504s
//! on old filings) is reported and tried again on the next run.

use std::error::Error;

use edgar::{Cache, Client, EdgarError, FORMS, USER_AGENT};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    let mut bulk = false;
    let mut tickers = Vec::new();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--bulk" => bulk = true,
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
    if tickers.is_empty() {
        return Err("usage: fetch [--bulk] [--list FILE ...] [TICKER ...]".into());
    }

    let mut client = Client::new(USER_AGENT, Cache::new(Cache::default_dir()))?;
    if bulk {
        client = client.bulk_submissions();
    }

    let mut failed = Vec::new();
    for ticker in &tickers {
        let before = client.requests();
        match fetch(&client, ticker) {
            Ok(Fetched { filings, new, old, errors }) => {
                eprintln!("{}: {} filings, {} new ({} predate rendered statements), {} requests{}",
                    ticker, filings, new, old, client.requests() - before,
                    if errors > 0 { format!(", {} failed", errors) } else { String::new() });
                if new > 0 { println!("{}", ticker); }
            },
            Err(e) => {
                eprintln!("{}: {}", ticker, e);
                failed.push(ticker.as_str());
            },
        }
    }
    eprintln!("{} companies, {} requests{}", tickers.len(), client.requests(),
        if failed.is_empty() { String::new() } else { format!("; failed: {}", failed.join(" ")) });
    if !failed.is_empty() {
        std::process::exit(1);
    }
    Ok(())
}

struct Fetched {
    filings: usize,
    /// Filings with statements that were not on disk before.
    new: usize,
    /// Filings before EDGAR rendered statements.
    old: usize,
    errors: usize,
}

fn fetch(client: &Client, ticker: &str) -> Result<Fetched, Box<dyn Error>> {
    let cik = client.cik(ticker)?;
    let filings = client.filings(cik, &FORMS)?;
    let (mut new, mut old, mut errors) = (0, 0, 0);
    for f in &filings {
        let had = client.cache().filing(f).is_ok();
        match client.fetch_filing(f) {
            Ok(_) => if !had { new += 1 },
            Err(e) if matches!(e.downcast_ref(), Some(EdgarError::NoFilingSummary)) => old += 1,
            Err(e) => {
                eprintln!("{} {} {} ({}): {}", ticker, f.form, f.report_date, f.accession, e);
                errors += 1;
            },
        }
    }
    Ok(Fetched { filings: filings.len(), new, old, errors })
}
