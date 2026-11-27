//! A data-quality scan of the reported items: series that look broken, ranked.
//!
//!     source ~/.profile && cargo run --release -p equities --example quality [-- TICKER ...]
//!
//! QUALITY_TOP sets how many series to list (default 40; 0 for all).
//!
//! Reads the quarterly flows (3m) and balances (pit) in USD from InfluxDB, and
//! looks at each series (a company's item over time) three ways:
//!
//! - **Level shifts** (augurs changepoint detection, Bayesian online changepoint
//!   detection on the series scaled by its typical size): a stretch where the
//!   series collapses to about zero, or jumps to many times its usual size, as
//!   a misread item or a thousands/millions mix-up would.
//! - **Spikes**: a single value far from its neighbours (a Hampel filter, the
//!   median and median absolute deviation of the three values either side).
//! - **Sign flips**: a series that is almost always one sign (an outflow
//!   reported negative), with a few values of the other.
//! - **Exact zeros** in items a company of this size never has at zero
//!   (revenue, operating or financing cash flow): the mark of a quarter
//!   derived as a year-to-date total less an equal one.
//!
//! Buybacks, dividends and acquisitions are lumpy (nothing some quarters,
//! a large repurchase others), so they skip the checks for zeros.
//!
//! Then, across companies, augurs outlier detection proper: for each item, the
//! companies whose path over the last five years (each scaled by its own size,
//! so only the shape counts) is unlike the rest of the group. Some of these are
//! real (a company that grew tenfold), some are errors.
//!
//! Everything flagged is either something true about a company or a bug in
//! the reader; this lists them, it doesn't decide which.

use std::collections::{BTreeMap, HashMap};
use std::error::Error;

use augurs::changepoint::{Detector, NormalGammaDetector};
use augurs::outlier::{MADDetector, OutlierDetector};

const ITEMS: &[&str] = &[
    "TotalAssets", "TotalLiabilities", "StockholdersEquity", "TotalDebt", "CashAndCashEquivalents",
    "AccountsReceivableNet", "Inventories", "PropertyAndEquipmentNet", "Goodwill",
    "Revenue", "OperatingIncome", "NetIncome", "PaymentsRelatedToRepurchasesOfCommonStock",
    "DividendsPaid", "NetCashProvidedByOperatingActivities", "NetCashUsedInInvestingActivities",
    "NetCashUsedInFinancingActivities", "PurchasesRelatedToPropertyAndEquipmentAndIntangibleAssets",
    "DepreciationAndAmoritization",
];

/// Items that are often zero for a quarter, legitimately.
const LUMPY: &[&str] = &["PaymentsRelatedToRepurchasesOfCommonStock", "DividendsPaid", "TotalDebt",
    "Inventories", "Goodwill"];

/// A company's item over time.
struct Series {
    ticker: String,
    item: String,
    period: String,
    dates: Vec<String>,
    values: Vec<f64>,
}

impl Series {
    fn name(&self) -> String {
        format!("{} {} {}", self.ticker.to_uppercase(), self.item, self.period)
    }
}

/// Something odd about a series, with a weight for ranking.
struct Finding {
    weight: f64,
    what: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let only: Vec<String> = std::env::args().skip(1).map(|t| t.to_lowercase()).collect();
    let series = fetch(&only)?;
    eprintln!("{} series", series.len());

    let mut ranked: Vec<(f64, &Series, Vec<Finding>)> = series.iter()
        .filter(|s| s.values.len() >= 8)
        .map(|s| {
            let mut found = level_shifts(s);
            found.extend(spikes(s));
            found.extend(sign_flips(s));
            found.extend(zeros(s));
            (found.iter().map(|f| f.weight).sum(), s, found)
        })
        .filter(|(score, _, _)| *score > 0.0)
        .collect();
    ranked.sort_by(|a, b| b.0.total_cmp(&a.0));

    let count = |kind: &str| ranked.iter().filter(|(_, _, f)| f.iter().any(|f| f.what.starts_with(kind))).count();
    println!("# Within series: {} of {} flagged ({} level shifts, {} spikes, {} sign flips, {} with exact zeros)\n",
        ranked.len(), series.len(), count("level"), count("spike"), count("sign"), count("zeros"));
    let top = std::env::var("QUALITY_TOP").ok().and_then(|n| n.parse().ok()).unwrap_or(40);
    let top = if top == 0 { ranked.len() } else { top };
    for (score, s, found) in ranked.iter().take(top) {
        println!("{:>5.1}  {}", score, s.name());
        for f in found.iter().take(4) {
            println!("         {}", f.what);
        }
        if found.len() > 4 {
            println!("         … {} more", found.len() - 4);
        }
    }

    println!("\n# Across companies: paths unlike the group's (last 20 quarters)\n");
    for item in ITEMS {
        for period in ["3m", "pit"] {
            let group: Vec<&Series> = series.iter().filter(|s| s.item == *item && s.period == period).collect();
            if let Some(out) = crowd_outliers(&group)? {
                println!("{} {}: {}", item, period, out);
            }
        }
    }
    Ok(())
}

/// The S&P's (or the given tickers') series, in one query.
fn fetch(only: &[String]) -> Result<Vec<Series>, Box<dyn Error>> {
    let token = std::env::var("INFLUXDB3_AUTH_TOKEN").map_err(|_| "INFLUXDB3_AUTH_TOKEN is not set")?;
    let list = |v: &[&str]| v.iter().map(|s| format!("'{}'", s)).collect::<Vec<_>>().join(",");
    let mut sql = format!(
        "SELECT ticker, item, period, time, value FROM reported_item_v15 WHERE currency = 'USD' \
         AND period IN ('3m', 'pit') AND item IN ({})", list(ITEMS));
    if !only.is_empty() {
        let tickers: Vec<&str> = only.iter().map(|s| s.as_str()).collect();
        sql += &format!(" AND ticker IN ({})", list(&tickers));
    }
    let body = reqwest::blocking::Client::builder().timeout(std::time::Duration::from_secs(600)).build()?
        .post("http://localhost:8181/api/v3/query_sql")
        .bearer_auth(token)
        .header("Content-Type", "application/json")
        .body(serde_json::json!({ "db": "financials", "q": sql, "format": "csv" }).to_string())
        .send()?.error_for_status()?.text()?;

    let mut by_key: BTreeMap<(String, String, String), Vec<(String, f64)>> = BTreeMap::new();
    for line in body.lines().skip(1) {
        let f: Vec<&str> = line.split(',').collect();
        let [ticker, item, period, time, value] = f[..] else { continue };
        let Ok(v) = value.parse::<f64>() else { continue };
        by_key.entry((ticker.into(), item.into(), period.into())).or_default().push((time[..10].into(), v));
    }
    Ok(by_key.into_iter().map(|((ticker, item, period), mut points)| {
        points.sort_by(|a, b| a.0.cmp(&b.0));
        let (dates, values) = points.into_iter().unzip();
        Series { ticker, item, period, dates, values }
    }).collect())
}

fn median(v: &mut Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.total_cmp(b));
    let n = v.len();
    if n == 0 { 0.0 } else if n % 2 == 1 { v[n / 2] } else { (v[n / 2 - 1] + v[n / 2]) / 2.0 }
}

/// The series' typical size: the median of its non-zero absolute values.
fn scale(values: &[f64]) -> f64 {
    median(&mut values.iter().filter(|v| **v != 0.0).map(|v| v.abs()).collect())
}

/// Stretches where the series collapses towards zero or jumps far above its
/// usual size, between changepoints found on the scaled series.
fn level_shifts(s: &Series) -> Vec<Finding> {
    let size = scale(&s.values);
    if size == 0.0 {
        return Vec::new();
    }
    let scaled: Vec<f64> = s.values.iter().map(|v| v / size).collect();
    let mut cuts = NormalGammaDetector::default().detect_changepoints(&scaled);
    cuts.retain(|&c| c > 0 && c < scaled.len());
    cuts.insert(0, 0);
    cuts.push(scaled.len());
    cuts.dedup();

    let mut found = Vec::new();
    for w in cuts.windows(2) {
        let (a, b) = (w[0], w[1]);
        // Two quarters or more, and not the whole series.
        if b - a < 2 || b - a == scaled.len() {
            continue;
        }
        let level = median(&mut scaled[a..b].iter().map(|v| v.abs()).collect());
        let span = format!("{} – {}", s.dates[a], s.dates[b - 1]);
        if level < 0.05 && !LUMPY.contains(&s.item.as_str()) {
            found.push(Finding { weight: 3.0 + (b - a) as f64 * 0.1,
                what: format!("level: about zero for {} values, {} (typical size {})", b - a, span, money(size)) });
        } else if level > 20.0 {
            found.push(Finding { weight: 3.0 + level.log10(),
                what: format!("level: {:.0}× the usual size for {} values, {}", level, b - a, span) });
        }
    }
    found
}

/// Single values far from their neighbours: a scale slip or a misread cell.
fn spikes(s: &Series) -> Vec<Finding> {
    let v = &s.values;
    let mut found = Vec::new();
    for i in 0..v.len() {
        let lo = i.saturating_sub(3);
        let hi = (i + 4).min(v.len());
        let mut around: Vec<f64> = (lo..hi).filter(|&j| j != i).map(|j| v[j]).collect();
        if around.len() < 4 {
            continue;
        }
        let med = median(&mut around.clone());
        let mad = 1.4826 * median(&mut around.iter().map(|x| (x - med).abs()).collect());
        // Zeros around it are the zeros check's business.
        if med == 0.0 {
            continue;
        }
        let ratio = (v[i] / med).abs();
        if (v[i] - med).abs() > (6.0 * mad).max(0.5 * med.abs()) && !(0.2..=5.0).contains(&ratio) && v[i] != 0.0 {
            found.push(Finding { weight: 1.0 + ratio.max(1.0 / ratio).log10().min(4.0),
                what: format!("spike: {} on {}, neighbours about {} ({:.1}×)", money(v[i]), s.dates[i], money(med), ratio) });
        }
    }
    found
}

/// A few values of the opposite sign in a series that is otherwise one sign.
fn sign_flips(s: &Series) -> Vec<Finding> {
    // Income can be a loss, and equity negative: only items with one sign.
    let one_signed = ["Revenue", "TotalAssets", "TotalLiabilities", "CashAndCashEquivalents",
        "Inventories", "Goodwill", "PropertyAndEquipmentNet", "AccountsReceivableNet", "TotalDebt",
        "DepreciationAndAmoritization", "PaymentsRelatedToRepurchasesOfCommonStock", "DividendsPaid",
        "PurchasesRelatedToPropertyAndEquipmentAndIntangibleAssets"];
    if !one_signed.contains(&s.item.as_str()) {
        return Vec::new();
    }
    let pos = s.values.iter().filter(|v| **v > 0.0).count();
    let neg = s.values.iter().filter(|v| **v < 0.0).count();
    let (minority, sign) = if pos >= neg { (neg, -1.0) } else { (pos, 1.0) };
    if minority == 0 || (minority as f64) > 0.15 * (pos + neg) as f64 {
        return Vec::new();
    }
    let dates: Vec<&str> = s.values.iter().zip(&s.dates)
        .filter(|(v, _)| v.signum() == sign).map(|(_, d)| d.as_str()).collect();
    vec![Finding { weight: 1.0 + minority as f64 * 0.5,
        what: format!("sign: {} of {} values {}: {}", minority, pos + neg, if sign < 0.0 { "negative" } else { "positive" },
            dates.iter().take(4).cloned().collect::<Vec<_>>().join(", ")) }]
}

/// Exact zeros in an item that is never zero for a real company of its size.
fn zeros(s: &Series) -> Vec<Finding> {
    if LUMPY.contains(&s.item.as_str()) || s.item.contains("Acquisitions") {
        return Vec::new();
    }
    let dates: Vec<&str> = s.values.iter().zip(&s.dates)
        .filter(|(v, _)| **v == 0.0).map(|(_, d)| d.as_str()).collect();
    if dates.len() < 2 || dates.len() * 2 > s.values.len() {
        return Vec::new();
    }
    vec![Finding { weight: 2.0 + dates.len() as f64 * 0.3,
        what: format!("zeros: {} of {} values exactly 0: {}{}", dates.len(), s.values.len(),
            dates.iter().take(6).cloned().collect::<Vec<_>>().join(", "),
            if dates.len() > 6 { ", …" } else { "" }) }]
}

/// augurs outlier detection across a group: the companies with all of the
/// last 20 quarters, each series scaled by its own size, and the MAD detector
/// asks which paths stray from the group's.
fn crowd_outliers(group: &[&Series]) -> Result<Option<String>, Box<dyn Error>> {
    // Calendar quarters, since fiscal quarters end on different days: the
    // 20 before the latest (which most companies haven't reported yet).
    let quarter = |d: &str| format!("{}Q{}", &d[..4], (d[5..7].parse::<u32>().unwrap_or(1) - 1) / 3 + 1);
    let mut all: Vec<String> = group.iter().flat_map(|s| s.dates.iter().map(|d| quarter(d))).collect();
    all.sort();
    all.dedup();
    all.pop();
    let quarters: Vec<String> = all.iter().rev().take(20).rev().cloned().collect();

    let mut names = Vec::new();
    let mut rows: Vec<Vec<f64>> = Vec::new();
    for s in group {
        let by_q: HashMap<String, f64> = s.dates.iter().zip(&s.values).map(|(d, v)| (quarter(d), *v)).collect();
        let row: Option<Vec<f64>> = quarters.iter().map(|q| by_q.get(q).copied()).collect();
        let Some(row) = row else { continue };
        let size = scale(&row);
        if size == 0.0 {
            continue;
        }
        names.push(s.ticker.to_uppercase());
        rows.push(row.iter().map(|v| v / size).collect());
    }
    if rows.len() < 20 {
        return Ok(None);
    }
    let data: Vec<&[f64]> = rows.iter().map(|r| r.as_slice()).collect();
    let detector = MADDetector::with_sensitivity(0.5)?;
    let processed = detector.preprocess(&data)?;
    let out = detector.detect(&processed)?;
    let mut flagged: Vec<String> = out.outlying_series.iter().map(|&i| names[i].clone()).collect();
    flagged.sort();
    if flagged.is_empty() {
        return Ok(None);
    }
    let shown: Vec<String> = flagged.iter().take(30).cloned().collect();
    Ok(Some(format!("{} of {}: {}{}", flagged.len(), rows.len(), shown.join(" "),
        if flagged.len() > 30 { " …" } else { "" })))
}

fn money(v: f64) -> String {
    let a = v.abs();
    let (n, unit) = if a >= 1e9 { (v / 1e9, "B") } else if a >= 1e6 { (v / 1e6, "M") } else if a >= 1e3 { (v / 1e3, "K") } else { (v, "") };
    format!("{:.1}{}", n, unit)
}
