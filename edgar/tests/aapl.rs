//! Apple's 10-Q for the second quarter of fiscal 2026 (filings/aapl/000032019326000013: its summary,
//! balance sheet and income statement), read by the EDGAR reader, against
//! its statements as transcribed in equities::aapl: where both name an item,
//! they agree.

mod common;

use equities::aapl::{Q2_2026_BALANCE_SHEET, Q2_2026_INCOME_STATEMENT};
use equities::item::Reported;
use equities::Ticker;

#[test]
fn the_reader_agrees() {
    let path = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/filings/aapl/000032019326000013"));
    let mut r = common::filing_dir(path, Ticker::AAPL).unwrap();
    let read: Vec<Reported> = [
        equities::Reader::process_balance_sheet(&mut r).unwrap(),
        equities::Reader::process_income_statement(&mut r).unwrap(),
    ].concat();
    let mine = Q2_2026_BALANCE_SHEET.reported().into_iter().chain(Q2_2026_INCOME_STATEMENT.reported());
    let mut agreed = 0;
    for m in mine {
        if let Some(r) = read.iter().find(|r| r.item == m.item && r.date == m.date && r.p == m.p) {
            assert_eq!(r.val, m.val, "{:?}", m.item);
            agreed += 1;
        }
    }
    assert!(agreed >= 30, "only {} items agreed", agreed);
}
