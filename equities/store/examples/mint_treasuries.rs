//! MINT (PIMCO Enhanced Short Maturity Active ETF) against the whole
//! Treasury curve, from its launch in November 2009:
//!
//!     fred-fetch --list fred/rates.txt && tiingo-fetch --full MINT SPY
//!     cargo run --release -p store --example mint_treasuries
//!
//! 1. By year: MINT's total return (Tiingo's adjusted closes, distributions
//!    reinvested) against rolling 1- and 3-month bills (each day accruing
//!    that day's constant-maturity yield, `y/100/365` a calendar day), and
//!    the yields at the start of the year from 1 month to 30 years.
//! 2. Which part of the curve MINT moves with: its weekly return regressed
//!    on the weekly change in each yield. The slope is minus its duration
//!    against that maturity (percent of price per point of yield); R² says
//!    how much of MINT's week it explains; then over four weeks (without
//!    March–April 2020), where the noise of a week's trading averages out.
//! 3. The 2021–22 turn, month by month: MINT from its peak, the curve, and
//!    SPY from its own.
//! 4. MINT as a signal: each time its total return falls 0.25% below its
//!    six-month high (126 trading days; a new episode once it has recovered
//!    to within a third of that), SPY's return over the next 3, 6 and 12
//!    months and its worst drawdown in the next 12; then SPY's own peaks
//!    before declines of 15% or more, to see which the signal preceded.

use std::collections::BTreeMap;
use std::error::Error;

use chrono::{Datelike, NaiveDate};

const CURVE: [(&str, &str); 10] = [
    ("1m", "DGS1MO"), ("3m", "DGS3MO"), ("6m", "DGS6MO"), ("1y", "DGS1"), ("2y", "DGS2"),
    ("3y", "DGS3"), ("5y", "DGS5"), ("7y", "DGS7"), ("10y", "DGS10"), ("30y", "DGS30"),
];

type Daily = BTreeMap<NaiveDate, f64>;

fn main() -> Result<(), Box<dyn Error>> {
    let fred = fred::Cache::new(fred::Cache::default_dir());
    let curve: Vec<(&str, Daily)> = CURVE.iter().map(|(label, id)| {
        let s = fred.series(id)?;
        Ok((*label, s.observations.iter().filter_map(|o| Some((o.date, o.value?))).collect()))
    }).collect::<Result<_, Box<dyn Error>>>()?;
    let tiingo = tiingo::Cache::new(tiingo::Cache::default_dir());
    let adj = |t: &str| -> Result<Daily, Box<dyn Error>> {
        Ok(tiingo.prices(t)?.iter().filter_map(|p| Some((p.day()?, p.adj_close))).collect())
    };
    let mint = adj("MINT")?;
    let spy = adj("SPY")?;
    let first = *mint.keys().next().ok_or("no MINT prices")?;
    let last = *mint.keys().last().unwrap();
    let at = |m: &Daily, d: NaiveDate| m.range(..=d).next_back().map(|(_, v)| *v);

    // 1. By year.
    let rolled = |m: &Daily, a: NaiveDate, b: NaiveDate| {
        let mut g = 1.0;
        let mut d = a;
        while d < b {
            g *= 1.0 + at(m, d).unwrap_or(0.0) / 100.0 / 365.0;
            d = d.succ_opt().unwrap();
        }
        g - 1.0
    };
    println!("1. MINT's total return against rolling bills, and the curve at the start of each year (%)\n");
    print!("{:<6} {:>6} {:>6} {:>6} {:>7} |", "year", "MINT", "1m", "3m", "MINT−3m");
    for (label, _) in &curve { print!(" {:>5}", label); }
    println!();
    for year in first.year()..=last.year() {
        let a = NaiveDate::from_ymd_opt(year - 1, 12, 31).unwrap().max(first);
        let b = NaiveDate::from_ymd_opt(year, 12, 31).unwrap().min(last);
        let (Some(m0), Some(m1)) = (at(&mint, a), at(&mint, b)) else { continue };
        let ret = (m1 / m0 - 1.0) * 100.0;
        let three = rolled(&curve[1].1, a, b) * 100.0;
        print!("{:<6} {:>+6.2} {:>+6.2} {:>+6.2} {:>+7.2} |", if b < NaiveDate::from_ymd_opt(year, 12, 31).unwrap() || a > NaiveDate::from_ymd_opt(year - 1, 12, 31).unwrap() { format!("{}*", year) } else { year.to_string() },
            ret, rolled(&curve[0].1, a, b) * 100.0, three, ret - three);
        for (_, m) in &curve { print!(" {:>5}", at(m, a).map(|v| format!("{:.2}", v)).unwrap_or_default()); }
        println!();
    }
    let years = (last - first).num_days() as f64 / 365.25;
    let total = at(&mint, last).unwrap() / at(&mint, first).unwrap();
    let bills = 1.0 + rolled(&curve[1].1, first, last);
    println!("{:<6} {:>+6.2} {:>6} {:>+6.2} {:>+7.2}   a year, {} to {}  (* part year)\n", "all",
        (total.powf(1.0 / years) - 1.0) * 100.0, "", (bills.powf(1.0 / years) - 1.0) * 100.0,
        (total.powf(1.0 / years) - bills.powf(1.0 / years)) * 100.0, first, last);

    // 2. Weekly sensitivity to each maturity: Fridays (or the last day before).
    let mut fridays = Vec::new();
    let mut d = first;
    while d <= last {
        if d.weekday() == chrono::Weekday::Fri { fridays.push(d); }
        d = d.succ_opt().unwrap();
    }
    let crisis = |d: NaiveDate| d >= NaiveDate::from_ymd_opt(2020, 2, 21).unwrap() && d <= NaiveDate::from_ymd_opt(2020, 5, 1).unwrap();
    println!("2. MINT's weekly return (%) on the weekly change in each yield (points), {} weeks", fridays.len() - 1);
    println!("   (slope = −duration in years; without March–April 2020, when MINT fell with credit, not rates)\n");
    println!("{:<5} {:>8} {:>6} | {:>8} {:>6} | {:>8} {:>6}", "", "slope", "R²", "ex-2020", "R²", "4 weeks", "R²");
    for (label, m) in &curve {
        let pairs = |skip: bool, step: usize| -> Vec<(f64, f64)> {
            fridays.iter().step_by(step).collect::<Vec<_>>().windows(2).filter(|w| !(skip && crisis(*w[1]))).filter_map(|w| {
                let r = (at(&mint, *w[1])? / at(&mint, *w[0])? - 1.0) * 100.0;
                let dy = at(m, *w[1])? - at(m, *w[0])?;
                Some((dy, r))
            }).collect()
        };
        let (b, r2) = ols(&pairs(false, 1));
        let (b2, r22) = ols(&pairs(true, 1));
        let (b4, r24) = ols(&pairs(true, 4));
        println!("{:<5} {:>8.2} {:>6.2} | {:>8.2} {:>6.2} | {:>8.2} {:>6.2}", label, b, r2, b2, r22, b4, r24);
    }

    // 3. The 2021–22 turn.
    println!("\n3. Month ends, July 2021 – December 2022: MINT and SPY from their peaks, and the curve (%)\n");
    let peak_of = |m: &Daily, to: NaiveDate| m.range(NaiveDate::from_ymd_opt(2021, 1, 1).unwrap()..=to).map(|(_, v)| *v).fold(f64::MIN, f64::max);
    print!("{:<8} {:>6} {:>6} |", "month", "MINT", "SPY");
    for (label, _) in &curve { print!(" {:>5}", label); }
    println!();
    for i in 0..18 {
        let (y, mo) = (2021 + (6 + i) / 12, (6 + i) % 12 + 1);
        let end = NaiveDate::from_ymd_opt(y, mo as u32, 1).unwrap().checked_add_months(chrono::Months::new(1)).unwrap().pred_opt().unwrap();
        let from_peak = |m: &Daily| (at(m, end).unwrap() / peak_of(m, end) - 1.0) * 100.0;
        print!("{:<8} {:>+6.2} {:>+6.1} |", format!("{}-{:02}", y, mo), from_peak(&mint), from_peak(&spy));
        for (_, m) in &curve { print!(" {:>5}", at(m, end).map(|v| format!("{:.2}", v)).unwrap_or_default()); }
        println!();
    }

    // 4. MINT as a signal.
    let days: Vec<NaiveDate> = mint.keys().filter(|d| spy.contains_key(d)).copied().collect();
    let m: Vec<f64> = days.iter().map(|d| mint[d]).collect();
    let s: Vec<f64> = days.iter().map(|d| spy[d]).collect();
    let forward = |i: usize, n: usize| (s[(i + n).min(s.len() - 1)] / s[i] - 1.0) * 100.0;
    let worst = |i: usize, n: usize| {
        let mut peak = s[i];
        let mut w: f64 = 0.0;
        for &v in &s[i..(i + n).min(s.len())] { peak = peak.max(v); w = w.min(v / peak - 1.0); }
        w * 100.0
    };
    println!("\n4. MINT 0.25% below its six-month high (first day of each episode), and SPY after (%)\n");
    println!("{:<12} {:>7} {:>7} {:>7} {:>9}", "date", "3m", "6m", "12m", "worst 12m");
    let (threshold, mut active) = (0.0025, false);
    for i in 126..days.len() {
        let high = m[i - 126..=i].iter().cloned().fold(f64::MIN, f64::max);
        let dd = m[i] / high - 1.0;
        if dd < -threshold && !active {
            active = true;
            println!("{:<12} {:>+7.1} {:>+7.1} {:>+7.1} {:>9.1}{}", days[i], forward(i, 63), forward(i, 126), forward(i, 252), worst(i, 252),
                if i + 252 >= days.len() { "  (less than a year since)" } else { "" });
        }
        if dd > -threshold / 3.0 { active = false; }
    }
    let n = days.len().saturating_sub(252);
    let year: Vec<f64> = (0..n).map(|i| forward(i, 252)).collect();
    println!("{:<12} {:>7} {:>7} {:>+7.1}   every day: {:.0}% of years negative", "all days", "", "",
        year.iter().sum::<f64>() / n as f64, 100.0 * year.iter().filter(|r| **r < 0.0).count() as f64 / n as f64);
    let mut peaks = Vec::new();
    let (mut peak, mut at_peak) = (s[0], days[0]);
    for (i, &v) in s.iter().enumerate() {
        if v > peak { peak = v; at_peak = days[i]; }
        if v / peak - 1.0 < -0.15 && peaks.last() != Some(&at_peak) { peaks.push(at_peak); }
    }
    println!("SPY's peaks before declines of 15% or more: {}", peaks.iter().map(|d| d.to_string()).collect::<Vec<_>>().join(", "));
    Ok(())
}

/// Least squares of y on x with an intercept: (slope, R²).
fn ols(p: &[(f64, f64)]) -> (f64, f64) {
    let n = p.len() as f64;
    let (mx, my) = (p.iter().map(|q| q.0).sum::<f64>() / n, p.iter().map(|q| q.1).sum::<f64>() / n);
    let sxy: f64 = p.iter().map(|q| (q.0 - mx) * (q.1 - my)).sum();
    let sxx: f64 = p.iter().map(|q| (q.0 - mx).powi(2)).sum();
    let syy: f64 = p.iter().map(|q| (q.1 - my).powi(2)).sum();
    (sxy / sxx, sxy * sxy / (sxx * syy))
}
