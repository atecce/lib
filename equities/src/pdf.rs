use std::error::Error;
use std::path::Path;

use crate::Ticker;
use crate::Period;
use crate::Reader as R;
use crate::item::{Item, Reported};

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
