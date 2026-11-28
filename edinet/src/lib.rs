//! EDINET (Electronic Disclosure for Investors' NETwork), Japan's EDGAR, as a
//! source: the annual, semiannual and quarterly reports of Japanese listed
//! companies, as the XBRL-to-CSV files EDINET publishes, kept under `cache/`:
//!
//! ```text
//! cache/documents/<yyyy>/<yyyy-mm-dd>.json        every document filed that day
//! cache/<EDINET code>/<docID>/XBRL_TO_CSV/*.csv  one filing's facts
//! ```
//!
//! EDINET lists documents by day, not by company, so a company's filings are
//! found by reading the day lists. Past days do not change; the client
//! fetches each once (and the last few days again, for late changes).
//!
//! The documents (Hepburn readings, for those of us who read kanji slowly):
//!
//! | code | document                                          |
//! |------|---------------------------------------------------|
//! | 120  | 有価証券報告書 (yūka shōken hōkokusho), annual     |
//! | 130  | 訂正有価証券報告書 (teisei …), its amendment      |
//! | 140  | 四半期報告書 (shihanki hōkokusho), quarterly, to 2024 |
//! | 150  | 訂正四半期報告書, its amendment                   |
//! | 160  | 半期報告書 (hanki hōkokusho), semiannual          |
//! | 170  | 訂正半期報告書, its amendment                     |
//!
//! `Cache` reads all this and never touches the network. With the `fetch`
//! feature, `Client` downloads into it (it needs an API key, in
//! `EDINET_API_KEY`), and `edinet-fetch` runs it.

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use chrono::NaiveDate;
use serde::Deserialize;

/// Reading a report's CSV files into equities' items: `report::read_csv`,
/// and `report::EdinetReport`, which implements equities' `Statements`.
pub mod report;

#[cfg(feature = "fetch")]
mod client;
#[cfg(feature = "fetch")]
pub use client::Client;

/// The reports kept: annual, quarterly and semiannual, and their amendments.
pub const DOC_TYPES: [&str; 6] = ["120", "130", "140", "150", "160", "170"];

/// A document as EDINET lists it.
#[derive(Debug, Clone, Deserialize)]
pub struct Document {
    #[serde(rename = "docID")]
    pub doc_id: String,
    /// The filer's EDINET code, e.g. "E02144" (Toyota).
    #[serde(rename = "edinetCode")]
    pub edinet_code: Option<String>,
    /// The securities code with its check digit, e.g. "72030" for 7203.
    #[serde(rename = "secCode")]
    pub sec_code: Option<String>,
    /// e.g. "トヨタ自動車株式会社" (Toyota Jidōsha Kabushiki-gaisha).
    #[serde(rename = "filerName")]
    pub filer_name: Option<String>,
    #[serde(rename = "docTypeCode")]
    pub doc_type: Option<String>,
    #[serde(rename = "periodStart")]
    pub period_start: Option<NaiveDate>,
    #[serde(rename = "periodEnd")]
    pub period_end: Option<NaiveDate>,
    /// "2025-06-18 09:00", Japan time.
    #[serde(rename = "submitDateTime")]
    pub submitted: Option<String>,
    /// e.g. "有価証券報告書－第121期(2024/04/01－2025/03/31)", the 121st term.
    #[serde(rename = "docDescription")]
    pub description: Option<String>,
    /// "1" when the XBRL facts are available as CSV.
    #[serde(rename = "csvFlag")]
    pub csv_flag: Option<String>,
    /// "1" when withdrawn (取下げ, torisage).
    #[serde(rename = "withdrawalStatus")]
    pub withdrawal: Option<String>,
}

impl Document {
    /// A report this crate keeps, with its facts as CSV, not withdrawn.
    pub fn is_report(&self) -> bool {
        self.doc_type.as_deref().is_some_and(|t| DOC_TYPES.contains(&t))
            && self.csv_flag.as_deref() == Some("1")
            && self.withdrawal.as_deref() != Some("1")
            && self.edinet_code.is_some()
    }
}

#[derive(Debug)]
pub enum EdinetError {
    /// Not in the cache: run `edinet-fetch` first.
    NotFetched(PathBuf),
    CompanyNotFound(String),
    /// The API refused, with its status and message.
    Api(String, String),
}

impl std::fmt::Display for EdinetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EdinetError::NotFetched(p) => write!(f, "not fetched yet: {}", p.display()),
            EdinetError::CompanyNotFound(c) => write!(f, "no reports found for {}", c),
            EdinetError::Api(status, message) => write!(f, "EDINET API {}: {}", status, message),
        }
    }
}

impl Error for EdinetError {}

/// A day's list, as the API returns it.
#[derive(Deserialize)]
struct DayList {
    #[serde(default)]
    results: Vec<Document>,
}

/// The securities code EDINET lists for a Tokyo ticker: "7203.T" or "7203"
/// is "72030" (the code and a check digit, always 0 in these lists); newer
/// alphanumeric codes ("130A") likewise.
pub fn sec_code(ticker: &str) -> String {
    let code = ticker.trim().trim_end_matches(".T").trim_end_matches(".t").to_uppercase();
    if code.len() == 4 { format!("{}0", code) } else { code }
}

/// The Tokyo ticker for a securities code: "72030" is "7203.T".
pub fn ticker(sec_code: &str) -> String {
    let code = sec_code.trim();
    let code = if code.len() == 5 && code.ends_with('0') { &code[..4] } else { code };
    format!("{}.T", code)
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

    /// `cache/` next to this crate's manifest, the one cache whatever
    /// directory a program runs in; `EDINET_CACHE` names another.
    pub fn default_dir() -> PathBuf {
        std::env::var_os("EDINET_CACHE").map(PathBuf::from)
            .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("cache"))
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    #[cfg_attr(not(feature = "fetch"), allow(dead_code))]
    fn day_path(&self, date: NaiveDate) -> PathBuf {
        self.dir.join("documents").join(date.format("%Y").to_string()).join(format!("{}.json", date))
    }

    /// Where a document's CSV files go.
    pub fn document_dir(&self, d: &Document) -> PathBuf {
        self.dir.join(d.edinet_code.as_deref().unwrap_or("unknown")).join(&d.doc_id)
    }

    /// Every day list on disk, in date order.
    fn days(&self) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        let root = self.dir.join("documents");
        let mut days = Vec::new();
        let Ok(years) = fs::read_dir(&root) else { return Err(EdinetError::NotFetched(root).into()) };
        for year in years.flatten() {
            for day in fs::read_dir(year.path())?.flatten() {
                if day.path().extension().is_some_and(|e| e == "json") {
                    days.push(day.path());
                }
            }
        }
        days.sort();
        Ok(days)
    }

    /// The company's reports, oldest first, by Tokyo ticker ("7203.T"),
    /// securities code ("72030") or EDINET code ("E02144").
    pub fn reports(&self, company: &str) -> Result<Vec<Document>, Box<dyn Error>> {
        let sec = sec_code(company);
        let reports: Vec<Document> = self.all_reports()?.into_iter()
            .filter(|d| d.edinet_code.as_deref() == Some(company) || d.sec_code.as_deref() == Some(sec.as_str()))
            .collect();
        if reports.is_empty() {
            return Err(EdinetError::CompanyNotFound(company.to_string()).into());
        }
        Ok(reports)
    }

    /// Every company's reports, oldest first: one read of the day lists, for
    /// a caller that wants many companies.
    pub fn all_reports(&self) -> Result<Vec<Document>, Box<dyn Error>> {
        let mut reports = Vec::new();
        for day in self.days()? {
            let list: DayList = serde_json::from_slice(&fs::read(&day)?)?;
            reports.extend(list.results.into_iter().filter(Document::is_report));
        }
        reports.sort_by(|a, b| a.submitted.cmp(&b.submitted));
        let mut seen = std::collections::HashSet::new();
        reports.retain(|d| seen.insert(d.doc_id.clone()));
        Ok(reports)
    }

    /// The Tokyo tickers of every listed company with reports (those with a
    /// securities code), in code order. "00000" is a placeholder for a
    /// company about to list (e.g. クラサスケミカル, Kurasasu Kemikaru, in
    /// September 2026), not a code.
    pub fn listed(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let codes: std::collections::BTreeSet<String> = self.all_reports()?.into_iter()
            .filter_map(|d| d.sec_code).filter(|c| c != "00000").collect();
        Ok(codes.iter().map(|c| ticker(c)).collect())
    }

    /// The CSV files of a document, if fetched.
    pub fn csv_files(&self, d: &Document) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        let dir = self.document_dir(d).join("XBRL_TO_CSV");
        let Ok(entries) = fs::read_dir(&dir) else { return Err(EdinetError::NotFetched(dir).into()) };
        let mut files: Vec<PathBuf> = entries.flatten().map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|e| e == "csv"))
            .collect();
        files.sort();
        Ok(files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn securities_codes() {
        assert_eq!(sec_code("7203.T"), "72030");
        assert_eq!(sec_code("7203"), "72030");
        assert_eq!(sec_code("130a.T"), "130A0");
        assert_eq!(sec_code("72030"), "72030");
    }
}
