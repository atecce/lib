//! The 2025 10-K (filings/000146785826000013: its summary, income statement
//! and balance sheet), read by the EDGAR reader, against figures read off
//! the statements by hand: the totals, and revenue and debt for each of
//! GM's two businesses, from sections named "Automotive" and "GM Financial".

mod common;

use equities::item::Item;
use equities::{Period, Ticker};
use chrono::NaiveDate;

const M: f64 = 1e6;

#[test]
fn the_2025_10k() {
    let dir = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/filings/gm/000146785826000013"));
    let mut r = crate::common::filing_dir(dir, Ticker::new("GM")).unwrap();
    let read = [equities::Reader::process_balance_sheet(&mut r).unwrap(), equities::Reader::process_income_statement(&mut r).unwrap()].concat();
    let year_end = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();
    let get = |item: Item, p: Period| read.iter().find(|r| r.item == item && r.p == p && r.date == year_end).map(|r| r.val);
    assert_eq!(get(Item::Revenue, Period::TwelveMonths), Some(185_019.0 * M));
    assert_eq!(get(Item::AutomotiveRevenue, Period::TwelveMonths), Some(167_971.0 * M));
    assert_eq!(get(Item::FinancialServicesRevenue, Period::TwelveMonths), Some(17_048.0 * M));
    assert_eq!(get(Item::TotalAssets, Period::PointInTime), Some(281_284.0 * M));
    assert_eq!(get(Item::TotalLiabilities, Period::PointInTime), Some(218_116.0 * M));
    // Each business's debt, short-term and long-term.
    assert_eq!(get(Item::AutomotiveDebt, Period::PointInTime), Some((656.0 + 15_591.0) * M));
    assert_eq!(get(Item::FinancialServicesDebt, Period::PointInTime), Some((35_012.0 + 79_018.0) * M));
    // The parent's ("attributable to stockholders", NetIncomeLoss), and
    // the consolidated figure.
    assert_eq!(get(Item::NetIncome, Period::TwelveMonths), Some(2_697.0 * M));
    assert_eq!(get(Item::ProfitLoss, Period::TwelveMonths), Some(2_780.0 * M));
    assert_eq!(get(Item::EarningsPerShareDiluted, Period::TwelveMonths), Some(3.27));
}
