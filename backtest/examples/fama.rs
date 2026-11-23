//! Fama (1965) and Fama & Blume (1966), again: the Dow 30's daily prices,
//! tested for a random walk and traded with Alexander's filter rules.
//!
//!     cargo run --release -p backtest --example fama [-- FROM TO]
//!
//! Dates as YYYY-MM-DD; default, all the prices in the tiingo cache (from
//! 1990). The tickers are equities/dow/all.txt: today's Dow, so this has the
//! survivorship bias equities/FORECASTING.md describes.

use std::error::Error;

use backtest::{filter, mean, random_walk, Prices};
use chrono::NaiveDate;

const LAGS: [usize; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
const INTERVALS: [usize; 3] = [4, 9, 16];
const FILTERS: [f64; 12] = [0.005, 0.01, 0.02, 0.03, 0.05, 0.075, 0.10, 0.15, 0.20, 0.25, 0.30, 0.50];

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let date = |i: usize| args.get(i).map(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d")).transpose();
    let (from, to) = (date(0)?, date(1)?);
    let list = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../equities/dow/all.txt"))?;
    let tickers: Vec<&str> = list.lines().filter(|l| !l.starts_with('#')).flat_map(|l| l.split_whitespace()).collect();
    let cache = tiingo::Cache::new(tiingo::Cache::default_dir());
    let stocks: Vec<Prices> = tickers.iter()
        .filter_map(|t| Prices::load(&cache, t, from, to).map_err(|e| eprintln!("{}: {}", t, e)).ok())
        .filter(|p| p.adjusted.len() > 250)
        .collect();
    let first = stocks.iter().filter_map(|s| s.dates.first()).min().unwrap();
    let last = stocks.iter().filter_map(|s| s.dates.last()).max().unwrap();
    println!("{} stocks, {} to {}\n", stocks.len(), first, last);

    // Table 1: serial correlations of daily changes, and of 4-, 9-, 16-day changes.
    println!("Serial correlation of log price changes (Fama 1965, tables 10–11)");
    print!("{:<6} {:>6} ", "", "days");
    for l in LAGS { print!("{:>7}", format!("lag {}", l)); }
    for i in INTERVALS { print!("{:>8}", format!("{}-day", i)); }
    println!();
    let mut by_lag = vec![Vec::new(); LAGS.len()];
    let mut by_interval = vec![Vec::new(); INTERVALS.len()];
    for s in &stocks {
        let r = s.log_returns();
        print!("{:<6} {:>6} ", s.ticker, r.len());
        for (k, &l) in LAGS.iter().enumerate() {
            let c = random_walk::serial_correlation(&r, l);
            by_lag[k].push(c);
            print!("{:>7.3}", c);
        }
        for (k, &i) in INTERVALS.iter().enumerate() {
            let c = random_walk::serial_correlation(&random_walk::differenced(&r, i), 1);
            by_interval[k].push(c);
            print!("{:>8.3}", c);
        }
        println!();
    }
    print!("{:<6} {:>6} ", "mean", "");
    for v in &by_lag { print!("{:>7.3}", mean(v)); }
    for v in &by_interval { print!("{:>8.3}", mean(v)); }
    let n = stocks.iter().map(|s| s.adjusted.len()).sum::<usize>() / stocks.len();
    println!("\n(about ±{:.3} would be significant at 5% for a stock with {} days)\n", 2.0 / (n as f64).sqrt(), n);

    // Table 2: runs of daily changes.
    println!("Runs of daily price changes (Fama 1965, table 13)");
    println!("{:<6} {:>8} {:>10} {:>8}", "", "actual", "expected", "z");
    let mut zs = Vec::new();
    for s in &stocks {
        let runs = random_walk::runs(&s.log_returns());
        zs.push(runs.z());
        println!("{:<6} {:>8.0} {:>10.1} {:>8.2}", s.ticker, runs.actual, runs.expected, runs.z());
    }
    println!("{:<6} {:>8} {:>10} {:>8.2}  (negative: fewer runs than chance, i.e. trends)\n", "mean", "", "", mean(&zs));

    // Table 3: filter rules against buying and holding, per year, averaged.
    for cost in [0.0, 0.001, 0.01] {
        println!("Filter rules, average over the stocks, per year (Fama & Blume 1966), cost {:.1}% a switch", cost * 100.0);
        println!("{:>7} {:>9} {:>9} {:>9} {:>11} {:>9}", "filter", "rule", "long", "short", "buy & hold", "trades/yr");
        for x in FILTERS {
            let outcomes: Vec<filter::Outcome> = stocks.iter().map(|s| filter::run(&s.adjusted, x, cost)).collect();
            let per_year = |f: &dyn Fn(&filter::Outcome) -> f64| mean(&outcomes.iter()
                .filter(|o| o.days > 0).map(|o| f(o) / (o.days as f64 / 252.0)).collect::<Vec<_>>());
            println!("{:>6.1}% {:>8.1}% {:>8.1}% {:>8.1}% {:>10.1}% {:>9.1}", x * 100.0,
                per_year(&|o| o.rule) * 100.0, per_year(&|o| o.long) * 100.0, per_year(&|o| o.short) * 100.0,
                per_year(&|o| o.buy_and_hold) * 100.0, per_year(&|o| o.trades as f64));
        }
        println!();
    }
    Ok(())
}
