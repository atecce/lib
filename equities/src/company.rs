//! What one company files that no other does: its own XBRL elements (an EDGAR
//! filer's extension tags, `tm_…`, an EDINET filer's, `E02144:…`), each read
//! by a stateless function in the company's module. The shared lists in
//! item.rs hold only the elements anyone may use.

use crate::item::Item;
use crate::Ticker;

/// The item a company's own element reports, as its module says.
pub fn item(ticker: Ticker, tag: &str) -> Option<Item> {
    match ticker.as_str() {
        "TM" | "7203.T" => crate::tm::item(tag),
        "F" => crate::f::item(tag),
        "GM" => crate::gm::item(tag),
        "NVDA" => crate::nvda::item(tag),
        "TSLA" => crate::tsla::item(tag),
        _ => None,
    }
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

#[cfg(test)]
mod tests {
    use super::*;

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
