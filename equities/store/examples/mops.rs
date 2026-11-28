//! A company's MOPS reports as read, for checking the reader:
//!
//!     cargo run -p store --example mops -- 2330.TW [ITEM]

use equities::Ticker;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ticker: Ticker = args.first().ok_or("usage: mops TICKER [ITEM]")?.parse().map_err(|_| "bad ticker")?;
    let cache = mops::Cache::new(mops::Cache::default_dir());
    let company = store::sources::mops(&cache, ticker)?;
    for f in &company.failures { eprintln!("{}", f) }
    let (read, failed) = (company.filings.len(), company.failures.len());
    let mut items = Vec::new();
    for mut filing in company.filings { items.extend(filing.read().items) }
    eprintln!("{} reports read, {} failed, {} values", read, failed, items.len());
    for it in items.iter().filter(|it| args.get(1).is_none_or(|i| format!("{:?}", it.item) == *i)) {
        println!("{}\t{}\t{:?}\t{}\t{}", it.date, it.p, it.item, it.val, it.filing.as_ref().map_or("", |f| f.accession.as_str()));
    }
    Ok(())
}
