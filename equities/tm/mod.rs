//! Toyota Motor (TM in New York, 7203.T in Tokyo): what it files that no
//! other company does. Its 20-F (EDGAR) uses its own elements for its leased
//! vehicles and its lending arm (金融事業, kin'yū jigyō, financial services);
//! its yūka shōken hōkokusho (有価証券報告書, EDINET, filer E02144) for its
//! revenue and its parts.

use crate::item::{Item, Reported};

/// The item one of Toyota's own elements reports.
pub fn item(tag: &str) -> Option<Item> {
    Some(match tag {
        "tm_VehiclesAndEquipmentOnOperatingLeases" => Item::OperatingLeaseVehiclesNet,
        "tm_CurrentReceivablesRelatedToFinancialServices" => Item::FinanceReceivablesCurrent,
        "tm_NoncurrentReceivablesRelatedToFinancialServices" => Item::FinanceReceivablesNonCurrent,
        // 売上収益 (uriage shūeki), total revenues, and its parts.
        "E02144:TotalNetRevenuesIFRS" => Item::Revenue,
        "E02144:SalesOfProductsIFRS" => Item::AutomotiveRevenue,
        "E02144:FinancingOperationsIFRS" => Item::FinancialServicesRevenue,
        _ => return None,
    })
}

/// The 20-F for fiscal 2019 (0001193125-19-178062) gives 283,243,167 shares
/// outstanding on its cover: a digit short. The years around it have about
/// 2.8 billion (2,909,923,992 in 2018, 2,766,152,532 in 2020), and the issued
/// shares less treasury stock come to about 2.83 billion, so the market cap
/// fell tenfold for the year that count was in effect. Read as ten times
/// it: within a thousand shares, which the filing doesn't say.
pub fn correct(items: &mut [Reported]) {
    for r in items.iter_mut() {
        if r.item == Item::SharesOutstanding && r.val == 283_243_167.0
            && r.filing.as_ref().is_some_and(|f| f.accession == "0001193125-19-178062") {
            r.val *= 10.0;
        }
    }
}

/// What Toyota calls its own sections and rows (its lending arm, its
/// vehicles), which the shared labels don't name: the item each reports.
pub fn label(label: &str) -> Option<Item> {
    match label {
        "Sales of products" => Some(Item::AutomotiveRevenue),
        "Financing operations" => Some(Item::FinancialServicesRevenue),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_2019_cover_count_is_read_with_its_missing_digit() {
        use crate::item::{Filed, Source};
        let filed = |accession: &str| Some(Filed { accession: accession.into(), filed: "2019-06-21".parse().unwrap(),
            source: Source::Edgar, form: "20-F".into(), period_end: Some("2019-03-31".parse().unwrap()) });
        let count = |accession| Reported { ticker: crate::Ticker::new("TM"), date: "2019-03-31".parse().unwrap(),
            p: crate::Period::PointInTime, item: Item::SharesOutstanding, currency: crate::Currency::USD,
            val: 283_243_167.0, filing: filed(accession), derived: false };
        let mut items = vec![count("0001193125-19-178062"), count("another")];
        super::correct(&mut items);
        assert_eq!(items[0].val, 2_832_431_670.0);
        // Only that filing's.
        assert_eq!(items[1].val, 283_243_167.0);
    }
    use super::*;

    #[test]
    fn its_own_elements() {
        assert_eq!(item("tm_VehiclesAndEquipmentOnOperatingLeases"), Some(Item::OperatingLeaseVehiclesNet));
        assert_eq!(item("E02144:TotalNetRevenuesIFRS"), Some(Item::Revenue));
        assert_eq!(item("us-gaap_Revenues"), None);
    }
}

/// The 20-F for fiscal 2026 (filings/000119312526264811: its summary,
/// statement of financial position and income statement), read by the
/// EDGAR reader, against figures read off the statements by hand, in yen:
/// the totals, and the lines Toyota tags with its own elements (`item`).
#[cfg(test)]
mod filings {
    use crate::item::Item;
    use crate::{Currency, Period, Ticker};
    use chrono::NaiveDate;

    const M: f64 = 1e6;

    #[test]
    fn the_fiscal_2026_20f() {
        let dir = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tm/filings/000119312526264811"));
        let mut r = crate::testing::filing_dir(dir, Ticker::new("TM")).unwrap();
        let read = [crate::Reader::process_balance_sheet(&mut r).unwrap(), crate::Reader::process_income_statement(&mut r).unwrap()].concat();
        let year_end = NaiveDate::from_ymd_opt(2026, 3, 31).unwrap();
        let get = |item: Item, p: Period| read.iter()
            .find(|r| r.item == item && r.p == p && r.date == year_end && r.currency == Currency::JPY).map(|r| r.val);
        assert_eq!(get(Item::Revenue, Period::TwelveMonths), Some(50_684_952.0 * M));
        assert_eq!(get(Item::OperatingIncome, Period::TwelveMonths), Some(3_766_216.0 * M));
        assert_eq!(get(Item::NetIncome, Period::TwelveMonths), Some(3_985_761.0 * M));
        assert_eq!(get(Item::TotalAssets, Period::PointInTime), Some(105_522_331.0 * M));
        assert_eq!(get(Item::TotalLiabilities, Period::PointInTime), Some(64_502_263.0 * M));
        // Its own elements: receivables of its lending arm, and its leases.
        assert_eq!(get(Item::FinanceReceivablesCurrent, Period::PointInTime), Some(13_478_474.0 * M));
        assert_eq!(get(Item::FinanceReceivablesNonCurrent, Period::PointInTime), Some(25_488_182.0 * M));
        assert_eq!(get(Item::OperatingLeaseVehiclesNet, Period::PointInTime), Some(9_705_647.0 * M));
    }
}
