//! Ford Motor: what it files that no other company does. Its vehicles on
//! operating leases, mostly Ford Credit's, carry its own element.

use crate::item::Item;

/// The item one of Ford's own elements reports.
pub fn item(tag: &str) -> Option<Item> {
    match tag {
        "f_NetInvestmentInOperatingLeasesNoncurrent" => Some(Item::OperatingLeaseVehiclesNet),
        _ => None,
    }
}

/// What Ford calls its own sections and rows (its lending arm, its
/// vehicles), which the shared labels don't name: the item each reports.
pub fn label(label: &str) -> Option<Item> {
    match label {
        "Company excluding Ford Credit" => Some(Item::AutomotiveRevenue),
        "Ford Credit" => Some(Item::FinancialServicesRevenue),
        "Ford Credit finance receivables, net" => Some(Item::FinanceReceivablesCurrent),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn its_own_elements() {
        assert_eq!(super::item("f_NetInvestmentInOperatingLeasesNoncurrent"), Some(crate::item::Item::OperatingLeaseVehiclesNet));
    }
}

/// The 2025 10-K (filings/000003799626000015: its summary, income statement
/// and balance sheet), read by the EDGAR reader, against figures read off
/// the statements by hand: the totals, and the debt of each of Ford's two
/// businesses, from sections named by Ford's own labels.
#[cfg(test)]
mod filings {
    use crate::item::{Item, Reported};
    use crate::{Period, Ticker};
    use chrono::NaiveDate;

    const M: f64 = 1e6;

    fn read() -> Vec<Reported> {
        let dir = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/f/filings/000003799626000015"));
        let mut r = crate::testing::filing_dir(dir, Ticker::new("F")).unwrap();
        [crate::Reader::process_balance_sheet(&mut r).unwrap(), crate::Reader::process_income_statement(&mut r).unwrap()].concat()
    }

    #[test]
    fn the_2025_10k() {
        let read = read();
        let year_end = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();
        let get = |item: Item, p: Period| read.iter().find(|r| r.item == item && r.p == p && r.date == year_end).map(|r| r.val);
        assert_eq!(get(Item::Revenue, Period::TwelveMonths), Some(187_267.0 * M));
        assert_eq!(get(Item::CostOfRevenue, Period::TwelveMonths), Some(174_466.0 * M));
        assert_eq!(get(Item::TotalAssets, Period::PointInTime), Some(289_160.0 * M));
        assert_eq!(get(Item::TotalLiabilities, Period::PointInTime), Some(253_180.0 * M));
        // Its two businesses' debt, current and long-term, from the sections
        // "Company excluding Ford Credit" and "Ford Credit" (f::label).
        assert_eq!(get(Item::AutomotiveDebt, Period::PointInTime), Some((5_550.0 + 16_369.0) * M));
        assert_eq!(get(Item::FinancialServicesDebt, Period::PointInTime), Some((51_752.0 + 89_665.0) * M));
        // Revenue by business, from the income statement's sections, adding
        // up to the total.
        assert_eq!(get(Item::AutomotiveRevenue, Period::TwelveMonths), Some(173_996.0 * M));
        assert_eq!(get(Item::FinancialServicesRevenue, Period::TwelveMonths), Some(13_271.0 * M));
        // Net loss: the parent's ("attributable to Ford Motor Company",
        // tagged available to common stockholders, Ford having no
        // NetIncomeLoss), and the consolidated figure; and per share.
        assert_eq!(get(Item::NetIncome, Period::TwelveMonths), Some(-8_182.0 * M));
        assert_eq!(get(Item::ProfitLoss, Period::TwelveMonths), Some(-8_162.0 * M));
        assert_eq!(get(Item::EarningsPerShareDiluted, Period::TwelveMonths), Some(-2.06));
    }
}
