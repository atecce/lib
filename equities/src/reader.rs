use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::Ticker;
use crate::Period;
use crate::item::Item;
use crate::ReportedItem;

use calamine::{open_workbook_auto, Data, DataType, Reader as CalamineReader, Sheets, Range};
use chrono::NaiveDate;

pub struct Reader {
    workbook: Sheets<BufReader<File>>,
    ticker: Ticker,
}

pub fn new_reader(path: &Path, ticker: Ticker) -> Result<Reader, Box<dyn Error>> {
    Ok(Reader {
        workbook: open_workbook_auto(path)?,
        ticker,
    })
}

impl Reader {
    pub fn process_balance_sheet(&mut self) -> Result<Vec<ReportedItem>, Box<dyn Error>> {
        let sheet_name = self.find_sheet(&["BALANCE_SHEET", "Consolidated Balance Sheets", "Condensed Consolidated Balance S"])
            .ok_or("Balance sheet not found")?;
        
        let range = self.workbook.worksheet_range(&sheet_name)?;
        let header_rows: Vec<&[Data]> = range.rows().take(30).collect();

        let multiplier = multiplier(&header_rows).ok_or("failed to get multiplier")?;
        let col_info = col_info_balance_sheet(&header_rows)?;

        let label_col = self.detect_label_column(&range);
        let ticker = self.ticker.clone();

        let mut reported_items = Vec::new();
        let mut found_items: HashMap<usize, HashSet<Item>> = col_info.keys().map(|&c| (c, HashSet::new())).collect();

        for row in range.rows().filter(|row| !row.iter().all(|c| c.is_empty())) {
            if let Some(label) = row.get(label_col).and_then(|c| c.get_string()) {
                if let Ok(item) = label.parse::<Item>() {
                    for (&col, &date) in &col_info {
                        if found_items.get(&col).map(|s| s.contains(&item)).unwrap_or(true) {
                            continue;
                        }

                        let val = match row.get(col) {
                            Some(Data::Float(f)) => *f,
                            Some(Data::Int(i)) => *i as f64,
                            _ => f64::NAN,
                        };

                        if !val.is_nan() {
                            found_items.get_mut(&col).unwrap().insert(item);
                            reported_items.push(ReportedItem {
                                ticker: ticker.to_string(),
                                t: date.to_string(),
                                p: Period::PointInTime,
                                item,
                                val: val * multiplier,
                            });
                        }
                    }
                }
            }
        }

        Ok(reported_items)
    }

    pub fn process_income_statement(&mut self) -> Result<Vec<ReportedItem>, Box<dyn Error>> {
        let sheet_name = self.find_sheet(&["INCOME_STATEMENT", "Consolidated Statements of Oper", "Consolidated Statements of Inco", "Condensed Consolidated Statemen"])
            .ok_or("Income statement not found")?;

        let range = self.workbook.worksheet_range(&sheet_name)?;
        let header_rows: Vec<&[Data]> = range.rows().take(30).collect();

        let multiplier = multiplier(&header_rows).ok_or("failed to get multiplier")?;
        let col_info = col_info_income_statement(&header_rows)?;

        let label_col = self.detect_label_column(&range);
        let ticker = self.ticker.clone();

        let mut reported_items = Vec::new();
        let mut found_items: HashMap<usize, HashSet<(Item, Period)>> = col_info.keys().map(|&c| (c, HashSet::new())).collect();

        for row in range.rows().filter(|row| !row.iter().all(|c| c.is_empty())) {
            if let Some(label) = row.get(label_col).and_then(|c| c.get_string()) {
                if let Ok(item) = label.parse::<Item>() {
                    for (&col, (date, period)) in &col_info {
                        if found_items.get(&col).map(|s| s.contains(&(item, *period))).unwrap_or(true) {
                            continue;
                        }

                        let val = match row.get(col) {
                            Some(Data::Float(f)) => *f,
                            Some(Data::Int(i)) => *i as f64,
                            _ => f64::NAN,
                        };

                        if !val.is_nan() {
                            found_items.get_mut(&col).unwrap().insert((item, *period));
                            reported_items.push(ReportedItem {
                                ticker: ticker.to_string(),
                                t: date.to_string(),
                                p: *period,
                                item,
                                val: val * multiplier,
                            });
                        }
                    }
                }
            }
        }

        Ok(reported_items)
    }

    fn find_sheet(&self, matches: &[&str]) -> Option<String> {
        let names = self.workbook.sheet_names();
        for m in matches {
            for name in &names {
                if name.to_lowercase().contains(&m.to_lowercase()) {
                    return Some(name.clone());
                }
            }
        }
        None
    }

    fn detect_label_column(&self, range: &Range<Data>) -> usize {
        let mut col0_count = 0;
        let mut col1_count = 0;
        for row in range.rows().take(50) {
            let row: &[Data] = row;
            if let Some(s) = row.get(0).and_then(|c| c.get_string()) {
                if s.parse::<Item>().is_ok() { col0_count += 1; }
            }
            if let Some(s) = row.get(1).and_then(|c| c.get_string()) {
                if s.parse::<Item>().is_ok() { col1_count += 1; }
            }
        }
        if col0_count >= col1_count { 0 } else { 1 }
    }
}

fn is_10k(rows: &[&[Data]]) -> bool {
    rows.iter().any(|r| r.iter()
        .any(|c| c.get_string()
            .map(|s| s.to_lowercase().contains("form type: 10-k") || s.to_lowercase().contains("12 months ended")).unwrap_or(false)))
}

fn multiplier(rows: &[&[Data]]) -> Option<f64> {
    rows.iter().find_map(|r| r.iter().find_map(|c| {
        let s = c.get_string()?.to_lowercase();
        if s.contains("in millions") { Some(1_000_000.0) }
        else if s.contains("in thousands") { Some(1_000.0) }
        else { None }
    }))
}

fn col_periods(rows: &[&[Data]]) -> HashMap<usize, Period> {
    let mut col_periods = HashMap::new();
    for row in rows {
        for (c, cell) in row.iter().enumerate() {
            if let Some(s) = cell.get_string() {
                let lower = s.to_lowercase();
                let p = if lower.contains("three months") { Some(Period::ThreeMonths) }
                    else if lower.contains("six months") { Some(Period::SixMonths) }
                    else if lower.contains("nine months") { Some(Period::NineMonths) }
                    else if lower.contains("year ended") || lower.contains("annual") || lower.contains("twelve months") { Some(Period::TwelveMonths) }
                    else { None };

                if let Some(period) = p {
                    col_periods.insert(c, period);
                }
            }
        }
    }

    col_periods
}

fn col_info_balance_sheet(rows: &[&[Data]]) -> Result<HashMap<usize, NaiveDate>, Box<dyn Error>> {
    let mut col_info = HashMap::new();
    for (r, row) in rows.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if let Some(date) = parse_date(cell) {
                col_info.entry(c).or_insert(date);
            } else if let Some(month_day) = cell.get_string().filter(|s| s.trim().ends_with(',') || s.trim().split_whitespace().count() >= 2) {
                if let Some(year) = rows.get(r + 1).and_then(|next| next.get(c)).and_then(|c| match c {
                    Data::Float(f) => Some(*f as i32),
                    Data::Int(i) => Some(*i as i32),
                    _ => None,
                }).filter(|&y| y > 1900 && y < 2100) {
                    if let Some(date) = parse_date_str(&format!("{} {}", month_day, year)) {
                        col_info.entry(c).or_insert(date);
                    }
                }
            }
        }
    }

    if col_info.is_empty() { return Err("no dates found".into()); }

    Ok(col_info)
}

fn col_info_income_statement(rows: &[&[Data]]) -> Result<HashMap<usize, (NaiveDate, Period)>, Box<dyn Error>> {
    let is_10k = is_10k(rows);
    let col_periods = col_periods(rows);
    let mut col_info = HashMap::new();

    for (r, row) in rows.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if let Some(date) = parse_date(cell) {
                // Find the nearest period label to the left or at the current column
                let mut p = None;
                for offset in (0..=c).rev().take(4) {
                    if let Some(detected_p) = col_periods.get(&offset) {
                        p = Some(*detected_p);
                        break;
                    }
                }
                
                let p = p.unwrap_or(if is_10k { Period::TwelveMonths } else { Period::ThreeMonths });
                col_info.entry(c).or_insert((date, p));
            } else if let Some(month_day) = cell.get_string().filter(|s| s.trim().ends_with(',') || s.trim().split_whitespace().count() >= 2) {
                if let Some(year) = rows.get(r + 1).and_then(|next| next.get(c)).and_then(|c| match c {
                    Data::Float(f) => Some(*f as i32),
                    Data::Int(i) => Some(*i as i32),
                    _ => None,
                }).filter(|&y| y > 1900 && y < 2100) {
                    if let Some(date) = parse_date_str(&format!("{} {}", month_day, year)) {
                        let mut p = None;
                        for offset in (0..=c).rev().take(4) {
                            if let Some(detected_p) = col_periods.get(&offset) {
                                p = Some(*detected_p);
                                break;
                            }
                        }
                        let p = p.unwrap_or(if is_10k { Period::TwelveMonths } else { Period::ThreeMonths });
                        col_info.entry(c).or_insert((date, p));
                    }
                }
            }
        }
    }

    if col_info.is_empty() { return Err("no dates found".into()); }

    Ok(col_info)
}

fn parse_date(cell: &Data) -> Option<NaiveDate> {
    match cell {
        Data::String(s) => parse_date_str(s),
        _ => None,
    }
}

fn parse_date_str(s: &str) -> Option<NaiveDate> {
    let s = s.trim().replace(",", "");
    let formats = ["%B %d %Y", "%b %d %Y", "%m/%d/%Y", "%Y-%m-%d", "%b. %d %Y"];
    formats.iter().find_map(|fmt| NaiveDate::parse_from_str(&s, fmt).ok())
}
