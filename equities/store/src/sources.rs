//! Reading the source caches on disk into `equities`' readers, which take
//! text and bytes and open no files: the IO half of reading a company.

use std::error::Error;

use equities::item::Reported;
use equities::Ticker;

/// Every EDINET report of a company in the cache, read oldest first:
/// (items, reports read, reports not fetched yet, failures as messages).
pub fn edinet_company(cache: &edinet::Cache, ticker: Ticker) -> Result<(Vec<Reported>, usize, usize, Vec<String>), Box<dyn Error>> {
    let (mut items, mut count, mut missing, mut failures) = (Vec::new(), 0, 0, Vec::new());
    for d in &cache.reports(ticker.as_str())? {
        match edinet_report(cache, d, ticker) {
            Ok(mut r) => { items.append(&mut r); count += 1 }
            Err(e) => match e.downcast_ref::<edinet::EdinetError>() {
                Some(edinet::EdinetError::NotFetched(_)) => missing += 1,
                _ => failures.push(format!("{} {}: {}", ticker, d.doc_id, e)),
            },
        }
    }
    Ok((items, count, missing, failures))
}

/// One EDINET report from the cache: its CSV files, read by
/// `equities::edinet::read_csv`.
pub fn edinet_report(cache: &edinet::Cache, d: &edinet::Document, ticker: Ticker) -> Result<Vec<Reported>, Box<dyn Error>> {
    let mut files = Vec::new();
    for path in cache.csv_files(d)? {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
        files.push((name, std::fs::read(&path)?));
    }
    equities::edinet::read_csv(d, ticker, &files)
}

/// Every MOPS report of a company in the cache: (items, reports read,
/// failures as messages).
pub fn mops_company(cache: &mops::Cache, ticker: Ticker) -> Result<(Vec<Reported>, usize, Vec<String>), Box<dyn Error>> {
    let (mut items, mut read, mut failures) = (Vec::new(), 0, Vec::new());
    for report in cache.reports(ticker.as_str())? {
        match std::fs::read(&report.path) {
            Ok(bytes) => { items.append(&mut equities::mops::read_bytes(&report, ticker, &bytes)); read += 1 }
            Err(e) => failures.push(format!("{} {}: {}", ticker, report.id(), e)),
        }
    }
    Ok((items, read, failures))
}
