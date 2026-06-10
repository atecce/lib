use std::str::FromStr;

macro_rules! items {
    (
        pub enum $item:ident {
            $(
                $variant:ident $(=> [$($alias:literal),+])?
            ),* $(,)?
        }
    ) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, serde::Serialize, uniffi::Enum)]
        pub enum $item {
            $($variant),*
        }

        impl $item {
            pub const ALL: [$item; 66] = [
                $($item::$variant,)*
            ];
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

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Item {
    pub fn is_aggregate(self) -> bool {
        self == Item::TotalCurrentAssets
            || self == Item::TotalAssets
            || self == Item::TotalCurrentLiabilities
            || self == Item::TotalLiabilities
    }
}

items! {
    pub enum Item {
        CurrentAssets,
        CashAndCashEquivalents => ["Cash and cash equivalents", "Cash and Cash Equivalents"],
        MarketableSecurities => ["Marketable securities", "Short-term investments", "Marketable Securities"],
        AccountsReceivableNet => ["Accounts receivable, net", "Accounts Receivable, Net"],
        Inventories => ["Inventories", "Inventory"],
        PrepaidExpensesAndOtherCurrentAssets => ["Prepaid expenses and other current assets", "Prepaid Expenses and Other Current Assets"],
        TotalCurrentAssets => ["Total current assets", "Total Current Assets"],
        PropertyAndEquipmentNet => ["Property and equipment, net", "Property, plant and equipment, net", "Property and Equipment, Net"],
        OperatingLeaseAssets => ["Operating lease assets", "Operating lease right-of-use assets", "Operating Lease Assets"],
        Goodwill => ["Goodwill"],
        IntangibleAssetsNet => ["Intangible assets, net", "Intangible Assets, Net"],
        DeferredIncomeTaxAssets => ["Deferred income tax assets", "Deferred tax assets", "Deferred Income Tax Assets"],
        NonMarketableEquitySecurities => ["Non-marketable equity securities", "Digital assets, net", "Non-Marketable Equity Securities"],
        OtherAssets => ["Other assets", "Other non-current assets", "Other Assets"],
        TotalAssets => ["Total assets", "Total Assets"],

        CurrentLiabilities,
        AccountsPayable => ["Accounts payable", "Accounts Payable"],
        AccruedAndOtherCurrentLiabilities => ["Accrued and other current liabilities", "Accrued liabilities and other", "Accrued and Other Current Liabilities"],
        ShortTermDebt => ["Short-term debt", "Current portion of debt and finance leases", "Short-Term Debt"],
        TotalCurrentLiabilities => ["Total current liabilities", "Total Current Liabilities"],
        LongTermDebt => ["Long-term debt", "Debt and finance leases, net of current portion", "Long-Term Debt"],
        LongTermOperatingLeaseLiabilities => ["Long-term operating lease liabilities", "Deferred revenue, net of current portion", "Long-Term Operating Lease Liabilities"],
        OtherLongTermLiabilities => ["Other long-term liabilities", "Other Long-Term Liabilities"],
        TotalLiabilities => ["Total liabilities", "Total Liabilities"],

        Revenue => ["Revenue", "Revenues", "Total revenues", "Total Revenues"],
        CostOfRevenue => ["Cost of revenue", "Cost of revenues", "Total cost of revenue", "Total cost of revenues", "Total Cost of Revenue", "Total Cost of Revenues"],
        GrossProfit => ["Gross profit", "Gross Profit", "Total gross profit", "Total Gross Profit"],

        OperatingExpenses,
        ResearchAndDevelopment => ["Research and development", "Research and Development"],
        SalesGeneralAndAdministrative => ["Sales, general and administrative", "Selling, general and administrative", "Sales, General and Administrative"],
        TotalOperatingExpenses => ["Total operating expenses", "Total Operating Expenses"],
        OperatingIncome => ["Operating income", "Income from operations", "Operating Income"],

        InterestIncome => ["Interest income", "Interest Income"],
        InterestExpense => ["Interest expense", "Interest Expense"],
        OtherIncomeNet => ["Other income, net", "Other income (expense), net", "Other Income, Net"],
        TotalOtherIncomeNet => ["Total other income, net", "Total Other Income, Net"],
        IncomeBeforeIncomeTax => ["Income before income tax", "Income before income taxes", "Income Before Income Tax"],

        IncomeTaxExpense => ["Income tax expense", "Provision for (benefit from) income taxes", "Income Tax Expense"],
        NetIncome => ["Net income", "Net income attributable to common stockholders"],

        StockBasedCompensationExpense => ["Stock-based compensation expense"],
        DepreciationAndAmoritization => ["Depreciation and amortization"],
        DeferredIncomeTaxes => ["Deferred income taxes"],
        GainsOnNonMarketableEquitySecuritiesAndPubliclyHeldSecuritiesNet => ["securities, net"],
        Other,
        AccountsReceivable => ["Accounts receivable"],
        ChangeInInventories,
        PrepaidExpensesAndOtherAssets => ["Prepaid expenses and other assets"],
        ChangeInAccountsPayable,
        ChangeInAccruedAndOtherCurrentLiabilities,
        ChangeInOtherLongTermLiabilities,
        NetCashProvidedByOperatingActivities => ["Net cash provided by operating activities"],
        ProceedsFromMaturitiesOfMarketableSecurities => ["Proceeds from maturities of marketable securities"],
        ProceedsFromSalesOfMarketableSecurities => ["Proceeds from sales of marketable securities"],
        ProceedsFromSalesOfNonMarketableEquitySecurities => ["Proceeds from sales of non-marketable equity securities"],
        PurchasesOfMarketableSecurities => ["Purchases of marketable securities"],
        PurchasesRelatedToPropertyAndEquipmentAndIntangibleAssets => ["Purchases related to property and equipment and intangible assets"],
        PurchasesOfNonMarketableEquitySecurities => ["Purchases of non-marketable equity securities"],
        AcquisitionsNetOfCashAcquired => ["Acquisitions, net of cash acquired"],
        NetCashUsedInInvestingActivities => ["Net cash used in investing activities"],
        ProceedsRelatedToEmployeeStockPlans => ["Proceeds related to employee stock plans"],
        PaymentsRelatedToRepurchasesOfCommonStock => ["Payments related to repurchases of common stock"],
        PaymentsRelatedToEmployeeStockPlanTaxes => ["Payments related to employee stock plan taxes"],
        DividendsPaid => ["Dividends paid"],
        PrincipalPaymentsOnPropertyAndEquipmentAndIntangibleAssets => ["Principal payments on property and equipment and intangible assets"],
        RepaymentOfDebt => ["Repayment of debt"],
        ChangeInCashAndCashEquivalents => ["Change in cash and cash equivalents"],
    }
}

pub enum ItemError {
    ItemNotFound,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct Reported {
    pub ticker: crate::Ticker,
    pub date: chrono::NaiveDate,
    pub p: crate::Period,
    pub item: Item,
    pub val: f64,
}
