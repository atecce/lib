use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

use crate::Ticker;
use crate::Period;
use crate::Reader as R;
use crate::item::{Item, Reported};

use chrono::NaiveDate;
use regex::Regex;
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

    fn process_income_statement(&mut self) -> Result<Vec<Reported>, Box<dyn Error>> {
        let page = self.doc.page(5)?;

        let mut reported = Vec::new();

        let text = page.extract_text();
        let lines = text.lines().collect::<Vec<_>>();

        let financial_headers = parse_financial_headers(&lines[5..=6]);

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

#[derive(Debug, PartialEq)]
struct ReportInterval {
    pub period: Period,
    pub end_date: NaiveDate,
}

fn parse_financial_headers(lines: &[&str]) -> Vec<ReportInterval> {

    // Step 1: Set up mapping for month strings to numeric values
    let month_map: HashMap<&str, u32> = [
        ("January", 1), ("February", 2), ("March", 3), ("April", 4),
        ("May", 5), ("June", 6), ("July", 7), ("August", 8),
        ("September", 9), ("October", 10), ("November", 11), ("December", 12)
    ].iter().cloned().collect();

    // Step 2: Use regex to extract the chunks from the first line
    // Capture group 1: Period string, Capture group 2: Month name, Capture group 3: Day number
    let header_regex = Regex::new(r"((?:Three|Six|Nine|Twelve)\s+Months?\s+Ended)\s+([A-Za-z]+)\s+(\d+),?").unwrap();

    let mut structural_periods = Vec::new();
    for cap in header_regex.captures_iter(lines[0]) {
        let period_str = &cap[1];
        let month_str = &cap[2];
        let day: u32 = cap[3].parse().unwrap();

        let period = match period_str {
            "Three Months Ended" => Period::ThreeMonths,
            "Nine Months Ended" => Period::NineMonths,
            _ => Period::PointInTime, // Fallback safety
        };

        let month = *month_map.get(month_str).unwrap_or(&9); // Defaults to September if parsing fails

        // Save the parsed structure metadata (each period applies to 2 consecutive years)
        structural_periods.push((period, month, day));
    }

    // Step 3: Extract all the numeric years from the second line
    let year_regex = Regex::new(r"\b\d{4}\b").unwrap();
    let years: Vec<i32> = year_regex
        .find_iter(lines[1])
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .collect();

    let mut results = Vec::new();

    // Step 4: Zip structural descriptors together with the layout years
    // The string format implies: [Period 1 (2025), Period 1 (2024), Period 2 (2025), Period 2 (2024)]
    if structural_periods.len() == 2 && years.len() == 4 {
        // First period applies to the first two years
        results.push(ReportInterval {
            period: structural_periods[0].0,
            end_date: NaiveDate::from_ymd_opt(years[0], structural_periods[0].1, structural_periods[0].2).unwrap(),
        });
        results.push(ReportInterval {
            period: structural_periods[0].0,
            end_date: NaiveDate::from_ymd_opt(years[1], structural_periods[0].1, structural_periods[0].2).unwrap(),
        });

        // Second period applies to the last two years
        results.push(ReportInterval {
            period: structural_periods[1].0,
            end_date: NaiveDate::from_ymd_opt(years[2], structural_periods[1].1, structural_periods[1].2).unwrap(),
        });
        results.push(ReportInterval {
            period: structural_periods[1].0,
            end_date: NaiveDate::from_ymd_opt(years[3], structural_periods[1].1, structural_periods[1].2).unwrap(),
        });
    }

    results
}

fn parse_date_across_lines(s: &str, next_s: &str) -> Result<NaiveDate, Box<dyn Error>> {
    if s.trim().ends_with(',') || s.trim().split_whitespace().count() >= 2 {
        let year = next_s.parse::<u16>()?;
        return parse_date_str(&format!("{} {}", s, year)).ok_or("failed to parse date str".into())
    }
    None.ok_or("failed to find trailing comma or split on whitespace greater than 2".into())
}

fn parse_date_str(s: &str) -> Option<NaiveDate> {
    let s = s.trim().replace(",", "");
    let formats = ["%B %d %Y", "%b %d %Y", "%m/%d/%Y", "%Y-%m-%d", "%b. %d %Y"];
    formats.iter().find_map(|fmt| NaiveDate::parse_from_str(&s, fmt).ok())
}

fn parse_val(ticker: Ticker, date: NaiveDate, period: Period, item: Item, val: &str) -> Result<Reported, Box<dyn Error>> {
    let ret: f64;
    if val.starts_with('(') && val.ends_with(')') {
        // Slice off the outer characters '(' and ')'
        let val = &val[1..val.len() - 1];
        // Parse the inner number and make it negative
        ret = val.parse::<f64>().map(|num| -num)? * 1_000_000.0;
    } else {
        ret = val.replace(',', "").parse::<f64>()? * 1_000_000.0;
    }
    return Ok(Reported {
        ticker: ticker,
        date: date,
        p: period,
        item: item,
        val: ret,
    })
}
