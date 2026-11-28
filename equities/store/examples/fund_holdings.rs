//! A fund's portfolio from its latest N-PORT report on disk:
//!
//!     edgar-funds
//!     cargo run --release -p store --example fund_holdings -- MINT [--top 20]
//!
//! Net assets and the report date; the portfolio by asset category, issuer
//! type and country (percent of net assets); for debt, the coupon type and,
//! for fixed-rate debt, the time to maturity (floating-rate notes reset
//! their coupon every month or quarter, so their final maturity says little
//! about their interest rate risk); the weighted coupon; then the largest
//! issuers.

use std::collections::HashMap;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ticker = args.first().ok_or("usage: fund_holdings TICKER [--top N]")?;
    let top: usize = args.iter().position(|a| a == "--top").and_then(|i| args.get(i + 1)).map(|n| n.parse()).transpose()?.unwrap_or(20);
    let edgar = edgar::Cache::new(edgar::Cache::default_dir());
    let class = edgar.fund_class(ticker)?;
    let p = edgar.portfolio(&class.series_id)?.ok_or("no N-PORT report on disk: run edgar-funds")?;
    let h = &p.holdings;
    println!("{} ({}): N-PORT {} for {}, net assets ${:.2}bn, {} holdings\n",
        ticker, class.series_id, p.accession, p.report_date, p.net_assets / 1e9, h.len());

    let breakdown = |title: &str, key: &dyn Fn(&edgar::Holding) -> String, n: usize| {
        let mut by: HashMap<String, (f64, usize)> = HashMap::new();
        for x in h {
            let e = by.entry(key(x)).or_default();
            e.0 += x.percent;
            e.1 += 1;
        }
        let mut v: Vec<_> = by.into_iter().collect();
        v.sort_by(|a, b| b.1.0.total_cmp(&a.1.0));
        println!("{}:", title);
        for (k, (pct, count)) in v.iter().take(n) {
            println!("  {:<28} {:>7.1}%  {:>5}", if k.is_empty() { "(none)" } else { k }, pct, count);
        }
        if v.len() > n {
            let rest: f64 = v[n..].iter().map(|x| x.1.0).sum();
            println!("  {:<28} {:>7.1}%", format!("{} others", v.len() - n), rest);
        }
        println!();
    };
    breakdown("Asset category", &|x| category(&x.asset_category), 10);
    breakdown("Issuer type", &|x| issuer_type(&x.issuer_type), 10);
    breakdown("Country", &|x| x.country.clone(), 8);
    breakdown("Coupon", &|x| x.coupon_type.clone().unwrap_or_else(|| "not debt".into()), 6);

    // Fixed-rate debt by time to maturity from the report date.
    let fixed: Vec<_> = h.iter().filter(|x| x.coupon_type.as_deref() == Some("Fixed") && x.maturity.is_some()).collect();
    if !fixed.is_empty() {
        let years = |x: &edgar::Holding| (x.maturity.unwrap() - p.report_date).num_days() as f64 / 365.25;
        let buckets = [(0.0, 0.25, "under 3 months"), (0.25, 1.0, "3 months–1 year"), (1.0, 2.0, "1–2 years"), (2.0, 3.0, "2–3 years"), (3.0, 5.0, "3–5 years"), (5.0, f64::MAX, "5 years and more")];
        println!("Fixed-rate debt by time to maturity:");
        for (a, b, label) in buckets {
            let pct: f64 = fixed.iter().filter(|x| { let y = years(x); y >= a && y < b }).map(|x| x.percent).sum();
            println!("  {:<28} {:>7.1}%", label, pct);
        }
        let w: f64 = fixed.iter().map(|x| x.percent).sum();
        println!("  weighted maturity {:.2} years\n", fixed.iter().map(|x| x.percent * years(x)).sum::<f64>() / w);
    }
    let debt: Vec<_> = h.iter().filter(|x| x.coupon.is_some() && x.percent > 0.0).collect();
    if !debt.is_empty() {
        let w: f64 = debt.iter().map(|x| x.percent).sum();
        println!("Weighted coupon of the debt: {:.2}% ({:.1}% of net assets)\n", debt.iter().map(|x| x.percent * x.coupon.unwrap()).sum::<f64>() / w, w);
    }

    let mut issuers: HashMap<&str, f64> = HashMap::new();
    for x in h { *issuers.entry(x.issuer.as_str()).or_default() += x.percent; }
    let mut v: Vec<_> = issuers.into_iter().collect();
    v.sort_by(|a, b| b.1.total_cmp(&a.1));
    println!("Largest issuers ({} in all):", v.len());
    for (name, pct) in v.iter().take(top) {
        println!("  {:<44} {:>6.2}%", name.chars().take(44).collect::<String>(), pct);
    }
    Ok(())
}

fn category(c: &str) -> String {
    match c {
        "DBT" => "debt",
        "EC" => "common equity",
        "EP" => "preferred equity",
        "ABS-MBS" => "mortgage-backed",
        "ABS-CBDO" => "CLOs and other CDOs",
        "ABS-APCP" => "asset-backed commercial paper",
        "ABS-O" => "other asset-backed",
        "STIV" => "short-term investment vehicle",
        "RA" => "repurchase agreements",
        "DIR" => "interest rate derivatives",
        "DFE" => "currency derivatives",
        "DCR" => "credit derivatives",
        "DE" => "equity derivatives",
        "" => "",
        other => other,
    }.to_string()
}

fn issuer_type(c: &str) -> String {
    match c {
        "CORP" => "corporate",
        "UST" => "U.S. Treasury",
        "USGA" => "U.S. government agency",
        "USGSE" => "U.S. sponsored enterprise",
        "MUN" => "municipal",
        "NUSS" => "foreign sovereign",
        "RF" => "registered fund",
        "PF" => "private fund",
        "" => "",
        other => other,
    }.to_string()
}
