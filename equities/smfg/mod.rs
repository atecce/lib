//! Sumitomo Mitsui Financial Group (SMFG; 三井住友フィナンシャルグループ,
//! Mitsui Sumitomo Finansharu Gurūpu): a bank reporting under IFRS in its
//! 20-F, whose own elements the shared lists don't know.

use crate::item::Item;

/// The item SMFG's own XBRL element reports.
///
/// Its income statement's total, "Total operating income" (net interest,
/// fees and commissions, trading and investment income, before expenses),
/// is its own element: a bank's revenue.
pub fn item(tag: &str) -> Option<Item> {
    match tag {
        "smfg_OperatingIncomeLossBeforeOperatingExpenses" => Some(Item::Revenue),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_operating_income_is_revenue() {
        assert_eq!(item("smfg_OperatingIncomeLossBeforeOperatingExpenses"), Some(Item::Revenue));
        // Before impairment charges, after expenses: not revenue.
        assert_eq!(item("smfg_OperatingIncomeLossBeforeImpairmentCharges"), None);
    }
}
