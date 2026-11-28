//! Bring the MOPS cache up to date for some companies: every quarter's
//! report from 2013 (or `--from YEAR`) that isn't on disk.
//!
//!     mops-fetch [--from YEAR] [--list FILE ...] [COMPANY ...]
//!
//! A company is a Taipei ticker ("2330.TW") or MOPS code ("2330"). Prints,
//! one per line on stdout, the companies that got reports not on disk
//! before; progress goes to stderr. Exits non-zero if a company could not be
//! fetched at all.

use std::error::Error;

use chrono::Datelike;
use mops::{Cache, Client};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    let mut from = mops::FIRST_YEAR;
    let mut companies = Vec::new();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--from" => from = args.next().ok_or("--from needs a year")?.parse()?,
            "--list" => {
                let path = args.next().ok_or("--list needs a file")?;
                for line in std::fs::read_to_string(&path)?.lines() {
                    let line = line.split('#').next().unwrap_or("");
                    companies.extend(line.split([' ', '\t', ',']).filter(|t| !t.is_empty()).map(str::to_string));
                }
            },
            a if a.starts_with("--") => return Err(format!("unknown option {}", a).into()),
            c => companies.push(c.to_string()),
        }
    }
    // Taipei companies only: MOPS knows nothing of the others.
    companies.retain(|c| c.to_uppercase().ends_with(".TW") || c.chars().all(|ch| ch.is_ascii_alphanumeric()) && c.chars().next().is_some_and(|ch| ch.is_ascii_digit()));
    if companies.is_empty() {
        return Err("usage: mops-fetch [--from YEAR] [--list FILE ...] [COMPANY ...]".into());
    }

    let client = Client::new(Cache::new(Cache::default_dir()))?;
    let today = chrono::Local::now().date_naive();
    let mut failed = Vec::new();
    for company in &companies {
        let code = mops::code(company);
        let (mut new, mut missing, mut errors) = (0, 0, 0);
        let before = client.requests();
        for year in from..=today.year() {
            for quarter in 1..=4 {
                // Not before the quarter has ended.
                if mops::quarter_end(year, quarter) >= today {
                    continue;
                }
                match client.fetch(&code, year, quarter) {
                    Ok(mops::Fetched::New) => new += 1,
                    Ok(mops::Fetched::Missing) => missing += 1,
                    Ok(mops::Fetched::Cached) => (),
                    Err(e) => {
                        eprintln!("{} {}Q{}: {}", company, year, quarter, e);
                        errors += 1;
                    },
                }
            }
        }
        eprintln!("{}: {} new, {} missing, {} requests{}", company, new, missing, client.requests() - before,
            if errors > 0 { format!(", {} failed", errors) } else { String::new() });
        if new > 0 {
            println!("{}", company);
        }
        if errors > 0 && new == 0 && client.cache().reports(company).map_or(true, |r| r.is_empty()) {
            failed.push(company.as_str());
        }
    }
    if !failed.is_empty() {
        eprintln!("failed: {}", failed.join(" "));
        std::process::exit(1);
    }
    Ok(())
}
