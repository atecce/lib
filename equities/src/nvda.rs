use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::Period;
use crate::Item;
use crate::ReportedItem;

use calamine::{open_workbook_auto, Data, DataType, Range, Reader as CalamineReader, Sheets};
use chrono::NaiveDate;

pub struct Reader {
    workbook: Sheets<BufReader<File>>,

    multiplier: f64,
}

pub fn new_reader(path: &Path) -> Result<Reader, Box<dyn Error>> {
    Ok(Reader {
        workbook: open_workbook_auto(path)?,
        multiplier: 1.0,
    })
}

impl Reader {
    pub fn process_balance_sheet(&mut self) -> Result<Vec<ReportedItem>, Box<dyn Error>> {

        let range = self.workbook.worksheet_range("BALANCE_SHEET")?;

        let mut items = Vec::new();

        let mut dates = HashMap::new();
        let rows: Vec<_> = range.rows().take(25).collect();
    
        for (r, row) in rows.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                self.find_multiplier(cell);
                if let Some(date) = parse_date(cell) {
                    dates.entry(c).or_insert(date);
                } else if let Some(month_day) = cell.get_string() {
                    if month_day.trim().ends_with(',') || month_day.trim().split_whitespace().count() >= 2 {
                         if let Some(next_row) = rows.get(r + 1) {
                             if let Some(year_cell) = next_row.get(c) {
                                 let year = match year_cell {
                                     Data::Float(f) => *f as i32,
                                     Data::Int(i) => *i as i32,
                                     _ => 0,
                                 };
                                 if year > 1900 && year < 2100 {
                                     let date_str = format!("{} {}", month_day, year);
                                     if let Some(date) = parse_date_str(&date_str) {
                                         dates.entry(c).or_insert(date);
                                     }
                                 }
                             }
                         }
                    }
                }
            }
        }
    
        if dates.is_empty() { return Err("dates are empty".into()); }
    
        for row in range.rows().filter(|row| !row.iter().all(|c| c.is_empty())) {
            let label = match row.get(1) {
                Some(l) if !l.is_empty() => l.get_string().unwrap_or(""),
                _ => continue,
            };
    
            match label.parse::<Item>() {
                Ok(item) => {
                    for (col, date) in &dates {
                        if let Some(v) = row.get(*col) {
                            let val = match v {
                                Data::Float(f) => *f,
                                Data::Int(i) => *i as f64,
                                _ => f64::NAN,
                            };
                            if !val.is_nan() {
                                items.push(ReportedItem { t: date.to_string(), p: Period::PointInTime, item: item.clone(), val: val * self.multiplier });
                            }
                        }
                    }
                },
                Err(e) => eprintln!("failed to parse label: {}", label),
            }
        }
        Ok(items)
    }

    pub fn process_income_statement(&mut self) -> Result<Vec<ReportedItem>, Box<dyn Error>> {

        let range = self.workbook.worksheet_range("INCOME_STATEMENT")?;

        let mut items = Vec::new();

        let mut is_10k = false;
        let rows: Vec<_> = range.rows().take(25).collect();
        for row in rows.iter().take(10) {
            for cell in row.iter() {
                self.find_multiplier(cell);
                if let Some(s) = cell.get_string() {
                    if s.to_lowercase().contains("form type: 10-k") {
                        is_10k = true;
                    }
                }
            }
        }
    
        let mut col_periods: HashMap<usize, Period> = HashMap::new();
        for row in &rows {
            for (c, cell) in row.iter().enumerate() {
                if let Some(s) = cell.get_string() {
                    let s = s.to_lowercase();
                    let p = if s.contains("three months") { Some(Period::ThreeMonths) }
                        else if s.contains("six months") { Some(Period::SixMonths) }
                        else if s.contains("nine months") { Some(Period::NineMonths) }
                        else if s.contains("year ended") || s.contains("annual") || s.contains("twelve months") { Some(Period::TwelveMonths) }
                        else { None };
                    
                    if let Some(period) = p {
                        col_periods.insert(c, period);
                        col_periods.insert(c + 1, period);
                        col_periods.insert(c + 2, period);
                    }
                }
            }
        }
    
        let mut col_info = HashMap::new();
        for (r, row) in rows.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                if let Some(date) = parse_date(cell) {
                    let p = col_periods.get(&c).cloned().unwrap_or_else(|| {
                        if is_10k { Period::TwelveMonths } else { Period::ThreeMonths }
                    });
                    col_info.entry(c).or_insert((date, p));
                } else if let Some(month_day) = cell.get_string() {
                    if month_day.trim().ends_with(',') || month_day.trim().split_whitespace().count() >= 2 {
                         if let Some(next_row) = rows.get(r + 1) {
                             if let Some(year_cell) = next_row.get(c) {
                                 let year = match year_cell {
                                     Data::Float(f) => *f as i32,
                                     Data::Int(i) => *i as i32,
                                     _ => 0,
                                 };
                                 if year > 1900 && year < 2100 {
                                     let date_str = format!("{} {}", month_day, year);
                                     if let Some(date) = parse_date_str(&date_str) {
                                         let p = col_periods.get(&c).cloned().unwrap_or_else(|| {
                                             if is_10k { Period::TwelveMonths } else { Period::ThreeMonths }
                                         });
                                         col_info.entry(c).or_insert((date, p));
                                     }
                                 }
                             }
                         }
                    }
                }
            }
        }
    
        if col_info.is_empty() { return Err("col info is empty".into()); }
    
        for row in range.rows().filter(|row| !row.iter().all(|c| c.is_empty())) {
            let label = match row.get(1) {
                Some(l) if !l.is_empty() => l.get_string().unwrap_or(""),
                _ => continue,
            };
    
            match label.parse::<Item>() {
                Ok(item) => {
                    for (col, (date, period)) in &col_info {
                        if let Some(v) = row.get(*col) {
                            let val = match v {
                                Data::Float(f) => *f,
                                Data::Int(i) => *i as f64,
                                _ => f64::NAN,
                            };
                            if !val.is_nan() {
                                items.push(ReportedItem { t: date.to_string(), p: *period, item: item.clone(), val: val * self.multiplier });
                            }
                        }
                    }
                },
                Err(e) => eprintln!("failed to parse label {}", label),
            }
        }

        Ok(items)
    }

    fn find_multiplier(&mut self, cell: &Data) {
        if let Some(s) = cell.get_string() {
            let s = s.to_lowercase();
            if s.contains("in millions") {
                self.multiplier = 1_000_000.0;
            }
            if s.contains("in thousands") {
                self.multiplier =  1_000.0;
            }
        }
    }
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
    for fmt in formats {
        if let Ok(date) = NaiveDate::parse_from_str(&s, fmt) {
            return Some(date);
        }
    }
    None
}
