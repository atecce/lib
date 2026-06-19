use std::error::Error;
use std::path::Path;

use crate::date::{parse_date_across_lines, parse_financial_headers};
use crate::Ticker;
use crate::Period;
use crate::Reader as R;
use crate::item::{Item, Reported};

use chrono::NaiveDate;
use pdfsink_rs::{PdfDocument, TableSettings};

pub struct Reader {
    doc: PdfDocument,
    ticker: Ticker,
}

pub fn new_reader(path: &Path, ticker: Ticker) -> Result<Reader, Box<dyn Error>> {
    Ok(Reader {
        doc: PdfDocument::open(path)?,
        ticker: ticker,
    })
}

impl R for Reader {
    fn process_balance_sheet(&mut self) -> Result<Vec<Reported>, Box<dyn Error>> {
        let page = self.doc.page(4)?;

        let mut reported = Vec::new();

        let text = page.extract_text();
        let lines = text.lines().collect::<Vec<_>>();

        let month_days = lines[7].split(",").filter(|l| *l != "").map(|l| l.trim()).collect::<Vec<_>>();
        let years = lines[8].split(" ").collect::<Vec<_>>();

        let present = parse_date_across_lines(month_days[0], years[0])?;
        let past = parse_date_across_lines(month_days[1], years[1])?;

        if let Some(table) = page.extract_table(TableSettings::default())? {
            for row in &table {
                if let Ok(item) = row[0].as_ref().ok_or("failed to get first row item")?.parse::<Item>() {
                    match item {
                        Item::CashAndCashEquivalents | Item::AccountsPayable | Item::TotalAssets => {
                            if let Some(val) = &row[2] {
                                reported.push(Reported {
                                    ticker: self.ticker,
                                    date: present,
                                    p: Period::PointInTime,
                                    item,
                                    val: val.replace(',', "").parse::<f64>().map_err(|e| format!("failed to parse '{}' as a float: {}", val, e))? * 1_000_000.0,
                                });
                            }
                            if let Some(val) = &row[5] {
                                reported.push(Reported {
                                    ticker: self.ticker,
                                    date: past,
                                    p: Period::PointInTime,
                                    item,
                                    val: val.replace(',', "").parse::<f64>().map_err(|e| format!("failed to parse '{}' as a float: {}", val, e))? * 1_000_000.0,
                                });
                            }
                        },
                        _ => {
                            if let Some(val) = &row[1] {
                                reported.push(Reported {
                                    ticker: self.ticker,
                                    date: present,
                                    p: Period::PointInTime,
                                    item,
                                    val: val.replace(',', "").parse::<f64>().map_err(|e| format!("failed to parse '{}' as a float: {}", val, e))? * 1_000_000.0,
                                });
                            }
                            if let Some(val) = &row[4] {
                                reported.push(Reported {
                                    ticker: self.ticker,
                                    date: past,
                                    p: Period::PointInTime,
                                    item,
                                    val: val.replace(',', "").parse::<f64>().map_err(|e| format!("failed to parse '{}' as a float: {}", val, e))? * 1_000_000.0,
                                });
                            }
                        }
                    }
                }
            }
        }
        Ok(reported)
    }

    fn process_income_statement(&mut self) -> Result<Vec<Reported>, Box<dyn Error>> {
        let page = self.doc.page(5)?;

        let mut reported = Vec::new();

        let text = page.extract_text();
        let lines = text.lines().collect::<Vec<_>>();

        let financial_headers = parse_financial_headers(&lines[5..=6]);
        if financial_headers.len() < 4 {
            return Err("less than 4 financial headers in income statement".into());
        }

        if let Some(table) = page.extract_table(TableSettings::default())? {
            for row in &table {
                if row[0].as_deref().unwrap_or_default() == "Revenues" || row[0].as_deref().unwrap_or_default() == "Cost of revenues" {
                    continue;
                }
                if let Ok(item) = row[0].as_ref().ok_or("failed to get first row item")?.parse::<Item>() {
                    match item {
//                        Item::AutomotiveSalesRevenue => {
//                            if let Some(val) = &row[2] {
//                                reported.push(parse_val(
//                                    self.ticker,
//                                    financial_headers[0].end_date,
//                                    financial_headers[0].period,
//                                    item,
//                                    val,
//                                )?);
//                            }
//                            if let Some(val) = &row[5] {
//                                reported.push(parse_val(
//                                    self.ticker,
//                                    financial_headers[1].end_date,
//                                    financial_headers[1].period,
//                                    item,
//                                    val,
//                                )?);
//                            }
//                            if let Some(val) = &row[8] {
//                                reported.push(parse_val(
//                                    self.ticker,
//                                    financial_headers[2].end_date,
//                                    financial_headers[2].period,
//                                    item,
//                                    val,
//                                )?);
//                            }
//                            if let Some(val) = &row[11] {
//                                reported.push(parse_val(
//                                    self.ticker,
//                                    financial_headers[3].end_date,
//                                    financial_headers[3].period,
//                                    item,
//                                    val,
//                                )?);
//                            }
//                        },
                        _ => {
                            if let Some(val) = &row[1] {
                                reported.push(parse_val(
                                    self.ticker,
                                    financial_headers[0].end_date,
                                    financial_headers[0].period,
                                    item,
                                    val,
                                )?);
                            }
                            if let Some(val) = &row[4] {
                                reported.push(parse_val(
                                    self.ticker,
                                    financial_headers[1].end_date,
                                    financial_headers[1].period,
                                    item,
                                    val,
                                )?);
                            }
                            if let Some(val) = &row[7] {
                                reported.push(parse_val(
                                    self.ticker,
                                    financial_headers[2].end_date,
                                    financial_headers[2].period,
                                    item,
                                    val,
                                )?);
                            }
                            if let Some(val) = &row[10] {
                                reported.push(parse_val(
                                    self.ticker,
                                    financial_headers[3].end_date,
                                    financial_headers[3].period,
                                    item,
                                    val,
                                )?);
                            }
                        }
                    }
                }
            }
        }
        Ok(reported)
    }
}

fn parse_val(ticker: Ticker, date: NaiveDate, period: Period, item: Item, val: &str) -> Result<Reported, Box<dyn Error>> {
    let ret: f64;
    if val.starts_with('(') && val.ends_with(')') {
        // Slice off the outer characters '(' and ')'
        let val = &val[1..val.len() - 1];
        // Parse the inner number and make it negative
        ret = val.parse::<f64>().map(|num| -num).map_err(|e| format!("failed to parse '{}' as a float: {}", val, e))? * 1_000_000.0;
    } else {
        ret = val.replace(',', "").parse::<f64>().map_err(|e| format!("failed to parse '{}' as a float: {}", val, e))? * 1_000_000.0;
    }
    return Ok(Reported {
        ticker: ticker,
        date: date,
        p: period,
        item: item,
        val: ret,
    })
}
