//! Apple's residual income before XBRL, fiscal 1991–2010, from the 10-K
//! figures in `equities::aapl::BEFORE_XBRL` and the Tiingo price history:
//! the same columns as `backtest`'s persistence example prints from 2011.
//!
//!     cargo run --release -p equities --example aapl_history [-- R]     # R defaults to 0.09

use equities::aapl::BEFORE_XBRL;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r: f64 = std::env::args().nth(1).map(|s| s.parse()).transpose()?.unwrap_or(0.09);
    let prices = tiingo::Cache::new(tiingo::Cache::default_dir()).prices("AAPL")?;
    println!("{:<11} {:>6} {:>9} {:>9} {:>6} {:>7} {:>8}", "year end", "ROE", "x ($bn)", "M ($bn)", "M/B", "ω impl", "growth");
    for y in &BEFORE_XBRL {
        // The close on the year end, or the last trading day before it
        // (unadjusted: in that day's shares, as the cover counts them).
        let close = prices.iter().filter(|p| p.day().is_some_and(|d| d <= y.year_end)).last().map(|p| p.close);
        let m = close.map(|c| c * y.shares / 1e3);
        let x = y.prior_equity.map(|b| y.net_income - r * b);
        let implied = match (m, x) {
            (Some(m), Some(x)) if x > 0.0 && m * 1e3 > y.equity => {
                let (mm, b, xx) = (m * 1e3, y.equity, x);
                Some((1.0 + r) * (mm - b) / (xx + mm - b))
            },
            _ => None,
        };
        println!("{:<11} {:>6} {:>9} {:>9} {:>6} {:>7} {:>8}", y.year_end,
            y.prior_equity.map(|b| format!("{:.0}%", y.net_income / b * 100.0)).unwrap_or_default(),
            x.map(|x| format!("{:.2}", x / 1e3)).unwrap_or_default(),
            m.map(|m| format!("{:.1}", m)).unwrap_or_default(),
            m.map(|m| format!("{:.1}", m * 1e3 / y.equity)).unwrap_or_default(),
            implied.map(|o| format!("{:.2}", o)).unwrap_or_default(),
            implied.map(|o| format!("{:+.1}%", (o - 1.0) * 100.0)).unwrap_or_default());
    }
    Ok(())
}
