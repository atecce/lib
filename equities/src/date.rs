use std::collections::HashMap;
use std::error::Error;
use std::sync::LazyLock;

use crate::Period;

use calamine::Data;
use chrono::NaiveDate;
use regex::Regex;

pub fn parse_date_str(s: &str) -> Option<NaiveDate> {
    let s = s.trim().replace(",", "");
    let formats = ["%B %d %Y", "%b %d %Y", "%m/%d/%Y", "%Y-%m-%d", "%b. %d %Y"];
    formats.iter().find_map(|fmt| NaiveDate::parse_from_str(&s, fmt).ok())
}

pub fn parse_date_across_lines(s: &str, next_s: &str) -> Result<NaiveDate, Box<dyn Error>> {
    if s.trim().ends_with(',') || s.trim().split_whitespace().count() >= 2 {
        let year = next_s.parse::<u16>()?;
        return parse_date_str(&format!("{} {}", s, year)).ok_or("failed to parse date str".into())
    }
    None.ok_or("failed to find trailing comma or split on whitespace greater than 2".into())
}

pub fn parse_date_across_cells(s: &str, next_cell: Option<&Data>) -> Option<NaiveDate> {
    if s.trim().ends_with(',') || s.trim().split_whitespace().count() >= 2 {
        if let Some(year) = next_cell.and_then(|c| match c {
            Data::Float(f) => Some(*f as i32),
            Data::Int(i) => Some(*i as i32),
            _ => None,
        }).filter(|&y| y > 1900 && y < 2100) {
            return parse_date_str(&format!("{} {}", s, year))
        }
    }
    None
}

#[derive(Debug, PartialEq)]
pub struct ReportInterval {
    pub period: Period,
    pub end_date: NaiveDate,
}

// Step 1: Set up mapping for month strings to numeric values
static MONTH_MAP: LazyLock<HashMap<&str, u32>> = LazyLock::new(|| {
    HashMap::from([
        ("January", 1), ("February", 2), ("March", 3), ("April", 4),
        ("May", 5), ("June", 6), ("July", 7), ("August", 8),
        ("September", 9), ("October", 10), ("November", 11), ("December", 12)
    ])
});

// Step 2: Use regex to extract the chunks from the first line
// Capture group 1: Period string, Capture group 2: Month name, Capture group 3: Day number
static HEADER_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"((?:Three|Six|Nine|Twelve)\s+Months?\s+Ended)\s+([A-Za-z]+)\s+(\d+),?").unwrap()
});

// Step 3: Extract all the numeric years from the second line
static YEAR_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b\d{4}\b").unwrap()
});

pub fn parse_financial_headers(lines: &[&str]) -> Vec<ReportInterval> {

    let mut structural_periods = Vec::new();
    for cap in HEADER_REGEX.captures_iter(lines[0]) {
        let period_str = &cap[1];
        let month_str = &cap[2];
        let day: u32 = cap[3].parse().unwrap();

        let period = match period_str {
            "Three Months Ended" => Period::ThreeMonths,
            "Nine Months Ended" => Period::NineMonths,
            _ => Period::PointInTime, // Fallback safety
        };

        let month = *MONTH_MAP.get(month_str).unwrap_or(&9); // Defaults to September if parsing fails

        // Save the parsed structure metadata (each period applies to 2 consecutive years)
        structural_periods.push((period, month, day));
    }

    let years: Vec<i32> = YEAR_REGEX
        .find_iter(lines[1])
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .collect();

    let mut results = Vec::new();

    // Step 4: Zip structural descriptors together with the layout years
    // The string format implies: [Period 1 (2025), Period 1 (2024), Period 2 (2025), Period 2 (2024)]
    if structural_periods.len() == 2 && years.len() == 4 {
        // First period applies to the first two years
        results.push(ReportInterval {
            period: structural_periods[0].0,
            end_date: NaiveDate::from_ymd_opt(years[0], structural_periods[0].1, structural_periods[0].2).unwrap(),
        });
        results.push(ReportInterval {
            period: structural_periods[0].0,
            end_date: NaiveDate::from_ymd_opt(years[1], structural_periods[0].1, structural_periods[0].2).unwrap(),
        });

        // Second period applies to the last two years
        results.push(ReportInterval {
            period: structural_periods[1].0,
            end_date: NaiveDate::from_ymd_opt(years[2], structural_periods[1].1, structural_periods[1].2).unwrap(),
        });
        results.push(ReportInterval {
            period: structural_periods[1].0,
            end_date: NaiveDate::from_ymd_opt(years[3], structural_periods[1].1, structural_periods[1].2).unwrap(),
        });
    }

    results
}
