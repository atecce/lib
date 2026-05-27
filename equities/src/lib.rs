uniffi::setup_scaffolding!();

pub mod nvda;

use std::str::FromStr;

// use chrono::NaiveDate;

#[derive(Debug, Clone, Copy, Hash, PartialEq, uniffi::Enum)]
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

macro_rules! items {
    (
        pub enum $item:ident {
            $(
                $variant:ident $(=> [$($alias:literal),+])?
            ),* $(,)?
        }
    ) => {
        #[derive(Clone, Debug, Eq, PartialEq, Hash, uniffi::Enum)]
        pub enum $item {
            $($variant),*
        }

        impl $item {
            const ALL: [$item; 39] = [
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

impl Item {
    pub fn as_str(&self) -> &'static str {
        match self {
            Item::CurrentAssets => "CurrentAssets",
            Item::CashAndCashEquivalents => "CashAndCashEquivalents",
            Item::MarketableSecurities => "MarketableSecurities",
            Item::AccountsReceivableNet => "AccountsReceivableNet",
            Item::Inventories => "Inventories",
            Item::PrepaidExpensesAndOtherCurrentAssets => "PrepaidExpensesAndOtherCurrentAssets",
            Item::TotalCurrentAssets => "TotalCurrentAssets",
            Item::PropertyAndEquipmentNet => "PropertyAndEquipmentNet",
            Item::OperatingLeaseAssets => "OperatingLeaseAssets",
            Item::Goodwill => "Goodwill",
            Item::IntangibleAssetsNet => "IntangibleAssetsNet",
            Item::DeferredIncomeTaxAssets => "DeferredIncomeTaxAssets",
            Item::NonMarketableEquitySecurities => "NonMarketableEquitySecurities",
            Item::OtherAssets => "OtherAssets",
            Item::TotalAssets => "TotalAssets",
            Item::CurrentLiabilities => "CurrentLiabilities",
            Item::AccountsPayable => "AccountsPayable",
            Item::AccruedAndOtherCurrentLiabilities => "AccruedAndOtherCurrentLiabilities",
            Item::ShortTermDebt => "ShortTermDebt",
            Item::TotalCurrentLiabilities => "TotalCurrentLiabilities",
            Item::LongTermDebt => "LongTermDebt",
            Item::LongTermOperatingLeaseLiabilities => "LongTermOperatingLeaseLiabilities",
            Item::OtherLongTermLiabilities => "OtherLongTermLiabilities",
            Item::TotalLiabilities => "TotalLiabilities",
            Item::Revenue => "Revenue",
            Item::CostOfRevenue => "CostOfRevenue",
            Item::GrossProfit => "GrossProfit",
            Item::OperatingExpenses => "OperatingExpenses",
            Item::ResearchAndDevelopment => "ResearchAndDevelopment",
            Item::SalesGeneralAndAdministrative => "SalesGeneralAndAdministrative",
            Item::TotalOperatingExpenses => "TotalOperatingExpenses",
            Item::OperatingIncome => "OperatingIncome",
            Item::InterestIncome => "InterestIncome",
            Item::InterestExpense => "InterestExpense",
            Item::OtherIncomeNet => "OtherIncomeNet",
            Item::TotalOtherIncomeNet => "TotalOtherIncomeNet",
            Item::IncomeBeforeIncomeTax => "IncomeBeforeIncomeTax",
            Item::IncomeTaxExpense => "IncomeTaxExpense",
            Item::NetIncome => "NetIncome",
        }
    }
}

pub enum ItemError {
    ItemNotFound,
}

#[derive(Debug, uniffi::Record)]
pub struct ReportedItem {
    pub t: String,
    pub p: Period,
    pub item: Item,
    pub val: f64,
}
