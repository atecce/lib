//! The 20-F for fiscal 2026 (filings/000119312526264811: its summary,
//! statement of financial position and income statement), read by the
//! EDGAR reader, against figures read off the statements by hand, in yen:
//! the totals, and the lines Toyota tags with its own elements (`item`).

mod common;

use equities::item::Item;
use equities::{Currency, Period, Ticker};
use chrono::NaiveDate;

const M: f64 = 1e6;

#[test]
fn the_fiscal_2026_20f() {
    let dir = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/filings/tm/000119312526264811"));
    let mut r = crate::common::filing_dir(dir, Ticker::new("TM")).unwrap();
    let read = [equities::Reader::process_balance_sheet(&mut r).unwrap(), equities::Reader::process_income_statement(&mut r).unwrap()].concat();
    let year_end = NaiveDate::from_ymd_opt(2026, 3, 31).unwrap();
    let get = |item: Item, p: Period| read.iter()
        .find(|r| r.item == item && r.p == p && r.date == year_end && r.currency == Currency::JPY).map(|r| r.val);
    assert_eq!(get(Item::Revenue, Period::TwelveMonths), Some(50_684_952.0 * M));
    assert_eq!(get(Item::OperatingIncome, Period::TwelveMonths), Some(3_766_216.0 * M));
    // The parent's ("attributable to Toyota Motor Corporation"), and the
    // consolidated figure.
    assert_eq!(get(Item::NetIncome, Period::TwelveMonths), Some(3_848_098.0 * M));
    assert_eq!(get(Item::ProfitLoss, Period::TwelveMonths), Some(3_985_761.0 * M));
    assert_eq!(get(Item::TotalAssets, Period::PointInTime), Some(105_522_331.0 * M));
    assert_eq!(get(Item::TotalLiabilities, Period::PointInTime), Some(64_502_263.0 * M));
    // Its own elements: receivables of its lending arm, and its leases.
    assert_eq!(get(Item::FinanceReceivablesCurrent, Period::PointInTime), Some(13_478_474.0 * M));
    assert_eq!(get(Item::FinanceReceivablesNonCurrent, Period::PointInTime), Some(25_488_182.0 * M));
    assert_eq!(get(Item::OperatingLeaseVehiclesNet, Period::PointInTime), Some(9_705_647.0 * M));
}
