//! Goldman Sachs's residual income per common share, fiscal 2000–2025, from
//! `equities::gs` (the 10-Ks) and its prices (Tiingo, from the May 1999 IPO).
//!
//!     cargo run --release -p store --example gs_history [-- R]     # R defaults to 0.09
//!     cargo run --release -p store --example gs_history -- --premium P
//!
//! With `--premium`, r is the 10-year Treasury yield at the end of the year
//! before (`equities::treasury`, from the FRED cache) plus P.
//!
//! Per year: book value per common share, the price at the fiscal year end,
//! price to book, ROE on opening book (diluted EPS over it), its excess over
//! r, residual income per share x = EPS − r·B₋₁, and the decay the price
//! implies, ω = R(P − B)/(x + P − B) where x > 0 and P > B. Then, by CEO:
//! ROE, its excess, mean price to book, the stock's total return (dividends
//! reinvested) and the persistence of excess ROE (x₊₁ on x, both over the
//! same B₋₁).

use chrono::NaiveDate;
use equities::{gs, treasury};

const ERAS: [(i32, i32, &str); 3] = [
    (2000, 2006, "Paulson"),
    (2007, 2018, "Blankfein"),
    (2019, 2025, "Solomon"),
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let premium: Option<f64> = match args.iter().position(|a| a == "--premium") {
        Some(i) => Some(args.get(i + 1).ok_or("--premium needs a value")?.parse()?),
        None => None,
    };
    let fixed: f64 = if premium.is_some() { 0.0 } else { args.first().map(|s| s.parse()).transpose()?.unwrap_or(0.09) };
    let ten_year = premium.map(|_| fred::Cache::new(fred::Cache::default_dir()).series(treasury::SERIES).map(treasury::TenYear::new)).transpose()?;
    // The fiscal year's r: for the November years, the December before.
    let cost = |year: i32| match (premium, &ten_year) {
        (Some(p), Some(t)) => t.cost_of_equity(year, p).expect("a Treasury yield for every year"),
        _ => fixed,
    };

    let prices = tiingo::Cache::new(tiingo::Cache::default_dir()).prices("GS")?;
    let on_or_before = |d: NaiveDate| prices.iter().filter(|p| p.day().is_some_and(|day| day <= d && day > d - chrono::Duration::days(10))).last();
    let close = |d: NaiveDate| on_or_before(d).map(|p| p.close);
    let adjusted = |d: NaiveDate| on_or_before(d).map(|p| p.adj_close);

    match premium {
        Some(p) => println!("Goldman Sachs per common share, r = 10-year Treasury (prior December) + {:.1}%\n", p * 100.0),
        None => println!("Goldman Sachs per common share, r = {:.0}%\n", fixed * 100.0),
    }
    println!("{:<5} {:>8} {:>8} {:>5} {:>6} {:>7} {:>7} {:>7} {:>7} {:>8}", "year", "book", "price", "P/B", "r", "ROE", "ROE−r", "x", "ω impl", "growth");
    let years = gs::years();
    let mut rows = Vec::new();
    for y in &years {
        let r = cost(y.year);
        let x = y.residual_income(r);
        let roe = y.roe() * 12.0 / y.months as f64;
        let p = close(y.year_end);
        let b = y.closing_book;
        let implied = p.filter(|&p| x > 0.0 && p > b).map(|p| (1.0 + r) * (p - b) / (x + p - b));
        println!("{:<5} {:>8.2} {:>8} {:>5} {:>5.1}% {:>6.1}% {:>+6.1}% {:>7.2} {:>7} {:>8}{}", y.year, b,
            p.map(|p| format!("{:.2}", p)).unwrap_or_default(),
            p.map(|p| format!("{:.2}", p / b)).unwrap_or_default(),
            r * 100.0, roe * 100.0, (roe - r) * 100.0, x,
            implied.map(|o| format!("{:.2}", o)).unwrap_or_default(),
            implied.map(|o| format!("{:+.1}%", (o - 1.0) * 100.0)).unwrap_or_default(),
            if y.months == 13 { "  (13 months)" } else { "" });
        rows.push((y, x, r, p));
    }

    println!("\n{:<22} {:>6} {:>8} {:>8} {:>6} {:>9} {:>13}", "era", "r", "ROE", "ROE−r", "P/B", "stock", "persistence ω");
    for (from, to, label) in ERAS {
        let era: Vec<_> = rows.iter().filter(|r| r.0.year >= from && r.0.year <= to).collect();
        let n = era.len() as f64;
        let roe = era.iter().map(|r| r.0.roe() * 12.0 / r.0.months as f64).sum::<f64>() / n;
        let r = era.iter().map(|r| r.2).sum::<f64>() / n;
        let pb: Vec<f64> = era.iter().filter_map(|r| r.3.map(|p| p / r.0.closing_book)).collect();
        let start = years.iter().find(|y| y.year == from - 1).map(|y| y.year_end).unwrap_or(gs::ANNUAL[0].year_end);
        let end = era.last().unwrap().0.year_end;
        let span = (end - start).num_days() as f64 / 365.25;
        let stock = adjusted(start).zip(adjusted(end)).map(|(a, b)| format!("{:+.1}%", ((b / a).powf(1.0 / span) - 1.0) * 100.0)).unwrap_or_default();
        let pairs: Vec<(f64, f64)> = rows.windows(2).filter(|w| w[0].0.year >= from && w[1].0.year <= to)
            .map(|w| (w[0].1 / w[0].0.opening_book, w[1].1 / w[0].0.opening_book)).collect();
        let omega = pairs.iter().map(|p| p.0 * p.1).sum::<f64>() / pairs.iter().map(|p| p.0 * p.0).sum::<f64>();
        println!("{:<22} {:>5.1}% {:>7.1}% {:>+7.1}% {:>6.2} {:>9} {:>13.2}", format!("{}–{} {}", from, to, label),
            r * 100.0, roe * 100.0, (roe - r) * 100.0, pb.iter().sum::<f64>() / pb.len().max(1) as f64, stock, omega);
    }
    // Whole span, IPO to the last year end.
    let (ipo, last) = (gs::IPO.0, years.last().unwrap().year_end);
    if let (Some(a), Some(b)) = (prices.iter().find(|p| p.day().is_some_and(|d| d >= ipo)), adjusted(last)) {
        let span = (last - ipo).num_days() as f64 / 365.25;
        println!("\nIPO ({}, first close ${:.2}) to {}: {:.1}× with dividends, {:+.1}% a year",
            a.day().unwrap(), a.close, last, b / a.adj_close, ((b / a.adj_close).powf(1.0 / span) - 1.0) * 100.0);
    }
    Ok(())
}
