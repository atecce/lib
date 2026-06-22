use chrono::NaiveDate;
use regex::Regex;
use std::error::Error;

#[derive(Debug)]
pub struct FinancialPeriod {
    pub duration_months: u32,
    pub end_date: NaiveDate,
}

fn parse_months_string(s: &str) -> u32 {
    match s.to_lowercase().as_str() {
        "three" => 3,
        "nine" => 9,
        "six" => 6,
        "twelve" => 12,
        _ => 0,
    }
}

pub fn extract_periods(text: &str) -> Result<Vec<FinancialPeriod>, Box<dyn Error>> {
    let mut periods = Vec::new();

    // Corrected verbose regex that allows flexible layout matching
    let re = Regex::new(r"(?x)
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
    ")?;

    if let Some(caps) = re.captures(text) {
        // --- Group 1 (Columns 1 & 2) ---
        let p1_months = parse_months_string(caps.get(1).map_or("", |m| m.as_str()));
        let p1_date_str = caps.get(2).map_or("", |m| m.as_str());
        
        let y1 = caps.get(5).map_or("", |m| m.as_str());
        let y2 = caps.get(6).map_or("", |m| m.as_str());

        let date1 = NaiveDate::parse_from_str(&format!("{}, {}", p1_date_str, y1), "%B %d, %Y")?;
        let date2 = NaiveDate::parse_from_str(&format!("{}, {}", p1_date_str, y2), "%B %d, %Y")?;

        periods.push(FinancialPeriod { duration_months: p1_months, end_date: date1 });
        periods.push(FinancialPeriod { duration_months: p1_months, end_date: date2 });

        // --- Group 2 (Columns 3 & 4) ---
        if let (Some(m3), Some(d3), Some(y3), Some(y4)) = 
            (caps.get(3), caps.get(4), caps.get(7), caps.get(8)) 
        {
            let p2_months = parse_months_string(m3.as_str());
            let p2_date_str = d3.as_str();

            let date3 = NaiveDate::parse_from_str(&format!("{}, {}", p2_date_str, y3.as_str()), "%B %d, %Y")?;
            let date4 = NaiveDate::parse_from_str(&format!("{}, {}", p2_date_str, y4.as_str()), "%B %d, %Y")?;

            periods.push(FinancialPeriod { duration_months: p2_months, end_date: date3 });
            periods.push(FinancialPeriod { duration_months: p2_months, end_date: date4 });
        }
    }

    Ok(periods)
}

fn main() {
    let dump1 = "
Table of Contents
Tesla, Inc.
Consolidated Statements of Operations
(in millions, except per share data)
(unaudited)
Three Months Ended September 30, Nine Months Ended September 30,
2025 2024 2025 2024
Revenues
";

    let dump2 = "
Tesla, Inc.
Consolidated Statements of Operations
(in millions, except per share data)
(unaudited)
Three Months Ended March 31,
2020 2019
Revenues
";

    println!("--- Dump 1 Results ---");
    if let Ok(res) = extract_periods(dump1) {
        for p in res { println!("{:?}", p); }
    }

    println!("\n--- Dump 2 Results ---");
    if let Ok(res) = extract_periods(dump2) {
        for p in res { println!("{:?}", p); }
    }
}
