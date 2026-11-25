//! A fund's portfolio over time, one row per N-PORT report on disk (one a
//! quarter, from late 2019):
//!
//!     edgar-funds && fred-fetch DGS3MO
//!     cargo run --release -p equities --example fund_history -- MINT
//!
//! Per report: net assets; fixed- and floating-rate debt; the fixed-rate
//! debt maturing within a year and its weighted maturity; corporate and
//! government issuers (Treasury, agencies, sponsored enterprises);
//! securitized debt (mortgage- and asset-backed, CLOs); the weighted coupon
//! of the debt against the 3-month Treasury yield that day; and the number
//! of holdings. All percentages of net assets.

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let ticker = std::env::args().nth(1).ok_or("usage: fund_history TICKER")?;
    let edgar = edgar::Cache::new(edgar::Cache::default_dir());
    let class = edgar.fund_class(&ticker)?;
    let bills = fred::Cache::new(fred::Cache::default_dir()).series("DGS3MO")?;
    let bill_on = |d: chrono::NaiveDate| bills.observations.iter().rev()
        .find(|o| o.date <= d && o.value.is_some()).and_then(|o| o.value);

    let portfolios = edgar.portfolios(&class.series_id)?;
    println!("{} ({}): {} N-PORT reports\n", ticker, class.series_id, portfolios.len());
    println!("{:<10} {:>7} {:>6} {:>6} {:>6} {:>5} {:>6} {:>6} {:>6} {:>6} {:>5} {:>6} {:>6}",
        "report", "$bn", "fixed", "float", "<1y", "yrs", "corp", "govt", "secur", "coupon", "3m", "spread", "count");
    for p in &portfolios {
        let sum = |f: &dyn Fn(&edgar::Holding) -> bool| p.holdings.iter().filter(|h| f(h)).map(|h| h.percent).sum::<f64>();
        let fixed: Vec<_> = p.holdings.iter().filter(|h| h.coupon_type.as_deref() == Some("Fixed") && h.maturity.is_some()).collect();
        let years = |h: &edgar::Holding| (h.maturity.unwrap() - p.report_date).num_days() as f64 / 365.25;
        let fixed_w: f64 = fixed.iter().map(|h| h.percent).sum();
        let maturity = if fixed_w > 0.0 { fixed.iter().map(|h| h.percent * years(h)).sum::<f64>() / fixed_w } else { 0.0 };
        let debt: Vec<_> = p.holdings.iter().filter(|h| h.coupon.is_some() && h.percent > 0.0).collect();
        let debt_w: f64 = debt.iter().map(|h| h.percent).sum();
        let coupon = if debt_w > 0.0 { Some(debt.iter().map(|h| h.percent * h.coupon.unwrap()).sum::<f64>() / debt_w) } else { None };
        let bill = bill_on(p.report_date);
        println!("{:<10} {:>7.2} {:>6.1} {:>6.1} {:>6.1} {:>5.2} {:>6.1} {:>6.1} {:>6.1} {:>6} {:>5} {:>6} {:>6}",
            p.report_date, p.net_assets / 1e9,
            fixed_w, sum(&|h| matches!(h.coupon_type.as_deref(), Some("Floating") | Some("Variable"))),
            fixed.iter().filter(|h| years(h) < 1.0).map(|h| h.percent).sum::<f64>(), maturity,
            sum(&|h| h.issuer_type == "CORP"),
            sum(&|h| matches!(h.issuer_type.as_str(), "UST" | "USGA" | "USGSE")),
            sum(&|h| h.asset_category.starts_with("ABS")),
            coupon.map(|c| format!("{:.2}", c)).unwrap_or_default(),
            bill.map(|b| format!("{:.2}", b)).unwrap_or_default(),
            coupon.zip(bill).map(|(c, b)| format!("{:+.2}", c - b)).unwrap_or_default(),
            p.holdings.len());
    }
    Ok(())
}
