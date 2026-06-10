use std::collections::HashMap;
use std::error::Error;

use crate::Period;
use crate::item::Item;

use calamine::{Data, DataType};
use chrono::NaiveDate;

#[derive(PartialEq)]
pub enum SheetType {
    BalanceSheet,
    IncomeStatement,
    CashFlowStatement,
}

pub struct SheetInfo {
    pub dates_and_periods: HashMap<usize, (NaiveDate, Period)>,
    pub labels: usize,
    pub multiplier: f64,
    pub sheet_type: SheetType,
}

pub fn new_sheet_info(rows: &[&[Data]], sheet_type: SheetType) -> Result<SheetInfo, Box<dyn Error>> {

    let mut is_10k = false;
    let mut multiplier = 0.0;

    let mut col0_count = 0;
    let mut col1_count = 0;

    let mut periods = HashMap::new();

    let mut dates_and_periods = HashMap::new();
    for (r, row) in rows.iter().enumerate() {

        let row: &[Data] = row;
        if let Some(s) = row.get(0).and_then(|c| c.get_string()) {
            if s.parse::<Item>().is_ok() { col0_count += 1; }
        }
        if let Some(s) = row.get(1).and_then(|c| c.get_string()) {
            if s.parse::<Item>().is_ok() { col1_count += 1; }
        }

        for (c, cell) in row.iter().enumerate() {
            match cell {
                Data::String(s) => {
                    let s = s.to_lowercase();

                    if let Ok(p) = s.parse::<Period>() {
                        periods.insert(c, p);
                    }

                    if s.contains("form type: 10-k") || s.contains("12 months ended") {
                        is_10k = true;
                    }

                    if s.contains("in millions") { multiplier = 1_000_000.0; }
                    if s.contains("in thousands") { multiplier = 1_000.0; }

                    if let Some(date) = parse_date_str(&s).or_else(|| parse_date_across_cells(&s, rows.get(r+1).and_then(|next| next.get(c)))) {
                        match sheet_type {
                            SheetType::BalanceSheet => {
                                dates_and_periods.entry(c).or_insert((date, Period::PointInTime));
                            },
                            _ => {
                                let mut p = None;
                                for offset in (0..=c).rev().take(4) {
                                    if let Some(detected_p) = periods.get(&offset) {
                                        p = Some(*detected_p);
                                        break;
                                    }
                                }
                                dates_and_periods.entry(c).or_insert((date, p.unwrap_or(if is_10k { Period::TwelveMonths } else { Period::ThreeMonths })));
                            },
                        }
                    }
                },
                _ => continue,
            }
        }
    }

    if dates_and_periods.is_empty() { return Err("no dates found".into()); }

    let labels = if col0_count >= col1_count { 0 } else { 1 };

    Ok(SheetInfo {
        dates_and_periods: dates_and_periods,
        labels: labels,
        multiplier: multiplier,
        sheet_type: sheet_type,
    })
}

fn parse_date_across_cells(s: &str, next_cell: Option<&Data>) -> Option<NaiveDate> {
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

fn parse_date_str(s: &str) -> Option<NaiveDate> {
    let s = s.trim().replace(",", "");
    let formats = ["%B %d %Y", "%b %d %Y", "%m/%d/%Y", "%Y-%m-%d", "%b. %d %Y"];
    formats.iter().find_map(|fmt| NaiveDate::parse_from_str(&s, fmt).ok())
}
