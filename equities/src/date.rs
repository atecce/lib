use std::error::Error;

use calamine::Data;
use chrono::NaiveDate;

pub fn parse_date_str(s: &str) -> Option<NaiveDate> {
    let s = s.trim().replace(",", "");
    let formats = ["%B %d %Y", "%b %d %Y", "%m/%d/%Y", "%Y-%m-%d", "%b. %d %Y"];
    formats.iter().find_map(|fmt| NaiveDate::parse_from_str(&s, fmt).ok())
}

pub fn parse_date_across_lines(s: &str, next_s: &str) -> Result<NaiveDate, Box<dyn Error>> {
    if s.trim().ends_with(',') || s.trim().split_whitespace().count() >= 2 {
        let year = next_s.parse::<u16>()?;
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
