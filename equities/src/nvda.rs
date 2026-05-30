use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::Period;
use crate::Item;
use crate::ReportedItem;

use calamine::{open_workbook_auto, Data, DataType, Reader as CalamineReader, Sheets};
use chrono::NaiveDate;

pub struct Reader {
    workbook: Sheets<BufReader<File>>,
}

pub fn new_reader(path: &Path) -> Result<Reader, Box<dyn Error>> {
    Ok(Reader {
        workbook: open_workbook_auto(path)?,
    })
}

impl Reader {
    pub fn process_balance_sheet(&mut self) -> Result<Vec<ReportedItem>, Box<dyn Error>> {
        let range = self.workbook.worksheet_range("BALANCE_SHEET")?;
        let header_rows: Vec<_> = range.rows().take(30).collect();

        let multiplier = header_rows.iter().take(20)
            .find_map(|r| r.iter().find_map(|c| {
                let s = c.get_string()?.to_lowercase();
                if s.contains("in millions") { Some(1_000_000.0) }
                else if s.contains("in thousands") { Some(1_000.0) }
                else { None }
            }))
            .unwrap_or(1.0);

        let col_info: HashMap<usize, NaiveDate> = header_rows.iter().enumerate()
            .flat_map(|(r, row)| {
                let rows = &header_rows; // Borrow for split-row check
                row.iter().enumerate().filter_map(move |(c, cell)| {
                    if let Some(date) = parse_date(cell) {
                        return Some((c, date));
                    }
                    if let Some(month_day) = cell.get_string().filter(|s| s.trim().ends_with(',') || s.trim().split_whitespace().count() >= 2) {
                        if let Some(year) = rows.get(r + 1).and_then(|next| next.get(c)).and_then(|c| match c {
                            Data::Float(f) => Some(*f as i32),
                            Data::Int(i) => Some(*i as i32),
                            _ => None,
                        }).filter(|&y| y > 1900 && y < 2100) {
                            if let Some(date) = parse_date_str(&format!("{} {}", month_day, year)) {
                                return Some((c, date));
                            }
                        }
                    }
                    None
                })
            })
            .collect();

        if col_info.is_empty() { return Err("no dates found".into()); }

        let items = range.rows()
            .filter(|row| !row.iter().all(|c| c.is_empty()))
            .filter_map(|row| {
                let label = row.get(1)?.get_string()?;
                let item = label.parse::<Item>().ok()?;
                Some((item, row))
            })
            .flat_map(|(item, row)| {
                col_info.iter().filter_map(move |(&col, &date)| {
                    let val = match row.get(col)? {
                        Data::Float(f) => *f,
                        Data::Int(i) => *i as f64,
                        _ => return None,
                    };
                    if val.is_nan() { return None; }
                    Some(ReportedItem { t: date.to_string(), p: Period::PointInTime, item, val: val * multiplier })
                })
            })
            .collect();

        Ok(items)
    }

    pub fn process_income_statement(&mut self) -> Result<Vec<ReportedItem>, Box<dyn Error>> {
        let range = self.workbook.worksheet_range("INCOME_STATEMENT")?;
        let header_rows: Vec<_> = range.rows().take(30).collect();

        let is_10k = header_rows.iter().take(10)
            .any(|r| r.iter().any(|c| c.get_string().map(|s| s.to_lowercase().contains("form type: 10-k")).unwrap_or(false)));

        let multiplier = header_rows.iter().take(20)
            .find_map(|r| r.iter().find_map(|c| {
                let s = c.get_string()?.to_lowercase();
                if s.contains("in millions") { Some(1_000_000.0) }
                else if s.contains("in thousands") { Some(1_000.0) }
                else { None }
            }))
            .unwrap_or(1.0);

        let mut col_periods = HashMap::new();
        header_rows.iter().enumerate().for_each(|(_, row)| {
            row.iter().enumerate().for_each(|(c, cell)| {
                if let Some(p) = cell.get_string().and_then(parse_period_str) {
                    col_periods.insert(c, p);
                    col_periods.insert(c + 1, p);
                    col_periods.insert(c + 2, p);
                }
            });
        });

        let col_info: HashMap<usize, (NaiveDate, Period)> = header_rows.iter().enumerate()
            .flat_map(|(r, row)| {
                let rows = &header_rows;
                let periods = &col_periods;
                row.iter().enumerate().filter_map(move |(c, cell)| {
                    if let Some(date) = parse_date(cell) {
                        let p = periods.get(&c).cloned().unwrap_or(if is_10k { Period::TwelveMonths } else { Period::ThreeMonths });
                        return Some((c, (date, p)));
                    }
                    if let Some(month_day) = cell.get_string().filter(|s| s.trim().ends_with(',') || s.trim().split_whitespace().count() >= 2) {
                        if let Some(year) = rows.get(r + 1).and_then(|next| next.get(c)).and_then(|c| match c {
                            Data::Float(f) => Some(*f as i32),
                            Data::Int(i) => Some(*i as i32),
                            _ => None,
                        }).filter(|&y| y > 1900 && y < 2100) {
                            if let Some(date) = parse_date_str(&format!("{} {}", month_day, year)) {
                                let p = periods.get(&c).cloned().unwrap_or(if is_10k { Period::TwelveMonths } else { Period::ThreeMonths });
                                return Some((c, (date, p)));
                            }
                        }
                    }
                    None
                })
            })
            .collect();

        if col_info.is_empty() { return Err("no dates found".into()); }

        let items = range.rows()
            .filter(|row| !row.iter().all(|c| c.is_empty()))
            .filter_map(|row| {
                let label = row.get(1)?.get_string()?;
                let item = label.parse::<Item>().ok()?;
                Some((item, row))
            })
            .flat_map(|(item, row)| {
                col_info.iter().filter_map(move |(&col, (date, period))| {
                    let val = match row.get(col)? {
                        Data::Float(f) => *f,
                        Data::Int(i) => *i as f64,
                        _ => return None,
                    };
                    if val.is_nan() { return None; }
                    Some(ReportedItem { t: date.to_string(), p: *period, item, val: val * multiplier })
                })
            })
            .collect();

        Ok(items)
    }
}

fn parse_period_str(s: &str) -> Option<Period> {
    let s = s.to_lowercase();
    if s.contains("three months") { Some(Period::ThreeMonths) }
    else if s.contains("six months") { Some(Period::SixMonths) }
    else if s.contains("nine months") { Some(Period::NineMonths) }
    else if s.contains("year ended") || s.contains("annual") || s.contains("twelve months") { Some(Period::TwelveMonths) }
    else { None }
}

fn parse_date(cell: &Data) -> Option<NaiveDate> {
    match cell {
        Data::String(s) => parse_date_str(s),
        _ => None,
    }
}

fn parse_date_str(s: &str) -> Option<NaiveDate> {
    let s = s.trim().replace(",", "");
    let formats = ["%B %d %Y", "%b %d %Y", "%m/%d/%Y", "%Y-%m-%d"];
    formats.iter().find_map(|fmt| NaiveDate::parse_from_str(&s, fmt).ok())
}
