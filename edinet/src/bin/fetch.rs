//! Bring the EDINET cache up to date: the day lists, then the reports of the
//! given companies not yet on disk.
//!
//!     edinet-fetch [--from YYYY-MM-DD] [--list FILE ...] [COMPANY ...]
//!     edinet-fetch [--from YYYY-MM-DD] --listed
//!
//! A company is a Tokyo ticker ("7203.T"), securities code or EDINET code.
//! Day lists are fetched from `--from` (default: ten years ago, as far back
//! as the API goes) to today; past days are fetched once. Needs an API key
//! in `EDINET_API_KEY`.
//!
//! `--listed` fetches the day lists, then prints the Tokyo tickers of every
//! listed company with reports, one per line, for a list of them all.
//!
//! Otherwise prints, one per line on stdout, the companies that got reports not on
//! disk before; progress goes to stderr. Exits non-zero if the day lists
//! could not be fetched or a company had no reports at all.

use std::error::Error;

use chrono::{Local, NaiveDate};
use edinet::{sec_code, Cache, Client, Document};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    let today = Local::now().date_naive();
    let mut from = today - chrono::Duration::days(3653);
    let mut companies = Vec::new();
    let mut listed = false;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--from" => from = args.next().ok_or("--from needs a date")?.parse::<NaiveDate>()?,
            "--listed" => listed = true,
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

    let key = std::env::var("EDINET_API_KEY").map_err(|_| "EDINET_API_KEY is not set")?;
    let client = Client::new(key, Cache::new(Cache::default_dir()))?;

    let days = client.fetch_days(from, today)?;
    eprintln!("day lists: {} fetched, {} requests", days, client.requests());

    if listed {
        for t in client.cache().listed()? {
            println!("{}", t);
        }
        return Ok(());
    }

    // The day lists, read once for all the companies.
    let all = client.cache().all_reports()?;
    let mut failed = Vec::new();
    for company in &companies {
        let before = client.requests();
        let sec = sec_code(company);
        let reports: Vec<&Document> = all.iter()
            .filter(|d| d.edinet_code.as_deref() == Some(company.as_str()) || d.sec_code.as_deref() == Some(sec.as_str()))
            .collect();
        if reports.is_empty() {
            eprintln!("{}: no reports found", company);
            failed.push(company.as_str());
            continue;
        }
        let (mut new, mut errors) = (0, 0);
        for &d in &reports {
            match client.fetch_document(d) {
                Ok(true) => new += 1,
                Ok(false) => (),
                Err(e) => {
                    eprintln!("{} {} {}: {}", company, d.doc_id, d.description.as_deref().unwrap_or(""), e);
                    errors += 1;
                },
            }
        }
        let name = reports.last().and_then(|d| d.filer_name.as_deref()).unwrap_or("");
        eprintln!("{} {}: {} reports, {} new, {} requests{}", company, name, reports.len(), new,
            client.requests() - before, if errors > 0 { format!(", {} failed", errors) } else { String::new() });
        if new > 0 { println!("{}", company); }
    }
    if !failed.is_empty() {
        eprintln!("failed: {}", failed.join(" "));
        std::process::exit(1);
    }
    Ok(())
}
