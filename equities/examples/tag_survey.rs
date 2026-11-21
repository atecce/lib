//! For every cached EDGAR filing, count which XBRL element each statement row
//! is tagged with, for rows whose label maps to an Item. Shows which elements
//! each Item corresponds to across companies and years.
//!
//! Usage: tag_survey <edgar cache dir>
use std::collections::BTreeMap;

use calamine::DataType;
use equities::filing;
use equities::item::Item;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cache = std::env::args().nth(1).ok_or("usage: tag_survey <edgar cache dir>")?;
    let mut counts: BTreeMap<(String, Item, String), usize> = BTreeMap::new();
    for company in std::fs::read_dir(&cache)? {
        let company = company?.path();
        for filing_dir in std::fs::read_dir(&company)? {
            let dir = filing_dir?.path();
            let Ok(summary) = std::fs::read_to_string(dir.join("FilingSummary.xml")) else { continue };
            let reports = filing::parse_filing_summary(&summary);
            for (statement, report) in [
                ("balance", filing::balance_sheet(&reports)),
                ("income", filing::income_statement(&reports)),
                ("cash flow", filing::cash_flow_statement(&reports)),
            ] {
                let Some(report) = report else { continue };
                let Ok(html) = std::fs::read_to_string(dir.join(&report.file)) else { continue };
                let table = filing::table(&html)?;
                for (row, tag) in table.rows.iter().zip(&table.tags) {
                    let (Some(label), Some(tag)) = (row.first().and_then(|c| c.get_string()), tag) else { continue };
                    if let Ok(item) = label.parse::<Item>() {
                        *counts.entry((statement.to_string(), item, tag.clone())).or_default() += 1;
                    }
                }
            }
        }
    }
    for ((statement, item, tag), n) in counts {
        println!("{:<10} {:<45} {:>4}  {}", statement, item.to_string(), n, tag);
    }
    Ok(())
}
