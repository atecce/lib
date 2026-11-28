//! Goldman Sachs's 2025 10-K (filings/gs/000088698226000091: its summary and
//! statement pages), read by the EDGAR reader, against its statements as
//! transcribed in equities::gs.

mod common;

use chrono::NaiveDate;
use equities::gs::{BALANCE_SHEET_2025, INCOME_STATEMENT_2025};
use equities::item::{Item, Reported};
use equities::{Period, Ticker};

const M: f64 = 1e6;

/// Goldman's top line is "Total net revenues", tagged
/// `RevenuesNetOfInterestExpense` (not `Revenues`): a bank's revenues are its
/// interest income net of interest expense, plus its fees and trading. It
/// reads as Revenue (item.rs lists the element last, after the usual totals).
#[test]
fn net_revenues_are_revenue() {
    let row = edgar::filing::row_item(Ticker::GS, Some("us-gaap_RevenuesNetOfInterestExpense"), Some("Total net revenues"));
    assert_eq!(row.map(|(item, _)| item), Some(Item::Revenue));
}

/// Where the reader and the transcribed statements name the same item, they
/// agree, every one of them; and total net revenues for the 10-K's three
/// years read as Revenue.
#[test]
fn the_reader_agrees() {
    let path = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/filings/gs/000088698226000091"));
    let mut r = common::filing_dir(path, Ticker::GS).unwrap();
    let read: Vec<Reported> = [
        equities::Reader::process_balance_sheet(&mut r).unwrap(),
        equities::Reader::process_income_statement(&mut r).unwrap(),
    ].concat();
    let mine = BALANCE_SHEET_2025.reported().into_iter().chain(INCOME_STATEMENT_2025.reported());
    let mut agreed = 0;
    let mut differ = Vec::new();
    for m in mine {
        if let Some(r) = read.iter().find(|r| r.item == m.item && r.date == m.date && r.p == m.p) {
            if r.val == m.val { agreed += 1 } else { differ.push((m.item, r.val, m.val)) }
        }
    }
    assert!(differ.is_empty(), "the reader differs: {:?}", differ);
    // Every item both name, not some: one the reader stops reading fails.
    assert_eq!(agreed, 8, "items agreed");

    let revenue = |year| read.iter()
        .find(|r| r.item == Item::Revenue && r.p == Period::TwelveMonths && r.date == NaiveDate::from_ymd_opt(year, 12, 31).unwrap())
        .map(|r| r.val);
    assert_eq!(revenue(2025), Some(58_283.0 * M));
    assert_eq!(revenue(2024), Some(53_512.0 * M));
    assert_eq!(revenue(2023), Some(46_254.0 * M));
}
