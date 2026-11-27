//! Bring the fund sources up to date: the fund ticker map and every
//! quarter's N-CEN and N-PORT data sets (see `edgar::funds`).
//!
//!     edgar-funds [--force QUARTER] [ncen] [nport]     # both by default
//!
//! A quarter already on disk is skipped; `--force 2026q3` downloads it again
//! (the SEC sometimes republishes the latest). N-PORT is about 400 MB a
//! quarter.

use std::error::Error;

use edgar::{Cache, Client, DataSet, USER_AGENT};

fn main() -> Result<(), Box<dyn Error>> {
    let mut sets = Vec::new();
    let mut force: Option<String> = None;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--force" => force = Some(args.next().ok_or("--force needs a quarter")?),
            "ncen" => sets.push(DataSet::Ncen),
            "nport" => sets.push(DataSet::Nport),
            a => return Err(format!("unknown argument {} (usage: edgar-funds [--force QUARTER] [ncen] [nport])", a).into()),
        }
    }
    if sets.is_empty() {
        sets = DataSet::ALL.to_vec();
    }
    let client = Client::new(USER_AGENT, Cache::new(Cache::default_dir()))?;
    client.fetch_fund_tickers()?;
    eprintln!("fund tickers: {} classes", client.cache().fund_classes()?.len());
    for set in sets {
        let quarters = client.data_set_quarters(set)?;
        eprintln!("{}: {} quarters, {} to {}", set.name(), quarters.len(),
            quarters.first().map_or("", |q| &q.0), quarters.last().map_or("", |q| &q.0));
        for (q, url) in &quarters {
            if client.fetch_data_set(set, q, url, force.as_deref() == Some(q.as_str()))? {
                eprintln!("  {} downloaded", q);
            }
        }
    }
    Ok(())
}
