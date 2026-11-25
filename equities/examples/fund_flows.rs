//! Where a fund's money went: its shareholders' net purchases each quarter
//! (N-PORT's monthly flows, summed), against what else the money could
//! have done:
//!
//!     edgar-funds && fred-fetch MORTGAGE30US DGS3MO MMMFFAQ027S WRMFNS
//!     cargo run --release -p equities --example fund_flows -- MINT
//!
//! Per quarter: the fund's net flow and net assets ($bn); the 30-year
//! mortgage rate (Freddie Mac's survey, the quarter's average) and the
//! 3-month Treasury yield (likewise); money market funds' total financial
//! assets (the Fed's financial accounts, $tn, at the quarter's end) and
//! their change in the quarter; retail money funds ($bn, the quarter's last
//! week).

use std::error::Error;

use chrono::{Datelike, NaiveDate};

fn main() -> Result<(), Box<dyn Error>> {
    let ticker = std::env::args().nth(1).ok_or("usage: fund_flows TICKER")?;
    let edgar = edgar::Cache::new(edgar::Cache::default_dir());
    let class = edgar.fund_class(&ticker)?;
    let fred = fred::Cache::new(fred::Cache::default_dir());
    let mortgage = fred.series("MORTGAGE30US")?;
    let bills = fred.series("DGS3MO")?;
    let money_funds = fred.series("MMMFFAQ027S")?;
    let retail = fred.series("WRMFNS")?;

    let mean_between = |s: &fred::Series, a: NaiveDate, b: NaiveDate| {
        let v: Vec<f64> = s.observations.iter().filter(|o| o.date > a && o.date <= b).filter_map(|o| o.value).collect();
        (!v.is_empty()).then(|| v.iter().sum::<f64>() / v.len() as f64)
    };
    let last_on_or_before = |s: &fred::Series, d: NaiveDate| s.observations.iter().rev().find(|o| o.date <= d && o.value.is_some()).and_then(|o| o.value);
    // The financial accounts date a quarter by its first day.
    let quarter_start = |d: NaiveDate| NaiveDate::from_ymd_opt(d.year(), (d.month0() / 3) * 3 + 1, 1).unwrap();
    let money_at = |d: NaiveDate| money_funds.observations.iter().find(|o| o.date == quarter_start(d)).and_then(|o| o.value);

    let portfolios = edgar.portfolios(&class.series_id)?;
    println!("{} ({}): net flows by quarter\n", ticker, class.series_id);
    println!("{:<10} {:>8} {:>8} | {:>8} {:>6} | {:>8} {:>8} {:>9}", "quarter", "flow $bn", "assets", "mortgage", "3m", "MMF $tn", "change", "retail $bn");
    let mut cumulative = 0.0;
    for p in &portfolios {
        let start = p.report_date.checked_sub_months(chrono::Months::new(3)).unwrap();
        let flow: f64 = p.flows.iter().map(|f| f.net()).sum::<f64>() / 1e9;
        cumulative += flow;
        let mmf = money_at(p.report_date);
        let prior = money_at(start);
        let cell = |v: Option<f64>, digits: usize| v.map(|v| format!("{:.*}", digits, v)).unwrap_or_default();
        println!("{:<10} {:>+8.2} {:>8.2} | {:>8} {:>6} | {:>8} {:>8} {:>9}", p.report_date, flow, p.net_assets / 1e9,
            cell(mean_between(&mortgage, start, p.report_date), 2), cell(mean_between(&bills, start, p.report_date), 2),
            cell(mmf.map(|m| m / 1e6), 2), mmf.zip(prior).map(|(a, b)| format!("{:+.2}", (a - b) / 1e6)).unwrap_or_default(),
            cell(last_on_or_before(&retail, p.report_date), 0));
    }
    println!("\nnet flow over all {} quarters: {:+.2}bn", portfolios.len(), cumulative);
    Ok(())
}
