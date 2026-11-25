//! The natural rate of interest, r*, from the New York Fed: the
//! Holston-Laubach-Williams estimates (one-sided, quarterly), with trend
//! growth g, the other determinants z, and the output gap, for the US, Canada
//! and the euro area, into Postgres beside FRED's series (fred_series,
//! fred_observation), so the API's Macro serves them too:
//!
//!     store-rstar
//!
//! IDs: HLW_RSTAR_US, HLW_G_US, HLW_Z_US, HLW_GAP_US, and _CA, _EA. The
//! spreadsheet is replaced each quarter, past estimates revised with it.

use std::error::Error;
use std::io::Cursor;

use calamine::{open_workbook_from_rs, Data, Reader, Xlsx};
use chrono::{Duration, NaiveDate};

const URL: &str = "https://www.newyorkfed.org/medialibrary/media/research/economists/williams/data/Holston_Laubach_Williams_current_estimates.xlsx";
const SHEET: &str = "HLW Estimates";

/// The sheet's blocks (first column, 0 = A) and what each holds.
const BLOCKS: [(usize, &str, &str); 4] = [
    (2, "G", "trend growth (g), annualized"),
    (6, "Z", "other determinants of r* (z)"),
    (10, "RSTAR", "natural rate of interest (r*)"),
    (14, "GAP", "output gap"),
];
/// Within a block, the economies, in order.
const ECONOMIES: [(&str, &str); 3] = [("US", "US"), ("CA", "Canada"), ("EA", "euro area")];

/// An Excel serial day (1900 system) as a date.
fn excel_day(serial: f64) -> Option<NaiveDate> {
    NaiveDate::from_ymd_opt(1899, 12, 30)?.checked_add_signed(Duration::days(serial as i64))
}

fn main() -> Result<(), Box<dyn Error>> {
    let bytes = reqwest::blocking::Client::builder().user_agent("lib-fetch").build()?
        .get(URL).send()?.error_for_status()?.bytes()?;
    let mut book: Xlsx<_> = open_workbook_from_rs(Cursor::new(bytes.to_vec()))?;
    let range = book.worksheet_range(SHEET)?;

    // In the sheet's order: block by block, economy by economy, so the i-th
    // series is column BLOCKS[i / 3].0 + i % 3. Each with its title.
    let mut series: Vec<(String, fred::Series)> = Vec::new();
    for (_, id, what) in BLOCKS {
        for (code, name) in ECONOMIES {
            series.push((format!("{} ({}), Holston-Laubach-Williams, New York Fed, quarterly", what, name),
                         fred::Series { id: format!("HLW_{}_{}", id, code), observations: Vec::new() }));
        }
    }
    for row in range.rows() {
        let day = match row.first() {
            Some(Data::Float(f)) => excel_day(*f),
            Some(Data::Int(i)) => excel_day(*i as f64),
            Some(Data::DateTime(d)) => excel_day(d.as_f64()),
            _ => None,
        };
        let Some(day) = day else { continue };
        for (i, (_, s)) in series.iter_mut().enumerate() {
            let col = BLOCKS[i / 3].0 + i % 3;
            let value = match row.get(col) {
                Some(Data::Float(f)) => Some(*f),
                Some(Data::Int(n)) => Some(*n as f64),
                _ => None,
            };
            s.observations.push(fred::Observation { date: day, value });
        }
    }

    let mut store = store::Store::connect(&store::default_url())?;
    for m in store.migrate()? {
        eprintln!("applied {}", m);
    }
    for (title, s) in &series {
        let last = s.observations.iter().rev().find(|o| o.value.is_some());
        let n = store.write_fred(s, Some(title))?;
        eprintln!("{}: {} quarters, latest {} = {}, {} written", s.id, s.observations.len(),
                  last.map(|o| o.date.to_string()).unwrap_or_default(),
                  last.and_then(|o| o.value).map(|v| format!("{:.3}", v)).unwrap_or_default(), n);
    }
    Ok(())
}
