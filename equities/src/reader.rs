use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::col_info::{new_col_info, ColInfo};
use crate::Ticker;
use crate::Period;
use crate::item::Item;
use crate::ReportedItem;

use calamine::{open_workbook_auto, Data, DataType, Reader as CalamineReader, Sheets};

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

        let rows: Vec<&[Data]> = range.rows().filter(|row| !row.iter().all(|c| c.is_empty())).collect();

        self.reported_items(&rows, new_col_info(&rows, true)?)
    }

    pub fn process_income_statement(&mut self) -> Result<Vec<ReportedItem>, Box<dyn Error>> {
        let sheet_name = self.find_sheet(&["INCOME_STATEMENT", "Consolidated Statements of Oper", "Consolidated Statements of Inco", "Condensed Consolidated Statemen"])
            .ok_or("Income statement not found")?;

        let range = self.workbook.worksheet_range(&sheet_name)?;

        let rows: Vec<&[Data]> = range.rows().filter(|row| !row.iter().all(|c| c.is_empty())).collect();

        self.reported_items(&rows, new_col_info(&rows, false)?)
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

    fn reported_items(&self, rows: &[&[Data]], col_info: ColInfo) -> Result<Vec<ReportedItem>, Box<dyn Error>> {

        let mut reported_items = Vec::new();
        let mut found_items: HashMap<usize, HashSet<(Item, Period)>> = col_info.dates_and_periods.keys().map(|&col| (col, HashSet::new())).collect();

        let multiplier = multiplier(&rows).ok_or("failed to get multiplier")?;

        for row in rows {
            if let Some(label) = row.get(col_info.labels).and_then(|c| c.get_string()) {
                if let Ok(item) = label.parse::<Item>() {
                    for (&col, (date, period)) in &col_info.dates_and_periods {
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
                                ticker: self.ticker,
                                date: *date,
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
}

fn multiplier(rows: &[&[Data]]) -> Option<f64> {
    rows.iter().find_map(|r| r.iter().find_map(|c| {
        let s = c.get_string()?.to_lowercase();
        if s.contains("in millions") { Some(1_000_000.0) }
        else if s.contains("in thousands") { Some(1_000.0) }
        else { None }
    }))
}
