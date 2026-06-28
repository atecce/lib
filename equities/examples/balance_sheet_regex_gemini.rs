use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use regex::Regex;

#[derive(Debug)]
pub struct FinancialValues {
    pub current_year: f64,
    pub prior_year: f64,
}

/// 1. Main pipeline function to run the process
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read raw text from your input file
    let mut file = File::open("financials.txt")?;
    let mut raw_content = String::new();
    file.read_to_string(&mut raw_content)?;

    // Extract just the balance sheet table out of the document noise
    if let Some(balance_sheet_block) = extract_balance_sheet(&raw_content) {
        // Parse the block into structured key-value pairs
        let financials = parse_balance_sheet_kv(balance_sheet_block);

        // Display results to the console
        for (key, values) in &financials {
            println!("{:<45} | Current: {:<10} | Prior: {:<10}", key, values.current_year, values.prior_year);
        }
    } else {
        println!("Error: Could not find the Balance Sheet section inside the file.");
    }

    Ok(())
}



/// 2. Isolates the raw Balance Sheet table from the rest of the text
fn extract_balance_sheet(raw_text: &str) -> Option<&str> {
    // let re = Regex::new(
    //     r"(?i)(?:CONDENSED\s+)?CONSOLIDATED\s+BALANCE\s+SHEETS[\s\S]*?Total\s+liabilities\s+and\s+shareholders’\s+equity\s+\$?\s*[\d,()-\s]+\$?\s*[\d,()-]+"
    // ).unwrap();
    // Move the hyphen to the very end of the character class
    let re = Regex::new(
        r"(?i)(?:CONDENSED\s+)?CONSOLIDATED\s+BALANCE\s+SHEETS[\s\S]*?Total\s+liabilities\s+and\s+shareholders’\s+equity\s+\$?\s*[\d,()\s-]+?\$?\s*[\d,()\s-]+"
    ).unwrap();

    re.find(raw_text).map(|mat| mat.as_str())
}

/// 3. Extracts individual metrics into a Map structure
pub fn parse_balance_sheet_kv(balance_sheet_text: &str) -> HashMap<String, FinancialValues> {
    let mut data_map = HashMap::new();
    
    // let re = Regex::new(
    //     r"(?m)^\s*([A-Za-z][A-Za-z\s’,/()\-]+?)\s+(?:[\$\(]*\s*([\d,]+)\s*[\)]*)\s+(?:[\$\(]*\s*([\d,]+)\s*[\)]*)"
    // ).unwrap();
    // Move the hyphen to the very end here as well
    let re = Regex::new(
        r"(?m)^\s*([A-Za-z][A-Za-z\s’,/()\s-]+?)\s+(?:[\$\(]*\s*([\d,]+)\s*[\)]*)\s+(?:[\$\(]*\s*([\d,]+)\s*[\)]*)"
    ).unwrap();


    for caps in re.captures_iter(balance_sheet_text) {
        let key = caps[1].trim().to_string();
        
        // Isolate the raw inner numbers to discard punctuation later
        let val1_str = &caps[2];
        let val2_str = &caps[3];

        let current_year: f64 = val1_str.replace(",", "").parse().unwrap_or(0.0);
        let prior_year: f64 = val2_str.replace(",", "").parse().unwrap_or(0.0);

        data_map.insert(key, FinancialValues { current_year, prior_year });
    }

    data_map
}
