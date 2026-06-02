uniffi::setup_scaffolding!();

pub mod item;
pub mod reader;

use std::str::FromStr;

use crate::item::Item;

// use chrono::NaiveDate;

pub struct BalanceSheet {

    cash_and_cash_equivalents: f64,
    marketable_securities: f64,
    accounts_receivable_net: f64,
    inventories: f64,
    prepaid_expenses_and_other_current_assets: f64,

    property_and_equipment_net: f64,
    operating_lease_assets: f64,
    goodwill: f64,
    intangible_assets_net: f64,
    deferred_income_tax_assets: f64,
    other_assets: f64,

    accounts_payable: f64,
    accrued_and_other_current_liabilities: f64,
    short_term_debt: f64,

    long_term_debt: f64,
    long_term_operating_lease_liabilities: f64,
    other_long_term_liabilities: f64,
}

impl BalanceSheet {
    fn total_current_assets(&self) -> f64 {
        self.cash_and_cash_equivalents
            + self.marketable_securities
            + self.accounts_receivable_net
            + self.inventories
            + self.prepaid_expenses_and_other_current_assets
    }
    fn total_assets(self) -> f64 {
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
    fn total_liabilities(self) -> f64 {
        self.total_current_liabilities()
            + self.long_term_debt
            + self.long_term_operating_lease_liabilities
            + self.other_long_term_liabilities
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

#[derive(Debug)]
pub enum PeriodError {
    InvalidPeriod,
}

#[derive(Debug, PartialEq, serde::Serialize, uniffi::Record)]
pub struct ReportedItem {
    pub ticker: String,
    pub t: String,
    pub p: Period,
    pub item: Item,
    pub val: f64,
}

#[derive(Clone, Debug)]
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
