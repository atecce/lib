//! Toyota Motor (TM in New York, 7203.T in Tokyo): what it files that no
//! other company does. Its 20-F (EDGAR) uses its own elements for its leased
//! vehicles and its lending arm (金融事業, kin'yū jigyō, financial services);
//! its yūka shōken hōkokusho (有価証券報告書, EDINET, filer E02144) for its
//! revenue and its parts.

use crate::item::Item;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_own_elements() {
        assert_eq!(item("tm_VehiclesAndEquipmentOnOperatingLeases"), Some(Item::OperatingLeaseVehiclesNet));
        assert_eq!(item("E02144:TotalNetRevenuesIFRS"), Some(Item::Revenue));
        assert_eq!(item("us-gaap_Revenues"), None);
    }
}
