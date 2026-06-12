use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::Ticker;
use crate::Period;
use crate::Reader as R;
use crate::item::{Item, Reported};
use crate::sheet_info::{new_sheet_info, SheetInfo};
use crate::sheet_info::SheetType::{BalanceSheet, IncomeStatement, CashFlowStatement};

use calamine::{open_workbook_auto, Data, DataType, Reader as CalamineReader, Sheets};
use pdfsink_rs::{PdfDocument, TableSettings};

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
}

impl Reader {
    pub fn process_income_statement(&mut self) -> Result<Vec<Reported>, Box<dyn Error>> {
        let sheet_name = self.find_sheet(&["INCOME_STATEMENT", "Consolidated Statements of Oper", "Consolidated Statements of Inco", "Condensed Consolidated Statemen"])
            .ok_or("Income statement not found")?;

        let range = self.workbook.worksheet_range(&sheet_name)?;

        let rows: Vec<&[Data]> = range.rows().filter(|row| !row.iter().all(|c| c.is_empty())).collect();

        self.reported_items(&rows, new_sheet_info(&rows, IncomeStatement)?)
    }

    pub fn process_cash_flow_statement(&mut self) -> Result<Vec<Reported>, Box<dyn Error>> {
        let range = self.workbook.worksheet_range("CASH_FLOW")?;
        let rows: Vec<&[Data]> = range.rows().filter(|row| !row.iter().all(|c| c.is_empty())).collect();

        self.reported_items(&rows, new_sheet_info(&rows, CashFlowStatement)?)
    }

    pub fn rows(&mut self, sheet_name: &str) -> Result<Vec<Vec<Data>>, Box<dyn Error>> {
        let range = self.workbook.worksheet_range(sheet_name)?;
        Ok(range.rows()
            .filter(|row| !row.iter().all(|c| c.is_empty()))
            .map(|row| row.to_vec())
            .collect())
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

pub struct PDFReader {
    doc: PdfDocument,
    ticker: Ticker,
}

pub fn new_pdf_reader(path: &Path, ticker: Ticker) -> Result<PDFReader, Box<dyn Error>> {
    Ok(PDFReader {
        doc: PdfDocument::open(path)?,
        ticker: ticker,
    })
}

impl R for PDFReader {
    fn process_balance_sheet(&mut self) -> Result<Vec<Reported>, Box<dyn Error>> {
        let page = self.doc.page(4)?;

        let mut reported = Vec::new();

        if let Some(table) = page.extract_table(TableSettings::default())? {
            for row in &table {
                if let Ok(item) = row[0].as_ref().ok_or("failed to get first row item")?.parse::<Item>() {
                    let q3_2025 = chrono::NaiveDate::from_ymd_opt(2025, 9, 30).ok_or("q3 2025 not valid naive date")?;
                    let q4_2024 = chrono::NaiveDate::from_ymd_opt(2024, 12, 31).ok_or("q4 2024 not valid naive date")?;
                    match item {
                        Item::CashAndCashEquivalents | Item::AccountsPayable | Item::TotalAssets => {
                            if let Some(val) = &row[2] {
                                reported.push(Reported {
                                    ticker: self.ticker,
                                    date: q3_2025,
                                    p: Period::PointInTime,
                                    item,
                                    val: val.replace(',', "").parse::<f64>()? * 1_000_000.0,
                                });
                            }
                            if let Some(val) = &row[5] {
                                reported.push(Reported {
                                    ticker: self.ticker,
                                    date: q4_2024,
                                    p: Period::PointInTime,
                                    item,
                                    val: val.replace(',', "").parse::<f64>()? * 1_000_000.0,
                                });
                            }
                        },
                        _ => {
                            if let Some(val) = &row[1] {
                                reported.push(Reported {
                                    ticker: self.ticker,
                                    date: q3_2025,
                                    p: Period::PointInTime,
                                    item,
                                    val: val.replace(',', "").parse::<f64>()? * 1_000_000.0,
                                });
                            }
                            if let Some(val) = &row[4] {
                                reported.push(Reported {
                                    ticker: self.ticker,
                                    date: q4_2024,
                                    p: Period::PointInTime,
                                    item,
                                    val: val.replace(',', "").parse::<f64>()? * 1_000_000.0,
                                });
                            }
                        }
                    }
                }
            }
        }
        Ok(reported)
    }
}
