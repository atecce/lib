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

impl R for Reader {
    fn process_balance_sheet(&mut self) -> Result<Vec<Reported>, Box<dyn Error>> {
        let sheet_name = self.find_sheet(&["BALANCE_SHEET", "Consolidated Balance Sheets", "Condensed Consolidated Balance S"])
            .ok_or("Balance sheet not found")?;

        let range = self.workbook.worksheet_range(&sheet_name)?;

        let rows: Vec<&[Data]> = range.rows().filter(|row| !row.iter().all(|c| c.is_empty())).collect();

        self.reported_items(&rows, new_sheet_info(&rows, BalanceSheet)?)
    }

    fn process_income_statement(&mut self) -> Result<Vec<Reported>, Box<dyn Error>> {
        let sheet_name = self.find_sheet(&["INCOME_STATEMENT", "Consolidated Statements of Oper", "Consolidated Statements of Inco", "Condensed Consolidated Statemen"])
            .ok_or("Income statement not found")?;

        let range = self.workbook.worksheet_range(&sheet_name)?;

        let rows: Vec<&[Data]> = range.rows().filter(|row| !row.iter().all(|c| c.is_empty())).collect();

        self.reported_items(&rows, new_sheet_info(&rows, IncomeStatement)?)
    }
}

impl Reader {
    pub fn process_cash_flow_statement(&mut self) -> Result<Vec<Reported>, Box<dyn Error>> {
        let range = self.workbook.worksheet_range("CASH_FLOW")?;
        let rows: Vec<&[Data]> = range.rows().filter(|row| !row.iter().all(|c| c.is_empty())).collect();

        self.reported_items(&rows, new_sheet_info(&rows, CashFlowStatement)?)
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
                if name.to_lowercase().contains(&m.to_lowercase()) {
                    return Some(name.clone());
                }
            }
        }
        None
    }

    fn reported_items(&self, rows: &[&[Data]], sheet_info: SheetInfo) -> Result<Vec<Reported>, Box<dyn Error>> {

        let mut reported_items = Vec::new();

        for row in rows {
            if let Some(label) = row.get(sheet_info.labels).and_then(|c| c.get_string()) {
                if let Ok(mut item) = label.parse::<Item>() {
                    for (&col, (date, period)) in &sheet_info.dates_and_periods {

                        let val = match row.get(col) {
                            Some(Data::Float(f)) => *f,
                            Some(Data::Int(i)) => *i as f64,
                            _ => f64::NAN,
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

                        if !val.is_nan() {
                            reported_items.push(Reported {
                                ticker: self.ticker,
                                date: *date,
                                p: *period,
                                item,
                                val: val * sheet_info.multiplier,
                            });
                        }
                    }
                }
            }
        }
        Ok(reported_items)
    }
}
