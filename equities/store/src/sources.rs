//! The source caches on disk, as `equities::statements`: each filing a
//! `Box<dyn Statements>` with its contents and what it is (`Filed`), made
//! here, at the edge, from each source's own description of it. `equities`
//! reads them and opens no file.

use std::error::Error;

use equities::item::{Filed, Source};
use edgar::filing::EdgarFiling;
use equities::statements::{EdinetReport, MopsReport, Statements};
use equities::Ticker;

/// A company's filings in a source's cache, ready to read, and what isn't.
#[derive(Default)]
pub struct Company {
    pub filings: Vec<Box<dyn Statements>>,
    /// Listed by the source but not in the cache yet.
    pub not_fetched: usize,
    /// Filed before the source rendered statements (EDGAR before 2009), or
    /// not a report the readers read.
    pub without_statements: usize,
    /// What failed, as messages for the caller to report.
    pub failures: Vec<String>,
}

/// A company's 10-Ks, 10-Qs and 20-Fs in the EDGAR cache.
pub fn edgar(cache: &edgar::Cache, ticker: Ticker) -> Result<Company, Box<dyn Error>> {
    let cik = cache.cik(ticker.sec_symbol())?;
    let mut company = Company::default();
    for f in cache.filings(cik, &edgar::FORMS)? {
        let filed = Filed {
            accession: f.accession.clone(),
            filed: f.filing_date,
            source: Source::Edgar,
            form: f.form.clone(),
            period_end: Some(f.report_date),
        };
        match cache.filing(&f).and_then(|dir| edgar_statements(&dir, filed, ticker)) {
            Ok(s) => company.filings.push(Box::new(s)),
            Err(e) => match e.downcast_ref::<edgar::EdgarError>() {
                Some(edgar::EdgarError::NoFilingSummary) => company.without_statements += 1,
                Some(edgar::EdgarError::NotFetched(_)) => company.not_fetched += 1,
                _ => company.failures.push(format!("{} {} {}: {}", ticker, f.form, f.accession, e)),
            },
        }
    }
    Ok(company)
}

/// One EDGAR filing from its folder: the index now, the pages as read.
fn edgar_statements(dir: &std::path::Path, filed: Filed, ticker: Ticker) -> Result<EdgarFiling, Box<dyn Error>> {
    let summary = std::fs::read_to_string(dir.join("FilingSummary.xml"))?;
    let folder = dir.to_path_buf();
    let pages: edgar::filing::Pages = Box::new(move |file| Ok(std::fs::read_to_string(folder.join(file))?));
    Ok(EdgarFiling::new(filed, ticker, &summary, pages))
}

/// A company's annual, semiannual and quarterly reports in the EDINET cache.
pub fn edinet(cache: &edinet::Cache, ticker: Ticker) -> Result<Company, Box<dyn Error>> {
    let mut company = Company::default();
    for d in &cache.reports(ticker.as_str())? {
        let Some(kind) = d.doc_type.as_deref().and_then(|t| equities::edinet::kind(t, d.description.as_deref())) else {
            company.without_statements += 1;
            continue;
        };
        // "2025-06-18 09:00", Japan time: the day is when the market learned of it.
        let Some(submitted) = d.submitted.as_deref().and_then(|s| s.get(..10)).and_then(|s| s.parse().ok()) else {
            company.failures.push(format!("{} {}: no submission date", ticker, d.doc_id));
            continue;
        };
        let filed = Filed {
            accession: d.doc_id.clone(),
            filed: submitted,
            source: Source::Edinet,
            form: d.doc_type.clone().unwrap_or_default(),
            period_end: d.period_end,
        };
        let files = cache.csv_files(d).and_then(|paths| paths.iter().map(|p| {
            let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
            Ok((name, std::fs::read(p)?))
        }).collect::<Result<Vec<_>, Box<dyn Error>>>());
        match files {
            Ok(files) => company.filings.push(Box::new(EdinetReport::new(filed, ticker, kind, files))),
            Err(e) => match e.downcast_ref::<edinet::EdinetError>() {
                Some(edinet::EdinetError::NotFetched(_)) => company.not_fetched += 1,
                _ => company.failures.push(format!("{} {}: {}", ticker, d.doc_id, e)),
            },
        }
    }
    Ok(company)
}

/// A company's quarterly and annual reports in the MOPS cache.
pub fn mops(cache: &mops::Cache, ticker: Ticker) -> Result<Company, Box<dyn Error>> {
    let mut company = Company::default();
    for report in cache.reports(ticker.as_str())? {
        let filed = Filed {
            accession: report.id(),
            // MOPS gives no filing day: the legal deadline stands in.
            filed: report.due(),
            source: Source::Mops,
            form: if report.quarter == 4 { "annual".into() } else { format!("Q{}", report.quarter) },
            period_end: Some(report.period_end()),
        };
        match std::fs::read(&report.path) {
            Ok(bytes) => company.filings.push(Box::new(MopsReport::new(filed, ticker, bytes))),
            Err(e) => company.failures.push(format!("{} {}: {}", ticker, report.id(), e)),
        }
    }
    Ok(company)
}

/// An EDGAR filing from its folder, as the reader alone, for tools that
/// read one statement at a time (dump_filing, trace).
pub fn edgar_filing(dir: &std::path::Path, ticker: Ticker) -> Result<edgar::filing::Reader, Box<dyn Error>> {
    let summary = std::fs::read_to_string(dir.join("FilingSummary.xml"))?;
    let folder = dir.to_path_buf();
    let pages: edgar::filing::Pages = Box::new(move |file| Ok(std::fs::read_to_string(folder.join(file))?));
    Ok(edgar::filing::reader(dir.display().to_string(), &summary, ticker, pages))
}

/// An EDGAR Financial_Report.xlsx from disk: `edgar::xlsx::reader`.
pub fn edgar_workbook(path: &std::path::Path, ticker: Ticker) -> Result<edgar::xlsx::Reader, Box<dyn Error>> {
    edgar::xlsx::reader(std::fs::read(path)?, ticker)
}
