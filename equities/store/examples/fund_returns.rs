//! Funds' total returns as they report them on N-PORT, compounded by
//! calendar year, against 3-month Treasury bills (FRED, TB3MS):
//!
//!     edgar-funds && fred-fetch TB3MS
//!     cargo run --release -p store --example fund_returns -- MINT EWT EWJ
//!
//! For each ticker, the N-CEN census first (ETF? index fund? its tracking
//! difference), then a row per year: the fund's return, the bills', and the
//! difference. A year is partial where N-PORT doesn't cover all twelve
//! months (it starts in late 2019; the count of months is shown).
//!
//! The bill return compounds each month's average yield over the month:
//! Π (1 + y/1200), an approximation that ignores the discount basis.

use std::collections::BTreeMap;
use std::error::Error;

use chrono::Datelike;

fn main() -> Result<(), Box<dyn Error>> {
    let tickers: Vec<String> = std::env::args().skip(1).collect();
    if tickers.is_empty() {
        return Err("usage: fund_returns TICKER ...".into());
    }
    let edgar = edgar::Cache::new(edgar::Cache::default_dir());
    let bills = fred::Cache::new(fred::Cache::default_dir()).series("TB3MS")?;
    let classes: Vec<edgar::FundClass> = tickers.iter().map(|t| edgar.fund_class(t)).collect::<Result<_, _>>()?;
    let census = edgar.latest_census()?;
    let wanted: std::collections::HashSet<&str> = classes.iter().map(|c| c.class_id.as_str()).collect();
    let returns = edgar.monthly_returns(|c| wanted.contains(c))?;

    for class in &classes {
        println!("{} ({}, {}, CIK {})", class.ticker, class.series_id, class.class_id, class.cik);
        match census.get(&class.series_id) {
            Some(c) => println!("  {}: ETF {}, index fund {}, money market {}{}; N-CEN for the year to {}",
                c.name, yn(c.is_etf), yn(c.is_index), yn(c.is_money_market),
                c.tracking_after_fees.map(|t| format!(", return less index's {:+.2}% after fees", t)).unwrap_or_default(),
                c.period),
            None => println!("  no N-CEN on disk"),
        }
        // year -> (growth, months, bill growth)
        let mut years: BTreeMap<i32, (f64, u32, f64)> = BTreeMap::new();
        for r in returns.iter().filter(|r| r.class_id == class.class_id) {
            let y = bills.month(r.month.year(), r.month.month()).ok_or(format!("no TB3MS for {}", r.month))?;
            let e = years.entry(r.month.year()).or_insert((1.0, 0, 1.0));
            e.0 *= 1.0 + r.percent / 100.0;
            e.1 += 1;
            e.2 *= 1.0 + y / 1200.0;
        }
        if years.is_empty() {
            println!("  no N-PORT returns on disk\n");
            continue;
        }
        println!("  {:<6} {:>7} {:>8} {:>8} {:>8}", "year", "months", "fund", "bills", "excess");
        let (mut fund, mut bill, mut months) = (1.0, 1.0, 0);
        for (year, (g, n, b)) in &years {
            println!("  {:<6} {:>7} {:>+7.2}% {:>+7.2}% {:>+7.2}", year, n, (g - 1.0) * 100.0, (b - 1.0) * 100.0, (g - b) * 100.0);
            fund *= g;
            bill *= b;
            months += n;
        }
        let a = |g: f64| (g.powf(12.0 / months as f64) - 1.0) * 100.0;
        println!("  {:<6} {:>7} {:>+7.2}% {:>+7.2}% {:>+7.2}   (annualized)\n", "all", months, a(fund), a(bill), a(fund) - a(bill));
    }
    Ok(())
}

fn yn(b: bool) -> &'static str {
    if b { "yes" } else { "no" }
}
