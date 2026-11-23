//! Entrepreneurial anomalies, after equities/VALUATION.md section 8: how
//! long a company's residual income lasts, against its industry's, and
//! against what its price implied.
//!
//!     cargo run --release -p store --example persistence -- AAPL [--r 0.09] [--digits 3]
//!
//! For each fiscal year: residual income x = E − r·B₋₁ (net income less the
//! cost of the equity at the start of the year); and, where Postgres has
//! prices, the market value M, the decay the price implies
//! ω = R(M − B)/(x + M − B), with R = 1 + r, and the residual income growth
//! that amounts to, ω − 1.
//!
//! Then the persistence actually realized: ω fitted by least squares through
//! the origin, x₊₁ = ω·x, each year's pair scaled by the same total assets
//! (A₋₁) so that size drops out and ω keeps its meaning. Once for the
//! company and once pooled over its industry: the issuers in Postgres whose
//! SIC code shares the first `--digits` digits (3: the industry group, 357
//! computer and office equipment for Apple's 3571). Years with book value at
//! or below zero are left out: there residual income on book means nothing.
//!
//! Values are as first reported (`fact_first`), so as known at the time.
//! The industry is what's loaded, today's survivors mostly; see section 8's
//! caveats.

use std::collections::HashMap;
use std::error::Error;

use chrono::NaiveDate;
use postgres::{Client, NoTls};

const ITEMS: [&str; 5] = ["NetIncome", "StockholdersEquity", "TotalAssets", "OperatingIncome", "SharesOutstanding"];

#[derive(Default, Clone)]
struct Year {
    end: NaiveDate,
    income: f64,
    operating: Option<f64>,
    equity: f64,
    assets: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let flag = |name: &str| args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).cloned();
    let r: f64 = flag("--r").map(|s| s.parse()).transpose()?.unwrap_or(0.09);
    let digits: usize = flag("--digits").map(|s| s.parse()).transpose()?.unwrap_or(3);
    let ticker = args.iter().enumerate()
        .find(|(i, a)| !a.starts_with("--") && (*i == 0 || !args[i - 1].starts_with("--")))
        .map(|(_, a)| a.to_uppercase()).ok_or("usage: persistence TICKER [--r 0.09] [--digits 3]")?;

    let mut db = Client::connect(&store::default_url(), NoTls)?;
    let row = db.query_opt("SELECT l.issuer_id, i.name, i.sic FROM listing l JOIN issuer i ON i.id = l.issuer_id WHERE upper(l.ticker) = $1", &[&ticker])?
        .ok_or(format!("{} isn't in Postgres", ticker))?;
    let (target, name, sic): (i32, String, Option<String>) = (row.get(0), row.get(1), row.get(2));
    let group = sic.as_deref().map(|s| s.chars().take(digits).collect::<String>());

    // The industry: issuers whose SIC starts with the same digits, with a ticker to show.
    let peers: Vec<(i32, String, String)> = match &group {
        Some(g) => db.query(
            "SELECT i.id, coalesce(min(l.ticker), i.name), i.sic FROM issuer i LEFT JOIN listing l ON l.issuer_id = i.id
             WHERE i.sic LIKE $1 || '%' GROUP BY i.id ORDER BY i.sic, 2", &[g])?
            .iter().map(|r| (r.get(0), r.get(1), r.get(2))).collect(),
        None => Vec::new(),
    };
    let mut ids: Vec<i32> = peers.iter().map(|p| p.0).collect();
    if !ids.contains(&target) { ids.push(target); }

    let items: Vec<String> = ITEMS.iter().map(|s| s.to_string()).collect();
    let rows = db.query(
        "SELECT issuer_id, item, period, period_end, currency, value FROM fact_first
         WHERE issuer_id = ANY($1) AND item = ANY($2) AND NOT derived AND period IN ('12m', 'pit')",
        &[&ids, &items])?;
    // issuer → (item, period, end, currency) → value
    let mut facts: HashMap<i32, HashMap<(String, String, NaiveDate, String), f64>> = HashMap::new();
    for r in &rows {
        facts.entry(r.get(0)).or_default().insert((r.get(1), r.get(2), r.get(3), r.get::<_, String>(4).trim().to_string()), r.get(5));
    }

    let years_of = |id: i32| -> Vec<Year> { years(facts.get(&id).cloned().unwrap_or_default()) };

    println!("{} ({}), SIC {}, r = {:.1}%\n", name, ticker, sic.as_deref().unwrap_or("unknown"), r * 100.0);
    let mine = years_of(target);
    let f = facts.get(&target).cloned().unwrap_or_default();
    println!("{:<11} {:>6} {:>7} {:>10} {:>10} {:>6} {:>7} {:>8}", "year end", "ROE", "OI/A₋₁", "x (bn)", "M (bn)", "M/B", "ω impl", "growth");
    for w in mine.windows(2) {
        let (prev, y) = (&w[0], &w[1]);
        if prev.equity <= 0.0 { continue; }
        let x = y.income - r * prev.equity;
        let market = market_value(&mut db, &ticker, &f, y.end)?;
        let (m, pb, implied) = match market {
            Some(m) if y.equity > 0.0 => {
                let omega = (1.0 + r) * (m - y.equity) / (x + m - y.equity);
                (format!("{:.0}", m / 1e9), format!("{:.1}", m / y.equity),
                 if x > 0.0 && m > y.equity { Some(omega) } else { None })
            },
            Some(m) => (format!("{:.0}", m / 1e9), "".into(), None),
            None => ("".into(), "".into(), None),
        };
        println!("{:<11} {:>5.0}% {:>6} {:>10.1} {:>10} {:>6} {:>7} {:>8}",
            y.end, y.income / prev.equity * 100.0,
            y.operating.map(|o| format!("{:.0}%", o / prev.assets * 100.0)).unwrap_or_default(),
            x / 1e9, m, pb,
            implied.map(|o| format!("{:.2}", o)).unwrap_or_default(),
            implied.map(|o| format!("{:+.1}%", (o - 1.0) * 100.0)).unwrap_or_default());
    }

    let pairs_of = |ys: &[Year]| -> Vec<(f64, f64)> {
        ys.windows(3).filter(|w| w[0].equity > 0.0 && w[1].equity > 0.0 && w[1].assets > 0.0)
            .map(|w| {
                let x0 = w[1].income - r * w[0].equity;
                let x1 = w[2].income - r * w[1].equity;
                (x0 / w[0].assets, x1 / w[0].assets)
            }).collect()
    };
    let mine_pairs = pairs_of(&mine);
    let first_last = mine.windows(2).filter(|w| w[0].equity > 0.0).map(|w| (w[1].end, w[1].income - r * w[0].equity)).collect::<Vec<_>>();
    println!();
    if let (Some(a), Some(b)) = (first_last.first(), first_last.last())
        && a.1 > 0.0 && b.1 > 0.0 && b.0 > a.0 {
        let years = (b.0 - a.0).num_days() as f64 / 365.25;
        println!("residual income grew {:.1}% a year, {} to {}", ((b.1 / a.1).powf(1.0 / years) - 1.0) * 100.0, a.0, b.0);
    }
    if let Some((omega, se, n)) = fit(&mine_pairs) {
        println!("realized ω, {}: {:.2} ± {:.2} ({} years)", ticker, omega, se, n);
    }

    let Some(g) = group else { return Ok(()) };
    println!("\nIndustry: SIC {}*, {} issuers in Postgres", g, peers.len());
    let mut pooled = Vec::new();
    for (id, t, s) in &peers {
        let p = pairs_of(&years_of(*id));
        let omega = fit(&p).map(|(o, se, n)| format!("{:>5.2} ± {:.2} ({} years)", o, se, n)).unwrap_or_else(|| "  too few years".into());
        println!("  {:<6} {:<8} {}", s, t, omega);
        pooled.extend(p);
    }
    if let Some((omega, se, n)) = fit(&pooled) {
        println!("pooled ω, SIC {}*: {:.2} ± {:.2} ({} company-years)", g, omega, se, n);
    }
    Ok(())
}

/// A company's fiscal years: each 12-month net income, with equity and
/// total assets at the same date, in the currency it reports most in,
/// consecutive ones a year apart.
fn years(facts: HashMap<(String, String, NaiveDate, String), f64>) -> Vec<Year> {
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for (item, _, _, currency) in facts.keys() {
        if item == "NetIncome" { *counts.entry(currency.as_str()).or_default() += 1; }
    }
    let Some(currency) = counts.into_iter().max_by_key(|c| c.1).map(|c| c.0.to_string()) else { return Vec::new() };
    let get = |item: &str, period: &str, end: NaiveDate| facts.get(&(item.to_string(), period.to_string(), end, currency.clone())).copied();
    let mut ends: Vec<NaiveDate> = facts.keys().filter(|k| k.0 == "NetIncome" && k.1 == "12m" && k.3 == currency).map(|k| k.2).collect();
    ends.sort();
    let all: Vec<Year> = ends.into_iter().filter_map(|end| Some(Year {
        end,
        income: get("NetIncome", "12m", end)?,
        operating: get("OperatingIncome", "12m", end),
        equity: get("StockholdersEquity", "pit", end)?,
        assets: get("TotalAssets", "pit", end)?,
    })).collect();
    // Keep the longest run of years about a year apart.
    let mut best: Vec<Year> = Vec::new();
    let mut run: Vec<Year> = Vec::new();
    for y in all {
        if let Some(last) = run.last() {
            let gap = (y.end - last.end).num_days();
            if !(330..=400).contains(&gap) {
                if gap < 330 { continue; }
                if run.len() > best.len() { best = std::mem::take(&mut run); } else { run.clear(); }
            }
        }
        run.push(y);
    }
    if run.len() > best.len() { best = run; }
    best
}

/// Market value at a fiscal year end: that day's close (unadjusted, so in
/// the shares of the day) times the shares outstanding on the report's
/// cover, the first count dated within 100 days after.
fn market_value(db: &mut Client, ticker: &str, facts: &HashMap<(String, String, NaiveDate, String), f64>, end: NaiveDate) -> Result<Option<f64>, Box<dyn Error>> {
    let shares = facts.iter()
        .filter(|(k, _)| k.0 == "SharesOutstanding" && k.1 == "pit" && k.2 >= end && (k.2 - end).num_days() <= 100)
        .min_by_key(|(k, _)| k.2).map(|(_, v)| *v);
    let Some(shares) = shares else { return Ok(None) };
    let close = db.query_opt("SELECT close FROM price WHERE upper(ticker) = $1 AND date <= $2 AND date > $2 - 7 ORDER BY date DESC LIMIT 1", &[&ticker, &end])?
        .map(|r| r.get::<_, f64>(0));
    Ok(close.map(|c| c * shares))
}

/// Least squares through the origin, y = ω·x: ω, its standard error, n.
fn fit(pairs: &[(f64, f64)]) -> Option<(f64, f64, usize)> {
    let n = pairs.len();
    if n < 3 { return None; }
    let sxx: f64 = pairs.iter().map(|p| p.0 * p.0).sum();
    if sxx == 0.0 { return None; }
    let omega = pairs.iter().map(|p| p.0 * p.1).sum::<f64>() / sxx;
    let sse: f64 = pairs.iter().map(|p| (p.1 - omega * p.0).powi(2)).sum();
    Some((omega, (sse / (n - 1) as f64 / sxx).sqrt(), n))
}
