//! What one company files that no other does: its own XBRL elements (an EDGAR
//! filer's extension tags, `tm_…`, an EDINET filer's, `E02144:…`), each read
//! by a stateless function in the company's module. The shared lists in
//! item.rs hold only the elements anyone may use.

use crate::item::{Item, Reported};
use crate::Ticker;

/// The item a company's own element reports, as its module says.
pub fn item(ticker: Ticker, tag: &str) -> Option<Item> {
    match ticker.as_str() {
        "TM" | "7203.T" => crate::tm::item(tag),
        "F" => crate::f::item(tag),
        "GM" => crate::gm::item(tag),
        "NVDA" => crate::nvda::item(tag),
        "SMFG" => crate::smfg::item(tag),
        "TSLA" => crate::tsla::item(tag),
        _ => None,
    }
}

/// The item a company's own name for a section or row reports (Ford's
/// "Ford Credit", GM's "GM Financial", Toyota's "Financing operations"),
/// as its module says.
pub fn label(ticker: Ticker, label: &str) -> Option<Item> {
    match ticker.as_str() {
        "F" => crate::f::label(label),
        "GM" => crate::gm::label(label),
        "TM" => crate::tm::label(label),
        _ => None,
    }
}

/// The item a label reports in `ticker`'s filings: the company's own names
/// first, then the shared labels.
pub fn label_item(ticker: Ticker, l: &str) -> Option<Item> {
    label(ticker, l).or_else(|| l.parse::<Item>().ok())
}

/// The item an element reports in `ticker`'s filings, and its rank among
/// that item's elements (as `Item::from_xbrl_tag_ranked`): the company's own
/// elements first, and first in rank, since they are the company's own word
/// on its own statements (Toyota's total revenues over EDINET's summary of
/// results); then the shared lists.
pub fn item_ranked(ticker: Ticker, tag: &str) -> Option<(Item, usize)> {
    match item(ticker, tag) {
        Some(item) => Some((item, 0)),
        None => Item::from_xbrl_tag_ranked(tag),
    }
}

/// A company's corrections to what its filings say, where a filing is
/// wrong (Toyota's 2019 20-F, a digit short of its share count): each in
/// its module, applied to the items as read, before anything is derived.
pub fn correct(ticker: Ticker, items: &mut [Reported]) {
    match ticker.as_str() {
        "TM" => crate::tm::correct(items),
        _ => {}
    }
}

/// Another listing of the same company whose reports a listing's valuation
/// also reads: TSM's, TSMC's Taipei listing (its quarters, in TWD).
pub fn valued_with(ticker: Ticker) -> Option<Ticker> {
    match ticker.as_str() {
        "TSM" => Some(crate::tsm::TAIPEI),
        _ => None,
    }
}

/// Figures a company's valuation needs beyond what it reported in the form
/// the generic valuation reads (crate::valuation::market), from its module;
/// put ahead of the reported items, so on a day both have, these are read.
/// Not facts: for valuation only.
///
/// `series` holds the FRED series the company's valuation needs
/// (`valuation_series`), by ID, as (date, value), oldest first; the caller
/// reads them. One missing: valued without it.
pub fn valuation_items(ticker: Ticker, items: &[Reported], series: &SeriesById) -> Vec<Reported> {
    match ticker.as_str() {
        "TSM" => series.get("DEXTAUS").map(|rates| crate::tsm::in_dollars(items, rates.points())).unwrap_or_default(),
        _ => Vec::new(),
    }
}

/// FRED series by ID (DEXTAUS, GS10).
pub type SeriesById = std::collections::HashMap<&'static str, crate::series::Series>;

/// The FRED series a company's valuation needs (for `valuation_items`):
/// TSMC's, the Taiwan dollar's rate (DEXTAUS, TWD per USD).
pub fn valuation_series(ticker: Ticker) -> &'static [&'static str] {
    match ticker.as_str() {
        "TSM" => &["DEXTAUS"],
        _ => &[],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_companys_own_names_are_its_own() {
        assert_eq!(label_item(Ticker::new("F"), "Ford Credit"), Some(Item::FinancialServicesRevenue));
        assert_eq!(label_item(Ticker::new("GM"), "GM Financial"), Some(Item::FinancialServicesRevenue));
        assert_eq!(label_item(Ticker::new("TM"), "Sales of products"), Some(Item::AutomotiveRevenue));
        // Another company's name means nothing elsewhere.
        assert_eq!(label_item(Ticker::new("GM"), "Ford Credit"), None);
        assert_eq!(label_item(Ticker::new("AAPL"), "Sales of products"), None);
        // The shared names read the same for everyone.
        assert_eq!(label_item(Ticker::new("GM"), "Automotive"), Some(Item::AutomotiveRevenue));
    }

    #[test]
    fn a_companys_own_elements_are_its_own() {
        let tag = "nvda_PurchasesOfPropertyAndEquipmentAndIntangibleAssets";
        assert_eq!(item(Ticker::NVDA, tag), Some(Item::PurchasesRelatedToPropertyAndEquipmentAndIntangibleAssets));
        // Not another company's, and not in the shared lists.
        assert_eq!(item(Ticker::TSLA, tag), None);
        assert_eq!(Item::from_xbrl_tag(tag), None);
        // First in rank: the company's word on its own statement.
        assert_eq!(item_ranked(Ticker::NVDA, tag).map(|r| r.1), Some(0));
        // Shared elements read the same for everyone.
        assert_eq!(item_ranked(Ticker::NVDA, "us-gaap_Revenues").map(|r| r.0), Some(Item::Revenue));
    }
}
