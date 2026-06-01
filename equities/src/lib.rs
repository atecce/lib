uniffi::setup_scaffolding!();

pub mod item;
pub mod reader;

use std::str::FromStr;

use crate::item::Item;

// use chrono::NaiveDate;

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

#[derive(Debug, serde::Serialize, uniffi::Record)]
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
