uniffi::setup_scaffolding!();

pub mod reader;

use std::str::FromStr;

// use chrono::NaiveDate;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, uniffi::Enum)]
pub enum Period {
    ThreeMonths,
    SixMonths,
    NineMonths,
    TwelveMonths,
    PointInTime,
}

impl Period {
    pub fn as_str(&self) -> &'static str {
        match self {
            Period::ThreeMonths => "3m",
            Period::SixMonths => "6m",
            Period::NineMonths => "9m",
            Period::TwelveMonths => "12m",
            Period::PointInTime => "pit",
        }
    }
}

impl FromStr for Period {
    type Err = PeriodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();

        let lower = trimmed.to_lowercase();
        if lower.contains("three months") { Ok(Period::ThreeMonths) }
        else if lower.contains("six months") { Ok(Period::SixMonths) }
        else if lower.contains("nine months") { Ok(Period::NineMonths) }
        else if lower.contains("year ended") || lower.contains("annual") || lower.contains("twelve months") { Ok(Period::TwelveMonths) }
        else { Err(PeriodError::InvalidPeriod) }
    }
}

#[derive(Debug)]
pub enum PeriodError {
    InvalidPeriod,
}

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
            pub const ALL: [$item; 39] = [
                $($item::$variant,)*
            ];

            pub fn as_str(&self) -> &'static str {
                match self {
                    $($item::$variant => stringify!($variant),)*
                }
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
    }
}

pub enum ItemError {
    ItemNotFound,
}

#[derive(Debug, serde::Serialize, uniffi::Record)]
pub struct ReportedItem {
    pub ticker: String,
    pub t: String,
    pub p: Period,
    pub item: Item,
    pub val: f64,
}
