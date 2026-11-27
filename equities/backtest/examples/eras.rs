//! A stock's returns over eras of its history (CEOs, say), against a
//! benchmark where the benchmark has prices:
//!
//!     cargo run --release -p backtest --example eras -- TICKER BENCH DATE=LABEL ...
//!     cargo run --release -p backtest --example eras -- AAPL SPY \
//!         1980-12-12="IPO" 1985-09-17="Jobs resigns" ...
//!
//! Each era runs from its date to the next one's (the last to the latest
//! price). For each: the multiple and annualized return on adjusted closes
//! (splits and dividends reinvested), the worst drawdown inside it (peak to
//! trough), and the benchmark's annualized return over the same days.
//! Prices from the Tiingo cache; `tiingo-fetch --full` gets a ticker's whole
//! history.

use std::error::Error;

use backtest::Prices;
use chrono::NaiveDate;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (ticker, bench) = match (args.first(), args.get(1)) {
        (Some(t), Some(b)) => (t.clone(), b.clone()),
        _ => return Err("usage: eras TICKER BENCH DATE=LABEL ...".into()),
    };
    let mut eras: Vec<(NaiveDate, String)> = args[2..].iter().map(|a| {
        let (d, l) = a.split_once('=').ok_or(format!("{}: expected DATE=LABEL", a))?;
        Ok((NaiveDate::parse_from_str(d, "%Y-%m-%d")?, l.to_string()))
    }).collect::<Result<_, Box<dyn Error>>>()?;
    eras.sort();

    let cache = tiingo::Cache::new(tiingo::Cache::default_dir());
    let stock = Prices::load(&cache, &ticker, None, None)?;
    let index = Prices::load(&cache, &bench, None, None).ok();
    let last = *stock.dates.last().ok_or("no prices")?;
    println!("{} from {} to {}; benchmark {}{}\n", ticker, stock.dates[0], last, bench,
        index.as_ref().map(|i| format!(" from {}", i.dates[0])).unwrap_or(" (no prices)".into()));
    println!("{:<12} {:<12} {:<34} {:>9} {:>8} {:>9} {:>8}", "from", "to", "era", "multiple", "a year", "drawdown", bench);

    let mut bounds: Vec<NaiveDate> = eras.iter().map(|e| e.0).collect();
    bounds.push(last);
    for (i, (from, label)) in eras.iter().enumerate() {
        let to = bounds[i + 1];
        let Some((a, b)) = span(&stock, *from, to) else { continue };
        let years = (stock.dates[b] - stock.dates[a]).num_days() as f64 / 365.25;
        let multiple = stock.adjusted[b] / stock.adjusted[a];
        let mut peak = stock.adjusted[a];
        let mut worst: f64 = 0.0;
        for &p in &stock.adjusted[a..=b] {
            peak = peak.max(p);
            worst = worst.min(p / peak - 1.0);
        }
        let versus = index.as_ref().and_then(|ix| {
            let (c, d) = span(ix, stock.dates[a], stock.dates[b])?;
            // Only where the benchmark covers the whole era.
            (ix.dates[c] <= stock.dates[a] + chrono::Duration::days(7))
                .then(|| format!("{:+.0}%", ((ix.adjusted[d] / ix.adjusted[c]).powf(1.0 / years) - 1.0) * 100.0))
        }).unwrap_or_default();
        println!("{:<12} {:<12} {:<34} {:>8.1}× {:>+7.0}% {:>8.0}% {:>8}",
            stock.dates[a], stock.dates[b], label, multiple,
            (multiple.powf(1.0 / years) - 1.0) * 100.0, worst * 100.0, versus);
    }
    Ok(())
}

/// The indices of the first trading day on or after `from` and the last on
/// or before `to`.
fn span(p: &Prices, from: NaiveDate, to: NaiveDate) -> Option<(usize, usize)> {
    let a = p.dates.iter().position(|&d| d >= from)?;
    let b = p.dates.iter().rposition(|&d| d <= to)?;
    (b > a).then_some((a, b))
}
