use equities::Reader;

/// Usage: dump_filing <ticker> <filing-dir>   (dir holds FilingSummary.xml and R pages)
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let ticker: equities::Ticker = args[1].parse().map_err(|_| "unknown ticker")?;
    let mut r = store::sources::edgar_filing(std::path::Path::new(&args[2]), ticker)?;
    let mut items = r.process_balance_sheet()?;
    items.extend(r.process_income_statement()?);
    items.sort_by_key(|i| (i.item, i.p, i.date));
    for i in items {
        println!("{:<40} {:<4} {}  {:>18.0}", i.item.to_string(), i.p.to_string(), i.date, i.val);
    }
    Ok(())
}
