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
                                    val: val.replace(',', "").parse::<f64>()? * 1_000_000.0,
                                });
                            }
                            if let Some(val) = &row[5] {
                                reported.push(Reported {
                                    ticker: self.ticker,
                                    date: past,
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
                                    date: present,
                                    p: Period::PointInTime,
                                    item,
                                    val: val.replace(',', "").parse::<f64>()? * 1_000_000.0,
                                });
                            }
                            if let Some(val) = &row[4] {
                                reported.push(Reported {
                                    ticker: self.ticker,
                                    date: past,
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

fn parse_date_across_lines(s: &str, next_s: &str) -> Result<chrono::NaiveDate, Box<dyn Error>> {
    if s.trim().ends_with(',') || s.trim().split_whitespace().count() >= 2 {
        let year = next_s.parse::<u16>()?;
        return parse_date_str(&format!("{} {}", s, year)).ok_or("failed to parse date str".into())
    }
    None.ok_or("failed to find trailing comma or split on whitespace greater than 2".into())
}

fn parse_date_str(s: &str) -> Option<chrono::NaiveDate> {
    let s = s.trim().replace(",", "");
    let formats = ["%B %d %Y", "%b %d %Y", "%m/%d/%Y", "%Y-%m-%d", "%b. %d %Y"];
    formats.iter().find_map(|fmt| chrono::NaiveDate::parse_from_str(&s, fmt).ok())
}
