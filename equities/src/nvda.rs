use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

use crate::Period;
use crate::Item;
use crate::ReportedItem;

use calamine::{open_workbook_auto, Data, DataType, Reader as CalamineReader, Range};
use chrono::NaiveDate;

pub struct Reader {
    balance_sheet: Range<Data>,
    income_statement: Range<Data>,
}

pub fn new_reader(path: &Path) -> Result<Reader, Box<dyn Error>> {

    let mut workbook = open_workbook_auto(path)?;

    Ok(Reader {
        balance_sheet: workbook.worksheet_range("BALANCE_SHEET")?,
        income_statement: workbook.worksheet_range("INCOME_STATEMENT")?,
    })
}

impl Reader {
    pub fn process_balance_sheet(&mut self) -> Result<Vec<ReportedItem>, Box<dyn Error>> {
        let header_rows: &Vec<&[Data]> = &self.balance_sheet.rows().take(30).collect();

        let multiplier = multiplier(header_rows).ok_or("failed to get multiplier")?;

        let col_info = col_info_balance_sheet(header_rows)?;

        let reported_items = self.balance_sheet.rows()
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

        Ok(reported_items)
    }

    pub fn process_income_statement(&mut self) -> Result<Vec<ReportedItem>, Box<dyn Error>> {
        let header_rows: &Vec<&[Data]> = &self.income_statement.rows().take(30).collect();

        let is_10k = is_10k(header_rows);
        let multiplier = multiplier(header_rows).ok_or("failed to get multiplier")?;

        let mut col_periods = HashMap::new();
        header_rows.iter().enumerate().for_each(|(_, row)| {
            row.iter().enumerate().for_each(|(c, cell)| {
                if let Some(s) = cell.get_string() {
                    match s.parse::<Period>() {
                        Ok(p) => {
                            col_periods.insert(c, p);
                            col_periods.insert(c + 1, p);
                            col_periods.insert(c + 2, p);
                        },
                        Err(e) => eprintln!("failed to parse {} into period: {:?}", s, e),
                    }
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

        let reported_items = self.income_statement.rows()
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

        Ok(reported_items)
    }
}

fn is_10k(rows: &Vec<&[Data]>) -> bool {
    rows.iter().any(|r| r.iter()
        .any(|c| c.get_string()
            .map(|s| s.to_lowercase().contains("form type: 10-k")).unwrap_or(false)))
}

fn multiplier(rows: &Vec<&[Data]>) -> Option<f64> {
    rows.iter().find_map(|r| r.iter().find_map(|c| {
        let s = c.get_string()?.to_lowercase();
        if s.contains("in millions") { Some(1_000_000.0) }
        else if s.contains("in thousands") { Some(1_000.0) }
        else { None }
    }))
}

fn col_info_balance_sheet(rows: &Vec<&[Data]>) -> Result<HashMap<usize, NaiveDate>, Box<dyn Error>> {
    let col_info: HashMap<usize, NaiveDate> = rows.iter().enumerate().flat_map(|(r, row)| {
        let rows = &rows; // Borrow for split-row check
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
    let formats = ["%B %d %Y", "%b %d %Y", "%m/%d/%Y", "%Y-%m-%d"];
    formats.iter().find_map(|fmt| NaiveDate::parse_from_str(&s, fmt).ok())
}
