//! Berkshire Hathaway's 2025 10-K (filings/brk/000119312526083899: its summary,
//! balance sheet and income statement), read by the EDGAR reader, against
//! its statements as transcribed in equities::brk: where both name an item,
//! they agree.

mod common;

use equities::brk::{BALANCE_SHEET_2025, INCOME_STATEMENT_2025};
use equities::item::Reported;
use equities::Ticker;

#[test]
fn the_reader_agrees() {
    let path = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/filings/brk/000119312526083899"));
    let mut r = common::filing_dir(path, Ticker::BRK).unwrap();
    let read: Vec<Reported> = [
        equities::Reader::process_balance_sheet(&mut r).unwrap(),
        equities::Reader::process_income_statement(&mut r).unwrap(),
    ].concat();
    let mine = BALANCE_SHEET_2025.reported().into_iter().chain(INCOME_STATEMENT_2025.reported());
    let mut agreed = 0;
    for m in mine {
        if let Some(r) = read.iter().find(|r| r.item == m.item && r.date == m.date && r.p == m.p) {
            assert_eq!(r.val, m.val, "{:?}", m.item);
            agreed += 1;
        }
    }
    assert!(agreed >= 6, "only {} items agreed", agreed);
}
