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
