//! The layers of r, by decade, from 1860: the short rate, inflation and the
//! real short rate after the fact; the long government rate and the term
//! spread; the high-grade corporate rate and the Baa–Aaa credit spread; and,
//! where they exist, the market's own split of the 10-year yield into a real
//! yield (TIPS), expected inflation (breakeven) and the term premium.
//!
//!     fred-fetch --list fred/rates.txt
//!     cargo run --release -p fred --example rates
//!
//! Spliced where one series ends and another begins:
//!
//! - short: commercial paper in New York to 1933, the 3-month bill after;
//! - long government: long-term government securities 1925–1952, the
//!   10-year Treasury after (none before 1925 in FRED);
//! - high grade: railroad bonds to 1918, Moody's Aaa after;
//! - prices: the NBER's general price level to 1913, the CPI after.
//!
//! Each decade's figure is the mean of its years' means (a spread, of the
//! years with both rates); inflation is the
//! compound rate between the average price levels of the year before the
//! decade and its last year. Real = nominal − realized inflation (ex post,
//! not what lenders expected).

use std::error::Error;

use fred::{Cache, Series};

const LAST: i32 = 2025;

fn main() -> Result<(), Box<dyn Error>> {
    let cache = Cache::new(Cache::default_dir());
    let load = |id: &str| cache.series(id);
    let paper = load("M13002US35620M156NNBR")?;
    let bill = load("TB3MS")?;
    let ltgov = load("LTGOVTBD")?;
    let gs10 = load("GS10")?;
    let rail = load("M13019USM156NNBR")?;
    let aaa = load("AAA")?;
    let baa = load("BAA")?;
    let nber_prices = load("M04051USM324NNBR")?;
    let cpi = load("CPIAUCNS")?;
    let tips = load("FII10")?;
    let breakeven = load("T10YIEM")?;
    let premium = load("THREEFYTP10")?;
    let fedfunds = load("FEDFUNDS")?;

    let m = |s: &Series, y: i32| s.year_mean(y, 6);
    let short = |y: i32| if y <= 1933 { m(&paper, y) } else { m(&bill, y) };
    let long = |y: i32| if y <= 1952 { m(&ltgov, y) } else { m(&gs10, y) };
    let high = |y: i32| if y <= 1918 { m(&rail, y) } else { m(&aaa, y) };
    let credit = |y: i32| Some(m(&baa, y)? - m(&aaa, y)?);
    // Both price levels are 1913 = 100, so they join at 1913.
    let cpi_1913 = m(&cpi, 1913).ok_or("no CPI for 1913")?;
    let price = |y: i32| if y <= 1913 { m(&nber_prices, y) } else { m(&cpi, y).map(|p| p * 100.0 / cpi_1913) };
    let funds = |y: i32| m(&fedfunds, y);
    let real_10 = |y: i32| m(&tips, y);
    let expected = |y: i32| m(&breakeven, y);
    let term_premium = |y: i32| premium.year_mean(y, 100);

    println!("Percent a year, decade means.\n");
    println!("{:<7} {:>6} {:>6} {:>6} | {:>6} {:>6} | {:>6} {:>6} {:>6} | {:>6} {:>6} {:>6} {:>6}",
        "decade", "short", "infl", "real", "long", "term", "high", "real", "Baa−A", "fedfnd", "TIPS", "brkevn", "termpr");
    let mut from = 1860;
    while from <= LAST {
        let to = (from + 9).min(LAST);
        let infl = match (price(from - 1), price(to)) {
            (Some(a), Some(b)) => Some(((b / a).powf(1.0 / (to - from + 1) as f64) - 1.0) * 100.0),
            _ => None,
        };
        let s = mean(&short, from, to);
        let l = mean(&long, from, to);
        let h = mean(&high, from, to);
        println!("{:<7} {:>6} {:>6} {:>6} | {:>6} {:>6} | {:>6} {:>6} {:>6} | {:>6} {:>6} {:>6} {:>6}",
            format!("{}s", from), cell(s), cell(infl), cell(s.zip(infl).map(|(s, i)| s - i)),
            cell(l), cell(mean(&|y| Some(long(y)? - short(y)?), from, to)),
            cell(h), cell(h.zip(infl).map(|(h, i)| h - i)), cell(mean(&credit, from, to)),
            cell(mean(&funds, from, to)), cell(mean(&real_10, from, to)),
            cell(mean(&expected, from, to)), cell(mean(&term_premium, from, to)));
        from += 10;
    }

    println!("\nWhole spans (yearly means):");
    span("short (paper, then bills)", &short);
    span("long government", &long);
    span("high-grade corporate", &high);
    span("Baa − Aaa", &credit);
    span("10-year TIPS", &real_10);
    span("10-year term premium", &term_premium);
    Ok(())
}

fn mean(f: &dyn Fn(i32) -> Option<f64>, a: i32, b: i32) -> Option<f64> {
    let v: Vec<f64> = (a..=b).filter_map(f).collect();
    (!v.is_empty()).then(|| v.iter().sum::<f64>() / v.len() as f64)
}

fn cell(v: Option<f64>) -> String {
    v.map(|v| format!("{:.1}", v)).unwrap_or_default()
}

fn span(name: &str, f: &dyn Fn(i32) -> Option<f64>) {
    let years: Vec<(i32, f64)> = (1857..=LAST).filter_map(|y| f(y).map(|v| (y, v))).collect();
    let (Some(first), Some(last)) = (years.first(), years.last()) else { return };
    let lo = years.iter().copied().fold(*first, |a, p| if p.1 < a.1 { p } else { a });
    let hi = years.iter().copied().fold(*first, |a, p| if p.1 > a.1 { p } else { a });
    println!("  {:<28} {}–{}  mean {:>5.2}  low {:>5.2} ({})  high {:>5.2} ({})", name, first.0, last.0,
        years.iter().map(|p| p.1).sum::<f64>() / years.len() as f64, lo.1, lo.0, hi.1, hi.0);
}
