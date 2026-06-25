use std::error::Error;
use std::path::Path;
use std::sync::LazyLock;

use crate::date::{parse_date_across_lines, parse_financial_headers};
use crate::Ticker;
use crate::Period;
use crate::Reader as R;
use crate::item::{Item, Reported};

use chrono::NaiveDate;
use regex::Regex;
use pdfsink_rs::{PdfDocument, TableSettings};

// Regex to match "For the quarterly period ended " followed by "Month DD, YYYY"
static QUARTERLY_PERIOD_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"For the quarterly period ended\s+([A-Za-z]+)\s+(\d{1,2}),\s+(\d{4})").unwrap()
});

pub struct Reader {
    ticker: Ticker,
    date: NaiveDate,
    doc: PdfDocument,
}

pub fn new_reader(path: &Path, ticker: Ticker) -> Result<Reader, Box<dyn Error>> {

    let doc = PdfDocument::open(path)?;
    let text = doc.page(1)?.extract_text();

    let caps = QUARTERLY_PERIOD_REGEX.captures(&text).ok_or("failed to capture quarterly_period")?;

    // Construct the full date string from the capture groups
    let date_str = format!("{} {}, {}", &caps[1], &caps[2], &caps[3]);

    Ok(Reader {
        ticker: ticker,
        // Parse "March 31, 2020" using the %B %e, %Y format specifier
        date: NaiveDate::parse_from_str(&date_str, "%B %e, %Y")?,
        doc: doc,
    })
}

impl R for Reader {
    fn process_balance_sheet(&mut self) -> Result<Vec<Reported>, Box<dyn Error>> {
        let page = self.doc.page(4)?;

        let mut reported = Vec::new();

        let mut table = page.extract_table(TableSettings::default())?.ok_or("failed to extract table")?;
        table.iter_mut().for_each(|row| {
            row.retain(|cell| {
                match cell.as_deref() {
                    None => false,
                    Some("") => false,
                    Some("$") => false,
                    _ => true,
                }
            });
        });

        let present: NaiveDate;
        let past: NaiveDate;
        if !(NaiveDate::from_ymd_opt(2020, 3, 31).unwrap() == self.date || NaiveDate::from_ymd_opt(2020, 6, 30).unwrap() == self.date) {

            let text = page.extract_text();
            let lines = text.lines().collect::<Vec<_>>();

            let month_days = lines[7].split(",").filter(|l| *l != "").map(|l| l.trim()).collect::<Vec<_>>();
            let years = lines[8].split(" ").collect::<Vec<_>>();

            present = parse_date_across_lines(month_days[0], years[0])?;
            past = parse_date_across_lines(month_days[1], years[1])?;
        } else {
            present = parse_date_across_lines(&table[0][0].as_ref().ok_or("failed to look up [0][0] in table")?, &table[1][0].as_ref().ok_or("failed to look up [1][0] in table")?)?;
            past = parse_date_across_lines(&table[0][1].as_ref().ok_or("failed to look up [0][1] in table")?, &table[1][1].as_ref().ok_or("failed to look up [1][1] in table")?)?;
        }

        for row in &table {
            if row.len() == 0 {
                continue;
            }
            if let Ok(item) = row[0].as_ref().ok_or("failed to get first row item")?.parse::<Item>() {
                if let Some(val) = &row[1] {
                    reported.push(parse_val(
                        self.ticker,
                        present,
                        Period::PointInTime,
                        item,
                        val,
                    )?);
                }
                if let Some(val) = &row[2] {
                    reported.push(parse_val(
                        self.ticker,
                        past,
                        Period::PointInTime,
                        item,
                        val,
                    )?);
                }
            }
        }
        Ok(reported)
    }

    fn process_income_statement(&mut self) -> Result<Vec<Reported>, Box<dyn Error>> {
        let page = self.doc.page(5)?;

        let mut reported = Vec::new();

        let mut table = page.extract_table(TableSettings::default())?.ok_or("failed to extract table")?;
        table.iter_mut().for_each(|row| {
            row.retain(|cell| {
                match cell.as_deref() {
                    None => false,
                    Some("") => false,
                    Some("$") => false,
                    Some(")") => false,
                    _ => true,
                }
            });
        });

        let text = page.extract_text();

        let financial_headers = parse_financial_headers(&text)?;
        if financial_headers.len() == 2 {
            for row in &table {
                if row.len() == 0 {
                    continue;
                }
                if row[0].as_deref().unwrap_or_default() == "Revenues" || row[0].as_deref().unwrap_or_default() == "Cost of revenues" {
                    continue;
                }
                if let Ok(item) = row[0].as_ref().ok_or("failed to get first row item")?.parse::<Item>() {
                    match item {
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
                            if let Some(val) = &row[2] {
                                reported.push(parse_val(
                                    self.ticker,
                                    financial_headers[1].end_date,
                                    financial_headers[1].period,
                                    item,
                                    val,
                                )?);
                            }
                        }
                    }
                }
            }
        } else if financial_headers.len() == 4 {
            for row in &table {
                if row.len() == 0 {
                    continue;
                }
                if row[0].as_deref().unwrap_or_default() == "Revenues" || row[0].as_deref().unwrap_or_default() == "Cost of revenues" {
                    continue;
                }
                if let Ok(item) = row[0].as_ref().ok_or("failed to get first row item")?.parse::<Item>() {
                    match item {
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
                            if let Some(val) = &row[2] {
                                reported.push(parse_val(
                                    self.ticker,
                                    financial_headers[1].end_date,
                                    financial_headers[1].period,
                                    item,
                                    val,
                                )?);
                            }
                            if let Some(val) = &row[3] {
                                reported.push(parse_val(
                                    self.ticker,
                                    financial_headers[2].end_date,
                                    financial_headers[2].period,
                                    item,
                                    val,
                                )?);
                            }
                            if let Some(val) = &row[4] {
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
        } else {
            return Err(format!("there should be 2 or 4 financial headers on income statement. found {:#?}", financial_headers).into());
        }

        Ok(reported)
    }
}

fn parse_val(ticker: Ticker, date: NaiveDate, period: Period, item: Item, val: &str) -> Result<Reported, Box<dyn Error>> {
    let ret: f64;
    if val == "—" {
        ret = 0.0;
    } else if val.starts_with('(') && val.ends_with(')') {
        // Slice off the outer characters '(' and ')'
        let val = &val[1..val.len() - 1];
        // Parse the inner number and make it negative
        ret = val.replace(",", "").parse::<f64>().map(|num| -num).map_err(|e| format!("failed to parse '{}' as a float for item '{}': {}", val, item, e))? * 1_000_000.0;
    } else if val.starts_with('(') {
        let val = &val[1..val.len()];
        ret = val.replace(",", "").parse::<f64>().map(|num| -num).map_err(|e| format!("failed to parse '{}' as a float for item '{}': {}", val, item, e))? * 1_000_000.0;
    } else {
        ret = val.replace(',', "").parse::<f64>().map_err(|e| format!("failed to parse '{}' as a float for item '{}': {}", val, item, e))? * 1_000_000.0;
    }
    return Ok(Reported {
        ticker: ticker,
        date: date,
        p: period,
        item: item,
        val: ret,
    })
}
