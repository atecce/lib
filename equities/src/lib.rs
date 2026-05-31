uniffi::setup_scaffolding!();

pub mod nvda;

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
        CashAndCashEquivalents => ["Cash and cash equivalents"],
        MarketableSecurities => ["Marketable securities"],
        AccountsReceivableNet => ["Accounts receivable, net"],
        Inventories => ["Inventories"],
        PrepaidExpensesAndOtherCurrentAssets => ["Prepaid expenses and other current assets"],
        TotalCurrentAssets => ["Total current assets"],
        PropertyAndEquipmentNet => ["Property and equipment, net"],
        OperatingLeaseAssets => ["Operating lease assets"],
        Goodwill => ["Goodwill"],
        IntangibleAssetsNet => ["Intangible assets, net"],
        DeferredIncomeTaxAssets => ["Deferred income tax assets"],
        NonMarketableEquitySecurities => ["Non-marketable equity securities"],
        OtherAssets => ["Other assets"],
        TotalAssets => ["Total assets"],

        CurrentLiabilities,
        AccountsPayable => ["Accounts payable"],
        AccruedAndOtherCurrentLiabilities => ["Accrued and other current liabilities"],
        ShortTermDebt => ["Short-term debt"],
        TotalCurrentLiabilities => ["Total current liabilities"],
        LongTermDebt => ["Long-term debt"],
        LongTermOperatingLeaseLiabilities => ["Long-term operating lease liabilities"],
        OtherLongTermLiabilities => ["Other long-term liabilities"],
        TotalLiabilities => ["Total liabilities"],

        Revenue => ["Revenue"],
        CostOfRevenue => ["Cost of revenue"],
        GrossProfit => ["Gross profit"],

        OperatingExpenses,
        ResearchAndDevelopment => ["Research and development"],
        SalesGeneralAndAdministrative => ["Sales, general and administrative"],
        TotalOperatingExpenses => ["Total operating expenses"],
        OperatingIncome => ["Operating income"],

        InterestIncome => ["Interest income"],
        InterestExpense => ["Interest expense"],
        OtherIncomeNet => ["Other income, net"],
        TotalOtherIncomeNet => ["Total other income, net"],
        IncomeBeforeIncomeTax => ["Income before income tax"],

        IncomeTaxExpense => ["Income tax expense"],
        NetIncome => ["Net income"],
    }
}

pub enum ItemError {
    ItemNotFound,
}

#[derive(Debug, serde::Serialize, uniffi::Record)]
pub struct ReportedItem {
    pub t: String,
    pub p: Period,
    pub item: Item,
    pub val: f64,
}
