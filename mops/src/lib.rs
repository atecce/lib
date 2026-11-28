//! MOPS (公開資訊觀測站, Gōngkāi Zīxùn Guāncè Zhàn, "public information
//! observation station"), Taiwan's register of listed companies' filings, as
//! a source: each company's quarterly financial reports, one page per
//! company and quarter, kept as MOPS serves them (Big5-encoded HTML) under
//! `cache/`:
//!
//! ```text
//! cache/<code>/<year>Q<quarter>.html    the report: balance sheet, statement
//!                                       of comprehensive income, cash flows
//! cache/<code>/<year>Q<quarter>.none    MOPS has no report for that quarter
//! ```
//!
//! The page is MOPS's XBRL viewer (`t164sb01`): consolidated statements
//! (REPORT_ID C), or a company's own where it has no subsidiaries (A). Reports
//! exist from 2013 Q1, when listed companies began filing IFRS statements in
//! XBRL. The fourth quarter's report is the annual one. Bulk zips of every
//! company's XBRL per quarter also exist, but are served at about 15 KB/s.
//!
//! `Cache` reads all this and never touches the network. With the `fetch`
//! feature, `Client` downloads into it, and `mops-fetch` runs it.

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

/// Reading a report's page into equities' items: `report::read_bytes`,
/// and `report::MopsReport`, which implements equities' `Statements`.
pub mod report;

#[cfg(feature = "fetch")]
mod client;
#[cfg(feature = "fetch")]
pub use client::{Client, Fetched};

/// The first quarter with reports: IFRS in XBRL, from 2013.
pub const FIRST_YEAR: i32 = 2013;

/// A quarter's report, cached.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Report {
    pub code: String,
    pub year: i32,
    /// 1 to 4; the fourth is the annual report.
    pub quarter: u32,
    pub path: PathBuf,
}

impl Report {
    /// An identifier for the report, standing in for an accession number:
    /// "MOPS-2330-2025Q2".
    pub fn id(&self) -> String {
        format!("MOPS-{}-{}Q{}", self.code, self.year, self.quarter)
    }

    /// The last day of the quarter (Taiwanese companies all report on the
    /// calendar year).
    pub fn period_end(&self) -> chrono::NaiveDate {
        quarter_end(self.year, self.quarter)
    }

    /// When the report was due: 45 days after the quarter (Q1: May 15; Q2:
    /// August 14; Q3: November 14), the annual one by March 31. MOPS's page
    /// doesn't say when a report was filed, so this stands in for it, as the
    /// date the market had it by.
    pub fn due(&self) -> chrono::NaiveDate {
        use chrono::NaiveDate;
        match self.quarter {
            1 => NaiveDate::from_ymd_opt(self.year, 5, 15),
            2 => NaiveDate::from_ymd_opt(self.year, 8, 14),
            3 => NaiveDate::from_ymd_opt(self.year, 11, 14),
            _ => NaiveDate::from_ymd_opt(self.year + 1, 3, 31),
        }.unwrap()
    }
}

pub fn quarter_end(year: i32, quarter: u32) -> chrono::NaiveDate {
    let (m, d) = match quarter { 1 => (3, 31), 2 => (6, 30), 3 => (9, 30), _ => (12, 31) };
    chrono::NaiveDate::from_ymd_opt(year, m, d).unwrap()
}

/// The MOPS company code for a ticker: "2330.TW" or "2330" is "2330".
pub fn code(ticker: &str) -> String {
    ticker.trim().trim_end_matches(".TW").trim_end_matches(".tw").to_string()
}

/// The cache, read only.
#[derive(Debug, Clone)]
pub struct Cache {
    dir: PathBuf,
}

impl Cache {
    pub fn new(dir: impl Into<PathBuf>) -> Cache {
        Cache { dir: dir.into() }
    }

    /// `cache/` next to this crate's manifest; `MOPS_CACHE` names another.
    pub fn default_dir() -> PathBuf {
        std::env::var_os("MOPS_CACHE").map(PathBuf::from)
            .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("cache"))
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    pub fn path(&self, code: &str, year: i32, quarter: u32, ext: &str) -> PathBuf {
        self.dir.join(code).join(format!("{}Q{}.{}", year, quarter, ext))
    }

    /// The company's cached reports, oldest first.
    pub fn reports(&self, ticker: &str) -> Result<Vec<Report>, Box<dyn Error>> {
        let code = code(ticker);
        let dir = self.dir.join(&code);
        let entries = fs::read_dir(&dir).map_err(|e| format!("{}: {} (run mops-fetch first)", dir.display(), e))?;
        let mut reports: Vec<Report> = entries.flatten().filter_map(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            let stem = name.strip_suffix(".html")?;
            let (y, q) = stem.split_once('Q')?;
            Some(Report { code: code.clone(), year: y.parse().ok()?, quarter: q.parse().ok()?, path: e.path() })
        }).collect();
        reports.sort();
        Ok(reports)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dates() {
        assert_eq!(code("2330.TW"), "2330");
        let r = Report { code: "2330".into(), year: 2025, quarter: 4, path: PathBuf::new() };
        assert_eq!(r.id(), "MOPS-2330-2025Q4");
        assert_eq!(r.period_end().to_string(), "2025-12-31");
        assert_eq!(r.due().to_string(), "2026-03-31");
    }
}
