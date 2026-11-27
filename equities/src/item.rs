use std::fmt::Display;
use std::str::FromStr;

macro_rules! items {
    (
        pub enum $item:ident {
            $(
                $variant:ident $(=> [$($alias:literal),*])? $(@ [$($tag:literal),+ $(,)?])?
            ),* $(,)?
        }
    ) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, serde::Serialize, uniffi::Enum)]
        pub enum $item {
            $($variant),*
        }

        impl $item {
            pub const ALL: [$item; 79] = [
                $($item::$variant,)*
            ];

            /// Every XBRL element mapped to an item. us-gaap elements are
            /// written without their prefix; IFRS elements ("ifrs-full_...")
            /// and company extensions with theirs.
            pub const XBRL_TAGS: &'static [(&'static str, $item)] = &[
                $($($(($tag, $item::$variant),)+)?)*
            ];

            /// The item an XBRL element reports, e.g. "us-gaap_Revenues" or
            /// "nvda_PurchasesOfPropertyAndEquipmentAndIntangibleAssets", as
            /// EDGAR's statement pages write them.
            pub fn from_xbrl_tag(tag: &str) -> Option<$item> {
                let name = tag.strip_prefix("us-gaap_").unwrap_or(tag);
                $($($(if name == $tag { return Some($item::$variant); })+)?)*
                None
            }
        }

        impl FromStr for $item {
            type Err = ItemError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let trimmed = s.trim();
                $(
                    $(
                        $(
                            if trimmed.eq_ignore_ascii_case($alias) {
                                return Ok($item::$variant);
                            }
                        )+
                    )?

                    if trimmed.eq_ignore_ascii_case(stringify!($variant)) {
                        return Ok($item::$variant);
                    }
                )*
                Err(ItemError::ItemNotFound)
            }
        }
    };
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

items! {
    pub enum Item {
        CashAndCashEquivalents => ["Cash and cash equivalents", "Cash and Cash Equivalents"] @ ["CashAndCashEquivalentsAtCarryingValue", "ifrs-full_CashAndCashEquivalents"],
        // TODO(atec): Short-term investments
        MarketableSecurities => ["Marketable securities", "Short-term investments", "Marketable Securities", "Marketable debt securities", "Short-term marketable securities"] @ ["MarketableSecuritiesCurrent", "AvailableForSaleSecuritiesCurrent", "DebtSecuritiesCurrent", "ShortTermInvestments", "AvailableForSaleSecuritiesDebtSecuritiesCurrent", "DebtSecuritiesAvailableForSaleExcludingAccruedInterestCurrent", "MarketableSecurities"],
        AccountsReceivableNet => ["Accounts receivable, net", "Accounts Receivable, Net"] @ ["AccountsReceivableNetCurrent"],
        VendorNonTradeReceivables => ["Vendor non-trade receivables"] @ ["NontradeReceivablesCurrent"],
        Inventories => ["Inventories", "Inventory"] @ ["InventoryNet", "ifrs-full_Inventories"],
        PrepaidExpensesAndOtherCurrentAssets => ["Prepaid expenses and other current assets", "Prepaid Expenses and Other Current Assets"] @ ["PrepaidExpenseAndOtherAssetsCurrent"],
        OtherCurrentAssets => ["Other current assets"] @ ["OtherAssetsCurrent", "ifrs-full_OtherCurrentAssets"],
        TotalCurrentAssets => ["Total current assets", "Total Current Assets"] @ ["AssetsCurrent", "ifrs-full_CurrentAssets"],

        OperatingLeaseVehiclesNet => ["Operating lease vehicles, net", "Operating Lease Vehicles [Member]"] @ ["PropertySubjectToOrAvailableForOperatingLeaseNet", "DeferredCostsLeasingNetNoncurrent"],
        SolarEnergySystemsNet => ["Solar energy systems, net", "Solar Energy Systems [Member]"],

        PropertyAndEquipmentNet => ["Property and equipment, net", "Property, plant and equipment, net", "Property and Equipment, Net"] @ ["PropertyPlantAndEquipmentNet", "PropertyPlantAndEquipmentAndFinanceLeaseRightOfUseAssetAfterAccumulatedDepreciationAndAmortization", "ifrs-full_PropertyPlantAndEquipment"],
        OperatingLeaseAssets => ["Operating lease assets", "Operating lease right-of-use assets", "Operating Lease Assets"] @ ["OperatingLeaseRightOfUseAsset", "ifrs-full_RightofuseAssets"],
        Goodwill => ["Goodwill"] @ ["Goodwill"],
        DigitalAssets => ["Digital assets"] @ ["CryptoAssetFairValueNoncurrent"],
        IntangibleAssetsNet => ["Intangible assets, net", "Intangible Assets, Net"] @ ["IntangibleAssetsNetExcludingGoodwill"],
        DeferredIncomeTaxAssets => ["Deferred income tax assets", "Deferred tax assets", "Deferred Income Tax Assets"] @ ["DeferredIncomeTaxAssetsNet", "DeferredTaxAssetsNetCurrent", "DeferredTaxAssetsLiabilitiesNetCurrent", "DeferredTaxAssetsLiabilitiesNetNoncurrent", "ifrs-full_DeferredTaxAssets"],
        // TODO(atec): Digital assets, net
        NonMarketableEquitySecurities => ["Non-marketable equity securities", "Digital assets, net", "Non-Marketable Equity Securities"],
        OtherAssets => ["Other assets", "Other non-current assets", "Other Assets"] @ ["OtherAssetsNoncurrent", "OtherAssets", "ifrs-full_OtherNoncurrentAssets"],
        TotalNonCurrentAssets => ["Total non-current assets"] @ ["AssetsNoncurrent", "ifrs-full_NoncurrentAssets"],

        TotalAssets => ["Total assets", "Total Assets"] @ ["Assets", "ifrs-full_Assets"],

        AccountsPayable => ["Accounts payable", "Accounts Payable"] @ ["AccountsPayableCurrent", "ifrs-full_TradeAndOtherCurrentPayablesToTradeSuppliers"],
        AccruedAndOtherCurrentLiabilities => ["Accrued and other current liabilities", "Accrued liabilities and other", "Accrued and Other Current Liabilities"] @ ["AccruedLiabilitiesCurrent"],
        DeferredRevenue => ["Deferred revenue"] @ ["ContractWithCustomerLiabilityCurrent", "DeferredRevenueCurrent"],
        // TODO(atec): Current portion of debt and finance leases
        ShortTermDebt => ["Short-term debt", "Current portion of debt and finance leases", "Short-Term Debt"] @ ["DebtCurrent", "LongTermDebtCurrent"],
        CommercialPaper => ["Commercial paper"] @ ["CommercialPaper"],
        TermDebt => ["Term debt"],
        OtherCurrentLiabilities => ["Other current liabilities"] @ ["OtherLiabilitiesCurrent"],
        TotalCurrentLiabilities => ["Total current liabilities", "Total Current Liabilities"] @ ["LiabilitiesCurrent", "ifrs-full_CurrentLiabilities"],
        LongTermDebt => ["Long-term debt", "Long-Term Debt"] @ ["LongTermDebtNoncurrent", "LongTermDebt"],
        DebtAndFinanceLeasesNetOfCurrentPortion => ["Debt and finance leases, net of current portion"],
        DeferredRevenueNetOfCurrentPortion => ["Deferred revenue, net of current portion"] @ ["ContractWithCustomerLiabilityNoncurrent", "DeferredRevenueNoncurrent"],
        LongTermOperatingLeaseLiabilities => ["Long-term operating lease liabilities", "Long-Term Operating Lease Liabilities"] @ ["OperatingLeaseLiabilityNoncurrent"],
        OtherLongTermLiabilities => ["Other long-term liabilities", "Other Long-Term Liabilities"] @ ["OtherLiabilitiesNoncurrent"],
        OtherNonCurrentLiabilities => ["Other non-current liabilities"],
        TotalNonCurrentLiabilities => ["Total non-current liabilities"] @ ["LiabilitiesNoncurrent", "ifrs-full_NoncurrentLiabilities"],

        TotalLiabilities => ["Total liabilities", "Total Liabilities"] @ ["Liabilities", "ifrs-full_Liabilities"],

        Revenue => ["Revenue", "Revenues", "Total revenues", "Total Revenues", "Net sales", "Total net sales"] @ ["Revenues", "RevenueFromContractWithCustomerExcludingAssessedTax", "SalesRevenueNet", "SalesRevenueGoodsNet", "ifrs-full_Revenue", "ifrs-full_RevenueFromContractsWithCustomers"],
        CostOfRevenue => ["Cost of revenue", "Cost of revenues", "Total cost of revenue", "Total cost of revenues", "Total Cost of Revenue", "Total Cost of Revenues", "Cost of sales", "Total cost of sales"] @ ["CostOfRevenue", "CostOfGoodsAndServicesSold", "CostOfGoodsSold", "ifrs-full_CostOfSales"],

//        AutomotiveSalesRevenue => ["Automotive sales"],
//        AutomotiveRegulatoryCredits => ["Automotive regulatory credits"],
//        AutomotiveLeasingRevenue => ["Automotive leasing"],
//        EnergyGenerationAndStorageRevenue => ["Energy generation and storage"],
//        ServicesAndOtherRevenue => ["Services and other"],
//
//        AutomotiveSalesCost => ["Automotive sales"],
//        AutomotiveLeasingCost => ["Automotive leasing"],
//        EnergyGenerationAndStorageCost => ["Energy generation and storage"],
//        ServicesAndOtherCost => ["Services and other"],

        GrossProfit => ["Gross profit", "Gross Profit", "Total gross profit", "Total Gross Profit"] @ ["GrossProfit", "ifrs-full_GrossProfit"],

        ResearchAndDevelopment => ["Research and development", "Research and Development"] @ ["ResearchAndDevelopmentExpense", "ifrs-full_ResearchAndDevelopmentExpense"],
        SalesGeneralAndAdministrative => ["Sales, general and administrative", "Selling, general and administrative", "Sales, General and Administrative"] @ ["SellingGeneralAndAdministrativeExpense"],
        RestructuringAndOther => ["Restructuring and other"],
        TotalOperatingExpenses => ["Total operating expenses", "Total Operating Expenses"] @ ["OperatingExpenses", "ifrs-full_OperatingExpenseExcludingCostOfSales"],
        OperatingIncome => ["Operating income", "Income from operations", "Operating Income", "Income (loss) from operations"] @ ["OperatingIncomeLoss", "ifrs-full_ProfitLossFromOperatingActivities"],

        InterestIncome => ["Interest income", "Interest Income"] @ ["InvestmentIncomeInterest", "ifrs-full_RevenueFromInterest"],
        InterestExpense => ["Interest expense", "Interest Expense"] @ ["InterestExpense", "InterestExpenseNonoperating"],
        OtherIncomeNet => ["Other income, net", "Other income (expense), net", "Other (expense) income, net", "Other Income, Net", "Other expense, net"] @ ["OtherNonoperatingIncomeExpense", "OtherNonoperatingIncome"],
        TotalOtherIncomeNet => ["Total other income, net", "Total Other Income, Net"] @ ["NonoperatingIncomeExpense"],
        IncomeBeforeIncomeTax => ["Income before income tax", "Income before income taxes", "Income Before Income Tax", "Income (loss) before income taxes"] @ ["IncomeLossFromContinuingOperationsBeforeIncomeTaxesExtraordinaryItemsNoncontrollingInterest", "IncomeLossFromContinuingOperationsBeforeIncomeTaxesMinorityInterestAndIncomeLossFromEquityMethodInvestments", "ifrs-full_ProfitLossBeforeTax"],

        IncomeTaxExpense => ["Income tax expense", "Provision for income taxes", "Provision for (benefit from) income taxes", "Income Tax Expense"] @ ["IncomeTaxExpenseBenefit", "ifrs-full_IncomeTaxExpenseContinuingOperations"],
        NetIncome => ["Net income", "Net income (loss)", "Net (loss) income", "Net loss"] @ ["ProfitLoss", "NetIncomeLoss", "ifrs-full_ProfitLoss"],

        StockBasedCompensationExpense => ["Stock-based compensation expense"] @ ["ShareBasedCompensation", "AllocatedShareBasedCompensationExpense", "ifrs-full_AdjustmentsForSharebasedPayments"],
        DepreciationAndAmoritization => ["Depreciation and amortization"] @ ["DepreciationDepletionAndAmortization", "DepreciationAmortizationAndAccretionNet", "DepreciationAndAmortization"],
        DeferredIncomeTaxes => ["Deferred income taxes"],
        GainsOnNonMarketableEquitySecuritiesAndPubliclyHeldSecuritiesNet => ["securities, net"],
        Other,
        AccountsReceivable => ["Accounts receivable"],
        ChangeInInventories @ ["IncreaseDecreaseInInventories", "ifrs-full_AdjustmentsForDecreaseIncreaseInInventories"],
        PrepaidExpensesAndOtherAssets => ["Prepaid expenses and other assets"],
        ChangeInAccountsPayable @ ["IncreaseDecreaseInAccountsPayable"],
        ChangeInAccruedAndOtherCurrentLiabilities,
        ChangeInOtherLongTermLiabilities,
        NetCashProvidedByOperatingActivities => ["Net cash provided by operating activities"] @ ["NetCashProvidedByUsedInOperatingActivities", "NetCashProvidedByUsedInOperatingActivitiesContinuingOperations", "ifrs-full_CashFlowsFromUsedInOperatingActivities"],
        ProceedsFromMaturitiesOfMarketableSecurities => ["Proceeds from maturities of marketable securities"],
        ProceedsFromSalesOfMarketableSecurities => ["Proceeds from sales of marketable securities"],
        ProceedsFromSalesOfNonMarketableEquitySecurities => ["Proceeds from sales of non-marketable equity securities"],
        PurchasesOfMarketableSecurities => ["Purchases of marketable securities"],
        PurchasesRelatedToPropertyAndEquipmentAndIntangibleAssets => ["Purchases related to property and equipment and intangible assets"] @ ["PaymentsToAcquirePropertyPlantAndEquipment", "PaymentsToAcquireProductiveAssets", "nvda_PurchasesOfPropertyAndEquipmentAndIntangibleAssets", "ifrs-full_PurchaseOfPropertyPlantAndEquipmentClassifiedAsInvestingActivities"],
        PurchasesOfNonMarketableEquitySecurities => ["Purchases of non-marketable equity securities"],
        AcquisitionsNetOfCashAcquired => ["Acquisitions, net of cash acquired"],
        NetCashUsedInInvestingActivities => ["Net cash used in investing activities"] @ ["NetCashProvidedByUsedInInvestingActivities", "NetCashProvidedByUsedInInvestingActivitiesContinuingOperations", "ifrs-full_CashFlowsFromUsedInInvestingActivities"],
        NetCashUsedInFinancingActivities => ["Net cash used in financing activities"] @ ["NetCashProvidedByUsedInFinancingActivities", "NetCashProvidedByUsedInFinancingActivitiesContinuingOperations", "ifrs-full_CashFlowsFromUsedInFinancingActivities"],
        ProceedsRelatedToEmployeeStockPlans => ["Proceeds related to employee stock plans"],
        PaymentsRelatedToRepurchasesOfCommonStock => ["Payments related to repurchases of common stock"] @ ["PaymentsForRepurchaseOfCommonStock", "ifrs-full_PaymentsToAcquireOrRedeemEntitysShares"],
        PaymentsRelatedToEmployeeStockPlanTaxes => ["Payments related to employee stock plan taxes"],
        DividendsPaid => ["Dividends paid"] @ ["PaymentsOfDividends", "PaymentsOfDividendsCommonStock", "ifrs-full_DividendsPaidClassifiedAsFinancingActivities"],
        PrincipalPaymentsOnPropertyAndEquipmentAndIntangibleAssets => ["Principal payments on property and equipment and intangible assets"],
        RepaymentOfDebt => ["Repayment of debt"],
        ChangeInCashAndCashEquivalents => ["Change in cash and cash equivalents"] @ ["CashAndCashEquivalentsPeriodIncreaseDecrease", "CashAndCashEquivalentsPeriodIncreaseDecreaseExcludingExchangeRateEffect", "CashCashEquivalentsRestrictedCashAndRestrictedCashEquivalentsPeriodIncreaseDecreaseIncludingExchangeRateEffect", "CashCashEquivalentsRestrictedCashAndRestrictedCashEquivalentsPeriodIncreaseDecreaseExcludingExchangeRateEffect", "NetCashProvidedByUsedInContinuingOperations", "ifrs-full_IncreaseDecreaseInCashAndCashEquivalents"],
    }
}

impl Item {
    /// The item an XBRL element reports and the element's rank among that
    /// item's elements: 0 for the first listed. Elements are listed most
    /// inclusive first, so when a statement reports an item twice the total
    /// wins over a component (Berkshire tags "Sales and service revenues" with
    /// the element other companies use for total revenue, and its total with
    /// `Revenues`).
    pub fn from_xbrl_tag_ranked(tag: &str) -> Option<(Item, usize)> {
        let item = Item::from_xbrl_tag(tag)?;
        let name = tag.strip_prefix("us-gaap_").unwrap_or(tag);
        let rank = Item::XBRL_TAGS.iter()
            .filter(|(_, i)| *i == item)
            .position(|(t, _)| *t == name)?;
        Some((item, rank))
    }

    /// Items reported on the cash flow statement. Some share labels or
    /// elements with balance sheet and income statement items ("Inventories",
    /// "Net income"), so each statement only reports its own.
    pub fn is_cash_flow(&self) -> bool {
        use Item::*;
        matches!(self,
            StockBasedCompensationExpense | DepreciationAndAmoritization | DeferredIncomeTaxes
            | GainsOnNonMarketableEquitySecuritiesAndPubliclyHeldSecuritiesNet | Other
            | AccountsReceivable | ChangeInInventories | PrepaidExpensesAndOtherAssets
            | ChangeInAccountsPayable | ChangeInAccruedAndOtherCurrentLiabilities
            | ChangeInOtherLongTermLiabilities | NetCashProvidedByOperatingActivities
            | ProceedsFromMaturitiesOfMarketableSecurities | ProceedsFromSalesOfMarketableSecurities
            | ProceedsFromSalesOfNonMarketableEquitySecurities | PurchasesOfMarketableSecurities
            | PurchasesRelatedToPropertyAndEquipmentAndIntangibleAssets
            | PurchasesOfNonMarketableEquitySecurities | AcquisitionsNetOfCashAcquired
            | NetCashUsedInInvestingActivities | NetCashUsedInFinancingActivities
            | ProceedsRelatedToEmployeeStockPlans | PaymentsRelatedToRepurchasesOfCommonStock
            | PaymentsRelatedToEmployeeStockPlanTaxes | DividendsPaid
            | PrincipalPaymentsOnPropertyAndEquipmentAndIntangibleAssets | RepaymentOfDebt
            | ChangeInCashAndCashEquivalents)
    }
}

#[derive(Debug)]
pub enum ItemError {
    ItemNotFound,
}

impl Display for ItemError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct Reported {
    pub ticker: crate::Ticker,
    pub date: chrono::NaiveDate,
    pub p: crate::Period,
    pub item: Item,
    pub currency: crate::Currency,
    pub val: f64,
}

#[cfg(test)]
mod tests {
    use super::Item;

    #[test]
    fn each_xbrl_tag_maps_to_one_item() {
        let mut seen = std::collections::HashMap::new();
        for (tag, item) in Item::XBRL_TAGS {
            if let Some(other) = seen.insert(*tag, *item) {
                panic!("{} is listed under both {:?} and {:?}", tag, other, item);
            }
        }
    }

    #[test]
    fn tags() {
        assert_eq!(Item::from_xbrl_tag("us-gaap_Revenues"), Some(Item::Revenue));
        assert_eq!(Item::from_xbrl_tag("us-gaap_RevenuesAbstract"), None);
        assert_eq!(Item::from_xbrl_tag("nvda_PurchasesOfPropertyAndEquipmentAndIntangibleAssets"),
            Some(Item::PurchasesRelatedToPropertyAndEquipmentAndIntangibleAssets));
        assert_eq!(Item::from_xbrl_tag("tsla_Revenues"), None);
    }
}
