//! General Motors: what it files that no other company does. GM Financial's
//! vehicles on operating leases carry GM's own element.

use crate::item::Item;

/// The item one of GM's own elements reports.
pub fn item(tag: &str) -> Option<Item> {
    match tag {
        "gm_OperatingLeaseRightofuseAssetNoncurrent" => Some(Item::OperatingLeaseVehiclesNet),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn its_own_elements() {
        assert_eq!(super::item("gm_OperatingLeaseRightofuseAssetNoncurrent"), Some(crate::item::Item::OperatingLeaseVehiclesNet));
    }
}
