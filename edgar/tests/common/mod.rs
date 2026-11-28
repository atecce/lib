#![allow(dead_code)] // each test file uses some of these

//! For the known-answer tests of the EDGAR reader (one file per company): a
//! statement as the reader reads it, against the same statement transcribed
//! by hand from the filing.

use equities::item::Reported;

pub fn assert_reported(mut actual: Vec<Reported>, mut expected: Vec<Reported>) {
    // The expected statements list money items only; share counts and
    // per-share amounts are checked separately.
    // Parts of revenue (automotive, financial services) come from segment
    // sections, and total debt sums several lines: not the statement's own
    // lines, so tests check them on their own.
    use equities::item::Item::{AutomotiveDebt, FinancialServicesDebt, TotalDebt};
    actual.retain(|r| r.item.unit() == equities::item::Unit::Money && r.item.part_of().is_none()
        && ![TotalDebt, AutomotiveDebt, FinancialServicesDebt].contains(&r.item));
    actual.sort_by_cached_key(|item| (item.date, item.p, item.item));
    // The expected in the same order, whatever order a statement lists them.
    expected.sort_by_cached_key(|item| (item.date, item.p, item.item));

    assert_eq!(actual.len(), expected.len(), "\nactual:\n{:#?}\n\nexpected:\n{:#?}\n", actual, expected);

    let zipped = actual.into_iter().zip(expected);
    println!("{:#?}", zipped);

    for (actual, expected) in zipped {
        assert!(
            actual == expected,
            "\nactual:\n{:#?}\n\nexpected:\n{:#?}\n",
            actual, expected,
        )
    }
}

/// A test's filing from its folder in the repository (tsla/filings/…):
/// the reader with its files. Tests read fixtures; the library doesn't.
pub fn filing_dir(dir: &std::path::Path, ticker: equities::Ticker) -> Result<edgar::filing::Reader, Box<dyn std::error::Error>> {
    let summary = std::fs::read_to_string(dir.join("FilingSummary.xml"))?;
    let folder = dir.to_path_buf();
    let pages: edgar::filing::Pages = Box::new(move |file| Ok(std::fs::read_to_string(folder.join(file))?));
    Ok(edgar::filing::reader(dir.display().to_string(), &summary, ticker, pages))
}

/// A test's workbook from the repository (nvda/filings/…).
pub fn workbook(path: &std::path::Path, ticker: equities::Ticker) -> Result<edgar::xlsx::Reader, Box<dyn std::error::Error>> {
    edgar::xlsx::reader(std::fs::read(path)?, ticker)
}
