uniffi::setup_scaffolding!();

pub mod item;
pub mod reader;

use std::str::FromStr;

use crate::item::Item;

use chrono::NaiveDate;

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

    pub fn reported_items(&self) -> [ReportedItem; 21] {
        [
            ReportedItem {
                ticker: self.ticker.clone(),
                t: self.t.clone(),
                p: Period::PointInTime,
                item: Item::CashAndCashEquivalents,
                val: self.cash_and_cash_equivalents,
            },
            ReportedItem {
                ticker: self.ticker.clone(),
                t: self.t.clone(),
                p: Period::PointInTime,
                item: Item::MarketableSecurities,
                val: self.marketable_securities,
            },
            ReportedItem {
                ticker: self.ticker.clone(),
                t: self.t.clone(),
                p: Period::PointInTime,
                item: Item::AccountsReceivableNet,
                val: self.accounts_receivable_net,
            },
            ReportedItem {
                ticker: self.ticker.clone(),
                t: self.t.clone(),
                p: Period::PointInTime,
                item: Item::Inventories,
                val: self.inventories,
            },
            ReportedItem {
                ticker: self.ticker.clone(),
                t: self.t.clone(),
                p: Period::PointInTime,
                item: Item::PrepaidExpensesAndOtherCurrentAssets,
                val: self.prepaid_expenses_and_other_current_assets,
            },
            ReportedItem {
                ticker: self.ticker.clone(),
                t: self.t.clone(),
                p: Period::PointInTime,
                item: Item::TotalCurrentAssets,
                val: self.total_current_assets(),
            },
            ReportedItem {
                ticker: self.ticker.clone(),
                t: self.t.clone(),
                p: Period::PointInTime,
                item: Item::PropertyAndEquipmentNet,
                val: self.property_and_equipment_net,
            },
            ReportedItem {
                ticker: self.ticker.clone(),
                t: self.t.clone(),
                p: Period::PointInTime,
                item: Item::OperatingLeaseAssets,
                val: self.operating_lease_assets,
            },
            ReportedItem {
                ticker: self.ticker.clone(),
                t: self.t.clone(),
                p: Period::PointInTime,
                item: Item::Goodwill,
                val: self.goodwill,
            },
            ReportedItem {
                ticker: self.ticker.clone(),
                t: self.t.clone(),
                p: Period::PointInTime,
                item: Item::IntangibleAssetsNet,
                val: self.intangible_assets_net,
            },
            ReportedItem {
                ticker: self.ticker.clone(),
                t: self.t.clone(),
                p: Period::PointInTime,
                item: Item::DeferredIncomeTaxAssets,
                val: self.deferred_income_tax_assets,
            },
            ReportedItem {
                ticker: self.ticker.clone(),
                t: self.t.clone(),
                p: Period::PointInTime,
                item: Item::OtherAssets,
                val: self.other_assets,
            },
            ReportedItem {
                ticker: self.ticker.clone(),
                t: self.t.clone(),
                p: Period::PointInTime,
                item: Item::TotalAssets,
                val: self.total_assets(),
            },
            ReportedItem {
                ticker: self.ticker.clone(),
                t: self.t.clone(),
                p: Period::PointInTime,
                item: Item::AccountsPayable,
                val: self.accounts_payable,
            },
            ReportedItem {
                ticker: self.ticker.clone(),
                t: self.t.clone(),
                p: Period::PointInTime,
                item: Item::AccruedAndOtherCurrentLiabilities,
                val: self.accrued_and_other_current_liabilities,
            },
            ReportedItem {
                ticker: self.ticker.clone(),
                t: self.t.clone(),
                p: Period::PointInTime,
                item: Item::ShortTermDebt,
                val: self.short_term_debt,
            },
            ReportedItem {
                ticker: self.ticker.clone(),
                t: self.t.clone(),
                p: Period::PointInTime,
                item: Item::TotalCurrentLiabilities,
                val: self.total_current_liabilities(),
            },
            ReportedItem {
                ticker: self.ticker.clone(),
                t: self.t.clone(),
                p: Period::PointInTime,
                item: Item::LongTermDebt,
                val: self.long_term_debt,
            },
            ReportedItem {
                ticker: self.ticker.clone(),
                t: self.t.clone(),
                p: Period::PointInTime,
                item: Item::LongTermOperatingLeaseLiabilities,
                val: self.long_term_operating_lease_liabilities,
            },
            ReportedItem {
                ticker: self.ticker.clone(),
                t: self.t.clone(),
                p: Period::PointInTime,
                item: Item::OtherLongTermLiabilities,
                val: self.other_long_term_liabilities,
            },
            ReportedItem {
                ticker: self.ticker.clone(),
                t: self.t.clone(),
                p: Period::PointInTime,
                item: Item::TotalLiabilities,
                val: self.total_liabilities(),
            },
        ]
    }
}

pub struct IncomeStatement {

    ticker: String,

    t: String,

    revenue: f64,
    cost_of_revenue: f64,

    research_and_development: f64,
    sales_general_and_administrative: f64,

    interest_income: f64,
    interest_expense: f64,
    other_income_net: f64,

    income_tax_expense: f64,
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

#[derive(Debug)]
pub enum PeriodError {
    InvalidPeriod,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReportedItem {
    pub ticker: Ticker,
    pub t: NaiveDate,
    pub p: Period,
    pub item: Item,
    pub val: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Ticker {
    NVDA,
    TSLA,
}

impl std::fmt::Display for Ticker {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Ticker {
    type Err = TickerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();

        let lower = trimmed.to_lowercase();
        if lower.contains("nvda") { Ok(Ticker::NVDA) }
        else if lower.contains("tsla") { Ok(Ticker::TSLA) }
        else { Err(TickerError::TickerNotFound) }
    }
}

pub enum TickerError {
    TickerNotFound,
}
