//! Berkshire Hathaway's residual income per share, 1965–2025, from
//! `equities::brk` (Buffett's scorecard, then the 10-Ks) and Class A prices
//! (Tiingo; before them, the scorecard's market changes scaled to the 2018
//! close).
//!
//!     cargo run --release -p store --example brk_history [-- R]     # R defaults to 0.09
//!     cargo run --release -p store --example brk_history -- --premium P
//!
//! With `--premium`, r moves year by year: the 10-year Treasury yield at the
//! end of the year before (`equities::treasury`, from the FRED cache:
//! `fred-fetch GS10`) plus P.
//! Comprehensive ROE is the growth of book value per share (Berkshire pays
//! no dividend), residual income per share x = B_t − (1 + r)·B_{t−1}, and the
//! implied decay ω = R(P − B)/(x + P − B) where x > 0 and P > B. Then, by era,
//! the averages and the persistence of excess ROE: x₊₁ on x, both over the
//! same B₋₁.

use equities::{brk, treasury};

const ERAS: [(i32, i32, &str); 4] = [
    (1965, 1979, "textile mill to insurer"),
    (1980, 1998, "the great compounding"),
    (1999, 2018, "size as an anchor"),
    (2019, 2025, "operating businesses, buybacks"),
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let premium: Option<f64> = match args.iter().position(|a| a == "--premium") {
        Some(i) => Some(args.get(i + 1).ok_or("--premium needs a value")?.parse()?),
        None => None,
    };
    let fixed: f64 = if premium.is_some() { 0.0 } else { args.first().map(|s| s.parse()).transpose()?.unwrap_or(0.09) };
    let ten_year = premium.map(|_| fred::Cache::new(fred::Cache::default_dir()).series(treasury::SERIES).map(treasury::TenYear::new)).transpose()?;
    let cost = |year: i32| match (premium, &ten_year) {
        (Some(p), Some(t)) => t.cost_of_equity(year, p).expect("a Treasury yield for every year"),
        _ => fixed,
    };
    // Without Class A prices (not yet fetched: `tiingo-fetch --full BRK-A`),
    // the book value columns alone.
    let prices = tiingo::Cache::new(tiingo::Cache::default_dir()).prices("BRK-A").unwrap_or_default();
    let close = |year: i32| prices.iter()
        .filter(|p| p.day().is_some_and(|d| d <= chrono::NaiveDate::from_ymd_opt(year, 12, 31).unwrap()
            && d >= chrono::NaiveDate::from_ymd_opt(year, 12, 20).unwrap()))
        .last().map(|p| p.close);
    let anchor = close(2018);
    let relative: std::collections::HashMap<i32, f64> = brk::market_relative_to_2018().into_iter().collect();
    let price = |year: i32| close(year).or_else(|| Some(relative.get(&year)? * anchor?));

    let book = brk::book_per_share();
    match premium {
        Some(p) => println!("Berkshire Hathaway per Class A share, r = 10-year Treasury (prior December) + {:.1}%\n", p * 100.0),
        None => println!("Berkshire Hathaway per Class A share, r = {:.0}%\n", fixed * 100.0),
    }
    println!("{:<5} {:>10} {:>10} {:>5} {:>6} {:>7} {:>7} {:>10} {:>7} {:>8}", "year", "book", "price", "P/B", "r", "ROE", "ROE−r", "x", "ω impl", "growth");
    let mut rows = Vec::new();
    for w in book.windows(2) {
        let ((_, b0), (year, b1)) = (w[0], w[1]);
        let g = b1 / b0 - 1.0;
        let r = cost(year);
        let x = b1 - (1.0 + r) * b0;
        let p = price(year);
        let implied = p.filter(|&p| x > 0.0 && p > b1).map(|p| (1.0 + r) * (p - b1) / (x + p - b1));
        println!("{:<5} {:>10.0} {:>10} {:>5} {:>5.1}% {:>6.1}% {:>+6.1}% {:>10.0} {:>7} {:>8}", year, b1,
            p.map(|p| format!("{:.0}", p)).unwrap_or_default(),
            p.map(|p| format!("{:.2}", p / b1)).unwrap_or_default(),
            r * 100.0, g * 100.0, (g - r) * 100.0, x,
            implied.map(|o| format!("{:.2}", o)).unwrap_or_default(),
            implied.map(|o| format!("{:+.1}%", (o - 1.0) * 100.0)).unwrap_or_default());
        rows.push((year, b0, b1, x, p, r));
    }

    println!("\n{:<40} {:>7} {:>9} {:>9} {:>9} {:>9} {:>13}", "era", "r", "ROE", "ROE−r", "P/B", "market", "persistence ω");
    for (from, to, label) in ERAS {
        let era: Vec<_> = rows.iter().filter(|r| r.0 >= from && r.0 <= to).collect();
        let n = era.len() as f64;
        let book_growth = (era.last().unwrap().2 / era[0].1).powf(1.0 / n) - 1.0;
        // The era's r, compounded like the book value it is compared with.
        let r = era.iter().map(|row| 1.0 + row.5).product::<f64>().powf(1.0 / n) - 1.0;
        let pb: Vec<f64> = era.iter().filter_map(|r| r.4.map(|p| p / r.2)).collect();
        let first_price = price(from - 1);
        let last_price = era.last().unwrap().4;
        let market = match (first_price, last_price) { (Some(a), Some(b)) => format!("{:+.1}%", ((b / a).powf(1.0 / n) - 1.0) * 100.0), _ => String::new() };
        // Persistence: next year's residual income on this year's, both over this year's opening book.
        let pairs: Vec<(f64, f64)> = rows.windows(2).filter(|w| w[0].0 >= from && w[1].0 <= to)
            .map(|w| (w[0].3 / w[0].1, w[1].3 / w[0].1)).collect();
        let sxx: f64 = pairs.iter().map(|p| p.0 * p.0).sum();
        let omega = pairs.iter().map(|p| p.0 * p.1).sum::<f64>() / sxx;
        let pb = if pb.is_empty() { String::new() } else { format!("{:.2}", pb.iter().sum::<f64>() / pb.len() as f64) };
        println!("{:<40} {:>6.1}% {:>8.1}% {:>+8.1}% {:>9} {:>9} {:>13.2}",
            format!("{}–{} {}", from, to, label), r * 100.0, book_growth * 100.0, (book_growth - r) * 100.0, pb, market, omega);
    }
    Ok(())
}
