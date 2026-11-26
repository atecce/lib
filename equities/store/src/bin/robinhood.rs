//! Import Robinhood account activity reports into Postgres: the buys, sells
//! and cash dividends before Plaid's first transaction in each account
//! (store::robinhood, Store::write_robinhood_report).
//!
//!     store-robinhood <account name> <report.csv> [<account name> <report.csv> ...]
//!
//! The account name is portfolio_account.name, as Plaid gave it ("Robinhood
//! individual"). Reports stay in ~/.plaid/reports (private, not in the repo).

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.len() % 2 != 0 {
        return Err("usage: store-robinhood <account name> <report.csv> [...]".into());
    }
    let mut store = store::Store::connect(&store::default_url())?;
    for m in store.migrate()? {
        eprintln!("applied {}", m);
    }
    for pair in args.chunks(2) {
        let (account, path) = (&pair[0], &pair[1]);
        let (rows, skipped) = store::robinhood::parse(&std::fs::read_to_string(path)?)?;
        let (written, cutoff, left_out) = store.write_robinhood_report(account, &rows)?;
        eprintln!("{}: {} rows read, {} written (before {}, where Plaid begins), {} of securities not held",
                  account, rows.len(), written, cutoff, left_out);
        let skipped: Vec<String> = skipped.iter().map(|(code, n)| format!("{} {}", code, n)).collect();
        eprintln!("  not read: {}", skipped.join(", "));
    }
    Ok(())
}
