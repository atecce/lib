//! Each issuer's industry, its SIC code, from EDGAR's own listing of filers:
//!
//!     universe | store-sic
//!
//! reads `universe`'s lines (CIK, name, tickers, SIC, first and last filing,
//! tab separated) and sets `issuer.sic` for the CIKs in the database.
//! Applies the migrations first, as `lib --postgres` does.

use std::error::Error;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn Error>> {
    let mut codes = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let line = line?;
        let fields: Vec<&str> = line.split('\t').collect();
        let (Some(cik), Some(sic)) = (fields.first(), fields.get(3)) else { continue };
        let (Ok(cik), false) = (cik.trim_start_matches("CIK").parse::<i64>(), sic.is_empty()) else { continue };
        codes.push((cik, sic.to_string()));
    }
    let mut store = store::Store::connect(&store::default_url())?;
    for m in store.migrate()? {
        eprintln!("applied {}", m);
    }
    let changed = store.set_sic(&codes)?;
    eprintln!("{} codes read, {} issuers changed", codes.len(), changed);
    Ok(())
}
