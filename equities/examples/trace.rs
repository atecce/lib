//! Which filing reported a value: every reading of an item by a company, with
//! the filing it came from, before any merging or derivation.
//!
//!     cargo run --release -p equities --example trace -- TICKER ITEM [FROM [TO]]
//!
//! e.g. `trace TGT NetCashUsedInFinancingActivities 2019-01-01 2020-03-01`.

use std::error::Error;

use equities::Reader;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [ticker, item, rest @ ..] = &args[..] else {
        return Err("usage: trace TICKER ITEM [FROM [TO]]".into());
    };
    let from = rest.first().map(|s| s.as_str()).unwrap_or("0000");
    let to = rest.get(1).map(|s| s.as_str()).unwrap_or("9999");
    let equity: equities::Ticker = ticker.parse().map_err(|_| "unknown ticker")?;
    let cache = edgar::Cache::new(edgar::Cache::default_dir());
    let cik = cache.cik(equity.sec_symbol())?;
    for f in cache.filings(cik, &edgar::FORMS)? {
        let Ok(dir) = cache.filing(&f) else { continue };
        let Ok(mut r) = equities::filing::new_reader(&dir, equity) else { continue };
        let mut items = Vec::new();
        for read in [r.process_balance_sheet(), r.process_income_statement(), r.process_cash_flow_statement()] {
            if let Ok(mut got) = read { items.append(&mut got) }
        }
        for it in items.iter().filter(|it| format!("{:?}", it.item) == *item) {
            let d = it.date.to_string();
            if d.as_str() >= from && d.as_str() <= to {
                println!("{} {} {} ({}): {} {} {}", f.form, f.report_date, f.filing_date, f.accession, it.date, it.p, it.val);
            }
        }
    }
    Ok(())
}
