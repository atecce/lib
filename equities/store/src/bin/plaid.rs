//! Sync every connected Plaid Item (plaid-link) into Postgres: today's
//! holdings (portfolio_holding, as of today), and investment transactions
//! (portfolio_transaction): the last 24 months the first time, then from a
//! month before the last sync, for late-posted ones.
//!
//!     store-plaid
//!
//! Applies the migrations first. Run daily (pub.atec.plaid.plist) and when
//! the app asks for fresh holdings.

use std::error::Error;

use chrono::{Duration, Local};

fn main() -> Result<(), Box<dyn Error>> {
    let client = plaid::Client::from_env_file()?;
    let items = plaid::Item::all()?;
    if items.is_empty() {
        return Err("no Plaid connections: run plaid-link".into());
    }
    let mut store = store::Store::connect(&store::default_url())?;
    for m in store.migrate()? {
        eprintln!("applied {}", m);
    }
    let today = Local::now().date_naive();
    for item in &items {
        let name = item.institution.as_deref().unwrap_or("an institution");
        let start = match store.portfolio_synced(&item.item_id)? {
            Some(at) => at.date_naive() - Duration::days(30),
            None => today - Duration::days(730),
        };
        let holdings = client.holdings(item)?;
        let (transactions, securities) = client.investment_transactions(item, &start.to_string(), &today.to_string())?;
        let w = store.write_portfolio(&item.item_id, item.institution.as_deref(), &holdings, &transactions, &securities, today)?;
        eprintln!("{}: {} accounts, {} holdings as of {}, {} transactions since {}", name, w.accounts, w.holdings, today, w.transactions, start);
    }
    Ok(())
}
