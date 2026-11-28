//! Every index fund in the N-CEN census (each series' latest filing), with
//! its share classes' tickers:
//!
//!     cargo run --release -p store --example index_funds [-- --etf] [--tickers]
//!
//! `--etf` keeps only exchange-traded funds. `--tickers` prints just the
//! tickers, one per line, for `tiingo-fetch --list`. Otherwise a summary,
//! then a row per fund, largest first: tickers, net assets (monthly average,
//! $bn), the return less the index's after fees, and the name.
//!
//! "Index fund" is the fund's own answer to N-CEN item C.3.a. Series that
//! have stopped filing N-CEN (merged, liquidated) are left out when their
//! latest census is more than two years old.

use std::collections::HashMap;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let etf_only = args.iter().any(|a| a == "--etf");
    let tickers_only = args.iter().any(|a| a == "--tickers");

    let edgar = edgar::Cache::new(edgar::Cache::default_dir());
    let census = edgar.latest_census()?;
    let mut tickers: HashMap<String, Vec<String>> = HashMap::new();
    for c in edgar.fund_classes()? {
        tickers.entry(c.series_id).or_default().push(c.ticker);
    }
    let newest = census.values().map(|c| c.period).max().ok_or("no N-CEN on disk: run edgar-funds")?;
    let cutoff = newest - chrono::Duration::days(730);

    let live: Vec<_> = census.values().filter(|c| c.period >= cutoff).collect();
    let mut funds: Vec<_> = live.iter().filter(|c| c.is_index && (!etf_only || c.is_etf)).collect();
    funds.sort_by(|a, b| b.monthly_avg_net_assets.unwrap_or(0.0).total_cmp(&a.monthly_avg_net_assets.unwrap_or(0.0)));

    if tickers_only {
        for f in &funds {
            for t in tickers.get(&f.series_id).into_iter().flatten() {
                println!("{}", t);
            }
        }
        return Ok(());
    }

    let assets = |v: &[&&edgar::Census]| v.iter().filter_map(|c| c.monthly_avg_net_assets).sum::<f64>() / 1e9;
    let all: Vec<_> = live.iter().collect();
    let etfs: Vec<_> = live.iter().filter(|c| c.is_etf).collect();
    let index: Vec<_> = live.iter().filter(|c| c.is_index).collect();
    let index_etfs: Vec<_> = live.iter().filter(|c| c.is_index && c.is_etf).collect();
    println!("N-CEN census, series filing since {}:", cutoff);
    println!("  {:<22} {:>6} series  ${:>7.0}bn", "all", all.len(), assets(&all));
    println!("  {:<22} {:>6} series  ${:>7.0}bn", "ETFs", etfs.len(), assets(&etfs));
    println!("  {:<22} {:>6} series  ${:>7.0}bn", "index funds", index.len(), assets(&index));
    println!("  {:<22} {:>6} series  ${:>7.0}bn", "index ETFs", index_etfs.len(), assets(&index_etfs));
    let with_tickers = funds.iter().filter(|f| tickers.contains_key(&f.series_id)).count();
    println!("  listed below: {} ({} with tickers)\n", funds.len(), with_tickers);

    println!("{:<24} {:>9} {:>9}  {}", "tickers", "$bn", "vs index", "name");
    for f in &funds {
        let t = tickers.get(&f.series_id).map(|t| t.join(" ")).unwrap_or_default();
        println!("{:<24} {:>9.1} {:>9}  {}", t.chars().take(24).collect::<String>(),
            f.monthly_avg_net_assets.unwrap_or(0.0) / 1e9,
            f.tracking_after_fees.map(|t| format!("{:+.2}", t)).unwrap_or_default(), f.name);
    }
    Ok(())
}
