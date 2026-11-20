use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::Ticker;
use crate::Reader as R;
use crate::item::{Item, Reported};
use crate::sheet_info::{new_sheet_info, SheetInfo, SheetType};
use crate::sheet_info::SheetType::{BalanceSheet, IncomeStatement, CashFlowStatement};

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

/// Non-empty rows of a statement sheet.
fn statement_rows(range: &calamine::Range<Data>) -> Vec<&[Data]> {
    range.rows()
        .filter(|row| !row.iter().all(|c| c.is_empty()))
        .collect()
}

impl R for Reader {
    fn process_balance_sheet(&mut self) -> Result<Vec<Reported>, Box<dyn Error>> {
        let sheet_name = self.find_sheet(&["BALANCE_SHEET", "Consolidated Balance"])
            .ok_or("Balance sheet not found")?;

        let range = self.workbook.worksheet_range(&sheet_name)?;

        let rows = statement_rows(&range);

        Self::reported_items(self.ticker, &rows, new_sheet_info(&rows, BalanceSheet)?)
    }

    fn process_income_statement(&mut self) -> Result<Vec<Reported>, Box<dyn Error>> {
        let sheet_name = self.find_sheet(&["INCOME_STATEMENT", "Consolidated Statements of Oper", "Consolidated Statements of Inco", "Consolidated Statem"])
            .ok_or("Income statement not found")?;

        let range = self.workbook.worksheet_range(&sheet_name)?;

        let rows = statement_rows(&range);

        Self::reported_items(self.ticker, &rows, new_sheet_info(&rows, IncomeStatement)?)
    }
}

impl Reader {
    pub fn process_cash_flow_statement(&mut self) -> Result<Vec<Reported>, Box<dyn Error>> {
        let range = self.workbook.worksheet_range("CASH_FLOW")?;
        let rows = statement_rows(&range);

        Self::reported_items(self.ticker, &rows, new_sheet_info(&rows, CashFlowStatement)?)
    }
    pub fn items(&mut self, sheet_type: SheetType) -> Result<Vec<Item>, Box<dyn Error>> {
        match sheet_type {
            BalanceSheet => {
                let range = self.workbook.worksheet_range("BALANCE_SHEET")?;
                let rows: Vec<&[Data]> = range.rows()
                    .filter(|row| !row.iter().all(|c| c.is_empty()))
                    .collect();

                let sheet_info = new_sheet_info(&rows, sheet_type)?;

                let mut items = Vec::new();
                for row in rows {
                    if let Some(label) = row.get(sheet_info.labels).and_then(|c| c.get_string()) {
                        if let Ok(item) = label.parse::<Item>() {
                            items.push(item);
                        }
                    }
                }

                Ok(items)
            },
            _ => Err("sheet type not supported".into()),
        }
    }

    fn find_sheet(&self, matches: &[&str]) -> Option<String> {
        let names = self.workbook.sheet_names();
        for m in matches {
            for name in &names {
                // Older EDGAR workbooks use underscores for spaces ("Condensed_Consolidated_Balance").
                if name.replace('_', " ").to_lowercase().contains(&m.replace('_', " ").to_lowercase()) {
                    return Some(name.clone());
                }
            }
        }
        None
    }

    /// Turn statement rows (label column plus one column per date) into reported items.
    /// Shared with `filing::Reader`, whose HTML tables are converted to the same rows.
    pub(crate) fn reported_items(ticker: Ticker, rows: &[&[Data]], sheet_info: SheetInfo) -> Result<Vec<Reported>, Box<dyn Error>> {

        let mut reported_items = Vec::new();

        // EDGAR's Financial_Report.xlsx appends per-segment breakdowns (rows under
        // headers like "Automotive Sales [Member]") after the statement proper. Some
        // line items only appear there (TSLA's "Operating lease vehicles, net"), while
        // others repeat labels from the face of the statement ("Revenues"). Keep the
        // first value seen for each (item, date, period) so the face statement wins.
        // A zero filled in from an empty cell (below) is only provisional: a later
        // row with a real number for the same key replaces it.
        let mut seen: std::collections::HashMap<_, (usize, bool)> = std::collections::HashMap::new();

        for row in rows {
            if let Some(label) = row.get(sheet_info.labels).and_then(|c| c.get_string()) {
                if let Ok(mut item) = label.parse::<Item>() {
                    // A line item with a number in some period but an empty cell in another
                    // is zero there (EDGAR leaves the cell empty where the filing shows "—").
                    // Rows with no numbers at all are section headers and are skipped.
                    let has_value = sheet_info.dates_and_periods.keys()
                        .any(|&col| matches!(row.get(col), Some(Data::Float(_)) | Some(Data::Int(_))));

                    for (&col, (date, period)) in &sheet_info.dates_and_periods {

                        let (val, filled) = match row.get(col) {
                            Some(Data::Float(f)) => (*f, false),
                            Some(Data::Int(i)) => (*i as f64, false),
                            Some(Data::Empty) | None if has_value => (0.0, true),
                            _ => (f64::NAN, false),
                        };

                        match sheet_info.sheet_type {
                            crate::sheet_info::SheetType::CashFlowStatement => {
                                match item {
                                    crate::Item::Inventories => {
                                        item = crate::Item::ChangeInInventories;
                                    },
                                    crate::Item::AccountsPayable => {
                                        item = crate::Item::ChangeInAccountsPayable;
                                    },
                                    crate::Item::AccruedAndOtherCurrentLiabilities => {
                                        item = crate::Item::ChangeInAccruedAndOtherCurrentLiabilities;
                                    },
                                    crate::Item::OtherLongTermLiabilities => {
                                        item = crate::Item::ChangeInOtherLongTermLiabilities;
                                    },
                                    _ => (),
                                }
                            },
                            _ => (),
                        }

                        if val.is_nan() { continue; }
                        let val = val * sheet_info.multiplier;
                        match seen.get_mut(&(item, *date, *period)) {
                            None => {
                                seen.insert((item, *date, *period), (reported_items.len(), filled));
                                reported_items.push(Reported {
                                    ticker,
                                    date: *date,
                                    p: *period,
                                    item,
                                    val,
                                });
                            },
                            Some((idx, was_filled)) if *was_filled && !filled => {
                                reported_items[*idx].val = val;
                                *was_filled = false;
                            },
                            Some(_) => (),
                        }
                    }
                }
            }
        }
        Ok(reported_items)
    }
}
