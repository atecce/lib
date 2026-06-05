uniffi::setup_scaffolding!();

pub mod item;
pub mod reader;

use std::str::FromStr;

use crate::item::Item;

use chrono::NaiveDate;

macro_rules! count_items {
    () => { 0 };
    ($head:ident $($tail:ident)*) => { 1 + count_items!($($tail)*) };
}

macro_rules! impl_reported_items {
    ($name:ident, @pit, [ $($variant:ident => $kind:ident $source:ident),* $(,)? ]) => {
        impl $name {
            pub fn reported_items(&self) -> [ReportedItem; count_items!($($variant)*)] {
                [
                    $(
                        ReportedItem {
                            ticker: self.ticker,
                            t: self.t,
                            p: Period::PointInTime,
                            item: Item::$variant,
                            val: impl_reported_items!(@val self, $kind $source),
                        },
                    )*
                ]
            }
        }
    };

    ($name:ident, @field $period_field:ident, [ $($variant:ident => $kind:ident $source:ident),* $(,)? ]) => {
        impl $name {
            pub fn reported_items(&self) -> [ReportedItem; count_items!($($variant)*)] {
                [
                    $(
                        ReportedItem {
                            ticker: self.ticker,
                            t: self.t,
                            p: self.$period_field,
                            item: Item::$variant,
                            val: impl_reported_items!(@val self, $kind $source),
                        },
                    )*
                ]
            }
        }
    };

    (@val $self:ident, field $source:ident) => { $self.$source };
    (@val $self:ident, method $source:ident) => { $self.$source() };
}

pub struct BalanceSheet {
    pub ticker: Ticker,
    pub t: NaiveDate,

    pub cash_and_cash_equivalents: f64,
    pub marketable_securities: f64,
    pub accounts_receivable_net: f64,
    pub inventories: f64,
    pub prepaid_expenses_and_other_current_assets: f64,

    pub property_and_equipment_net: f64,
    pub operating_lease_assets: f64,
    pub goodwill: f64,
    pub intangible_assets_net: f64,
    pub deferred_income_tax_assets: f64,
    pub other_assets: f64,

    pub accounts_payable: f64,
    pub accrued_and_other_current_liabilities: f64,
    pub short_term_debt: f64,

    pub long_term_debt: f64,
    pub long_term_operating_lease_liabilities: f64,
    pub other_long_term_liabilities: f64,
}

impl_reported_items! {
    BalanceSheet,
    @pit,
    [
        CashAndCashEquivalents => field cash_and_cash_equivalents,
        MarketableSecurities => field marketable_securities,
        AccountsReceivableNet => field accounts_receivable_net,
        Inventories => field inventories,
        PrepaidExpensesAndOtherCurrentAssets => field prepaid_expenses_and_other_current_assets,
        TotalCurrentAssets => method total_current_assets,
        PropertyAndEquipmentNet => field property_and_equipment_net,
        OperatingLeaseAssets => field operating_lease_assets,
        Goodwill => field goodwill,
        IntangibleAssetsNet => field intangible_assets_net,
        DeferredIncomeTaxAssets => field deferred_income_tax_assets,
        OtherAssets => field other_assets,
        TotalAssets => method total_assets,
        AccountsPayable => field accounts_payable,
        AccruedAndOtherCurrentLiabilities => field accrued_and_other_current_liabilities,
        ShortTermDebt => field short_term_debt,
        TotalCurrentLiabilities => method total_current_liabilities,
        LongTermDebt => field long_term_debt,
        LongTermOperatingLeaseLiabilities => field long_term_operating_lease_liabilities,
        OtherLongTermLiabilities => field other_long_term_liabilities,
        TotalLiabilities => method total_liabilities,
    ]
}

impl BalanceSheet {
    fn total_current_assets(&self) -> f64 {
        self.cash_and_cash_equivalents
            + self.marketable_securities
            + self.accounts_receivable_net
            + self.inventories
            + self.prepaid_expenses_and_other_current_assets
    }
    fn total_assets(&self) -> f64 {
        self.total_current_assets()
            + self.property_and_equipment_net
            + self.operating_lease_assets
            + self.goodwill
            + self.intangible_assets_net
            + self.deferred_income_tax_assets
            + self.other_assets
    }
    fn total_current_liabilities(&self) -> f64 {
        self.accounts_payable
            + self.accrued_and_other_current_liabilities
            + self.short_term_debt
    }
    fn total_liabilities(&self) -> f64 {
        self.total_current_liabilities()
            + self.long_term_debt
            + self.long_term_operating_lease_liabilities
            + self.other_long_term_liabilities
    }
}

pub struct IncomeStatement {
    pub ticker: Ticker,
    pub t: NaiveDate,
    pub p: Period,

    pub revenue: f64,
    pub cost_of_revenue: f64,

    pub research_and_development: f64,
    pub sales_general_and_administrative: f64,

    pub interest_income: f64,
    pub interest_expense: f64,
    pub other_income_net: f64,

    pub income_tax_expense: f64,
}

impl_reported_items! {
    IncomeStatement,
    @field p,
    [
        Revenue => field revenue,
        CostOfRevenue => field cost_of_revenue,
        GrossProfit => method gross_profit,
        ResearchAndDevelopment => field research_and_development,
        SalesGeneralAndAdministrative => field sales_general_and_administrative,
        TotalOperatingExpenses => method total_operating_expenses,
        OperatingIncome => method operating_income,
        InterestIncome => field interest_income,
        InterestExpense => field interest_expense,
        OtherIncomeNet => field other_income_net,
        TotalOtherIncomeNet => method total_other_income_net,
        IncomeBeforeIncomeTax => method income_before_income_tax,
        IncomeTaxExpense => field income_tax_expense,
        NetIncome => method net_income,
    ]
}

impl IncomeStatement {
    fn gross_profit(&self) -> f64 {
        self.revenue - self.cost_of_revenue
    }
    fn total_operating_expenses(&self) -> f64 {
        self.research_and_development + self.sales_general_and_administrative
    }
    fn operating_income(&self) -> f64 {
        self.gross_profit() - self.total_operating_expenses()
    }
    fn total_other_income_net(&self) -> f64 {
        self.interest_income + self.interest_expense + self.other_income_net
    }
    fn income_before_income_tax(&self) -> f64 {
        self.operating_income() + self.total_other_income_net()
    }
    fn net_income(&self) -> f64 {
        self.income_before_income_tax() - self.income_tax_expense
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, uniffi::Enum)]
pub enum Period {
    ThreeMonths,
    SixMonths,
    NineMonths,
    TwelveMonths,
    PointInTime,
}

impl std::fmt::Display for Period {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Period::ThreeMonths => "3m",
            Period::SixMonths => "6m",
            Period::NineMonths => "9m",
            Period::TwelveMonths => "12m",
            Period::PointInTime => "pit",
        };
        write!(f, "{}", s)
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

#[derive(Debug, uniffi::Error)]
pub enum PeriodError {
    InvalidPeriod,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct ReportedItem {
    pub ticker: Ticker,
    pub t: NaiveDate,
    pub p: Period,
    pub item: Item,
    pub val: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize)]
pub enum Ticker {
    NVDA,
    TSLA,
}

impl FromStr for Ticker {
    type Err = TickerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NVDA" => Ok(Ticker::NVDA),
            "TSLA" => Ok(Ticker::TSLA),
            _ => Err(TickerError::TickerNotFound),
        }
    }
}

pub enum TickerError {
    TickerNotFound,
}

impl std::fmt::Display for Ticker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ticker::NVDA => write!(f, "NVDA"),
            Ticker::TSLA => write!(f, "TSLA"),
        }
    }
}
