use std::error::Error;
use std::sync::LazyLock;

use crate::Period;

use calamine::Data;
use chrono::NaiveDate;
use regex::Regex;

pub fn parse_date_str(s: &str) -> Option<NaiveDate> {
    let s = s.trim().replace(",", "");
    let formats = ["%B %d %Y", "%b %d %Y", "%m/%d/%Y", "%Y-%m-%d", "%b. %d %Y"];
    formats.iter().find_map(|fmt| NaiveDate::parse_from_str(&s, fmt).ok())
}

pub fn parse_date_across_lines(s: &str, next_s: &str) -> Result<NaiveDate, Box<dyn Error>> {
    if s.trim().ends_with(',') || s.trim().split_whitespace().count() >= 2 {
        let year = next_s.parse::<u16>().map_err(|e| format!("failed to parse '{}' as a int: {}", next_s, e))?;
        return parse_date_str(&format!("{} {}", s, year)).ok_or("failed to parse date str".into())
    }
    None.ok_or("failed to find trailing comma or split on whitespace greater than 2".into())
}

pub fn parse_date_across_cells(s: &str, next_cell: Option<&Data>) -> Option<NaiveDate> {
    if s.trim().ends_with(',') || s.trim().split_whitespace().count() >= 2 {
        if let Some(year) = next_cell.and_then(|c| match c {
            Data::Float(f) => Some(*f as i32),
            Data::Int(i) => Some(*i as i32),
            _ => None,
        }).filter(|&y| y > 1900 && y < 2100) {
            return parse_date_str(&format!("{} {}", s, year))
        }
    }
    None
}

#[derive(Debug, PartialEq)]
pub struct ReportInterval {
    pub period: Period,
    pub end_date: NaiveDate,
}

static REPORT_INTERVAL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    // Corrected verbose regex that allows flexible layout matching
    Regex::new(r"(?x)
        # Find the first period marker anywhere in the text
        ([A-Za-z]+)\s+Months?\s+Ended\s+([A-Za-z]+\s+\d{1,2}),\s*

        # Capture an optional second period marker on the same line
        (?:
            \s*([A-Za-z]+)\s+Months?\s+Ended\s+([A-Za-z]+\s+\d{1,2}),\s*
        )?

        # Handle trailing characters and potential clean lines before the years
        [^\d]*

        # Match the first two year columns
        \b(\d{4})\b\s+\b(\d{4})\b

        # Optionally match the third and fourth year columns
        (?:
            \s+\b(\d{4})\b\s+\b(\d{4})\b
        )?
    ").unwrap()
});

pub fn parse_financial_headers(text: &str) -> Result<Vec<ReportInterval>, Box<dyn Error>> {

    let mut ret = Vec::new();

    if let Some(caps) = REPORT_INTERVAL_REGEX.captures(text) {
        let p1_name = caps.get(1).map_or("", |m| m.as_str());
        let p1_date = caps.get(2).map_or("", |m| m.as_str());
        let year_col1 = caps.get(5).map_or("", |m| m.as_str());
        let year_col2 = caps.get(6).map_or("", |m| m.as_str());

        let p1 = match p1_name {
            "Three" => Period::ThreeMonths,
            "Six" => Period::SixMonths,
            "Nine" => Period::NineMonths,
            _ => Period::PointInTime, // Fallback safety
        };
        let date1 = NaiveDate::parse_from_str(&format!("{}, {}", p1_date, year_col1), "%B %d, %Y")?;
        let date2 = NaiveDate::parse_from_str(&format!("{}, {}", p1_date, year_col2), "%B %d, %Y")?;

        ret.push(ReportInterval {
            period: p1,
            end_date: date1,
        });
        ret.push(ReportInterval {
            period: p1,
            end_date: date2,
        });

        if let (Some(p2_name), Some(p2_date), Some(year_col3), Some(year_col4)) =
            (caps.get(3), caps.get(4), caps.get(7), caps.get(8))
        {
            let p2 = match p2_name.as_str() {
                "Three" => Period::ThreeMonths,
                "Six" => Period::SixMonths,
                "Nine" => Period::NineMonths,
                _ => Period::PointInTime, // Fallback safety
            };
            let date3 = NaiveDate::parse_from_str(&format!("{}, {}", p2_date.as_str(), year_col3.as_str()), "%B %d, %Y")?;
            let date4 = NaiveDate::parse_from_str(&format!("{}, {}", p2_date.as_str(), year_col4.as_str()), "%B %d, %Y")?;

            ret.push(ReportInterval {
                period: p2,
                end_date: date3,
            });
            ret.push(ReportInterval {
                period: p2,
                end_date: date4,
            });
        }
    }
    Ok(ret)
}
