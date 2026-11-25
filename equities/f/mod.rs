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

#[cfg(test)]
mod tests {
    #[test]
    fn its_own_elements() {
        assert_eq!(super::item("f_NetInvestmentInOperatingLeasesNoncurrent"), Some(crate::item::Item::OperatingLeaseVehiclesNet));
    }
}
