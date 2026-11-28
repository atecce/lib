//! For the companies' known-answer tests (tsla/filings, nvda/filings): a
//! statement as the reader reads it, against the same statement transcribed
//! by hand from the filing.

use crate::item::Reported;

pub(crate) fn assert_reported(mut actual: Vec<Reported>, mut expected: Vec<Reported>) {
    // The expected statements list money items only; share counts and
    // per-share amounts are checked separately.
    // Parts of revenue (automotive, financial services) come from segment
    // sections, and total debt sums several lines: not the statement's own
    // lines, so tests check them on their own.
    use crate::item::Item::{AutomotiveDebt, FinancialServicesDebt, TotalDebt};
    actual.retain(|r| r.item.unit() == crate::item::Unit::Money && r.item.part_of().is_none()
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
pub(crate) fn filing_dir(dir: &std::path::Path, ticker: crate::Ticker) -> Result<crate::filing::Reader, Box<dyn std::error::Error>> {
    let summary = std::fs::read_to_string(dir.join("FilingSummary.xml"))?;
    let folder = dir.to_path_buf();
    let pages: crate::filing::Pages = Box::new(move |file| Ok(std::fs::read_to_string(folder.join(file))?));
    Ok(crate::filing::reader(dir.display().to_string(), &summary, ticker, pages))
}

/// A test's workbook from the repository (nvda/filings/…).
pub(crate) fn workbook(path: &std::path::Path, ticker: crate::Ticker) -> Result<crate::xlsx::Reader, Box<dyn std::error::Error>> {
    crate::xlsx::reader(std::fs::read(path)?, ticker)
}
