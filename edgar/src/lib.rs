//! SEC EDGAR as a source, the way `gutenberg` is one for books: companies'
//! filing lists and the statement pages of their 10-Q, 10-K and 20-F filings,
//! kept on disk under `cache/`:
//!
//! ```text
//! cache/company_tickers.json           ticker -> CIK
//! cache/<cik>/submissions.json         the company's filings
//! cache/<cik>/CIK…-submissions-001.json   older filings, paged out
//! cache/<cik>/<accession>/FilingSummary.xml
//! cache/<cik>/<accession>/R2.htm …     the statement pages
//! ```
//!
//! `Cache` reads this and never touches the network; it is all a reader of
//! filings (`equities`) needs. With the `fetch` feature, `Client` downloads
//! into it, and the `fetch` executable runs the client over a list of tickers.

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use chrono::NaiveDate;
use serde::Deserialize;

pub mod funds;
pub use funds::{Census, DataSet, FundClass, MonthlyReturn};

#[cfg(feature = "fetch")]
mod client;
#[cfg(feature = "fetch")]
pub use client::{Client, USER_AGENT};

/// The reports kept: foreign filers (TSMC) file annual 20-Fs instead of
/// 10-Ks, and no 10-Qs.
pub const FORMS: [&str; 3] = ["10-Q", "10-K", "20-F"];

/// Written in a filing's directory when EDGAR has no rendered statements for
/// it (filings before XBRL), so it is not requested again.
const NO_SUMMARY: &str = "FilingSummary.missing";

#[derive(Debug, Clone)]
pub struct Filing {
    pub cik: u64,
    /// e.g. "0001564590-20-047486"
    pub accession: String,
    pub form: String,
    pub report_date: NaiveDate,
    pub filing_date: NaiveDate,
}

impl Filing {
    fn accession_dir(&self) -> String {
        self.accession.replace('-', "")
    }
}

#[derive(Debug)]
pub enum EdgarError {
    /// The filing predates EDGAR's rendered HTML statement pages: it has no
    /// `FilingSummary.xml`, or one listing only the older `R*.xml` renderings.
    NoFilingSummary,
    TickerNotFound(String),
    /// Not in the cache: run `fetch` first.
    NotFetched(PathBuf),
}

impl std::fmt::Display for EdgarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EdgarError::NoFilingSummary => write!(f, "no rendered HTML statements"),
            EdgarError::TickerNotFound(t) => write!(f, "ticker {} not found", t),
            EdgarError::NotFetched(p) => write!(f, "not fetched yet: {}", p.display()),
        }
    }
}

impl Error for EdgarError {}

/// One page listed in `FilingSummary.xml`.
#[derive(Debug, Clone)]
pub struct Report {
    pub file: String,
    pub short_name: String,
    /// "Cover", "Statements", "Details", "Notes", ...; absent in older summaries.
    pub category: Option<String>,
    /// EDGAR's own classification, from the long name ("1003000 - Statement -
    /// Consolidated Balance Sheets"): "Document" (the cover), "Statement" or
    /// "Disclosure" (the notes).
    pub kind: Option<String>,
}

/// The pages listed in `FilingSummary.xml`, in filing order (cover page, then
/// statements, then notes).
pub fn parse_filing_summary(xml: &str) -> Vec<Report> {
    let mut reports = Vec::new();
    let mut rest = xml;
    while let Some(start) = rest.find("<Report ").or_else(|| rest.find("<Report>")) {
        let Some(len) = rest[start..].find("</Report>") else { break };
        let block = &rest[start..start + len];
        if let Some(file) = tag(block, "HtmlFileName") {
            reports.push(Report {
                file,
                short_name: tag(block, "ShortName").unwrap_or_default(),
                category: tag(block, "MenuCategory"),
                kind: tag(block, "LongName").and_then(|n| Some(n.split(" - ").nth(1)?.trim().to_string())),
            });
        }
        rest = &rest[start + len..];
    }
    reports
}

fn tag(block: &str, name: &str) -> Option<String> {
    let open = format!("<{}>", name);
    let close = format!("</{}>", name);
    let start = block.find(&open)? + open.len();
    let end = block[start..].find(&close)? + start;
    Some(block[start..end].trim().to_string())
}

/// The pages kept from a filing: its cover and statements, by EDGAR's
/// classification of each page (`Report::kind`), wherever it is listed: some
/// filers list statements after their notes. Without one, by category (early
/// 2022 summaries put some statements under "Uncategorized"); without either,
/// every page.
pub fn statement_pages(reports: &[Report]) -> Vec<&Report> {
    const NOTES: [&str; 4] = ["Notes", "Details", "Policies", "Tables"];
    reports.iter()
        .filter(|r| match (&r.kind, &r.category) {
            (Some(kind), _) => kind == "Document" || kind == "Statement",
            (None, Some(c)) => !NOTES.contains(&c.as_str()),
            (None, None) => true,
        })
        .collect()
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
    /// directory a program runs in; `EDGAR_CACHE` names another (a worktree
    /// sharing the main checkout's).
    pub fn default_dir() -> PathBuf {
        std::env::var_os("EDGAR_CACHE").map(PathBuf::from)
            .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("cache"))
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    fn tickers_path(&self) -> PathBuf {
        self.dir.join("company_tickers.json")
    }

    fn company_dir(&self, cik: u64) -> PathBuf {
        self.dir.join(cik.to_string())
    }

    pub fn filing_path(&self, f: &Filing) -> PathBuf {
        self.company_dir(f.cik).join(f.accession_dir())
    }

    /// Whether any of the company's filings got pages after `since`: a
    /// filing's directory changes when a page is written into it. (The
    /// filing lists, rewritten every night, are not counted.)
    pub fn changed_since(&self, cik: u64, since: std::time::SystemTime) -> bool {
        let Ok(entries) = fs::read_dir(self.company_dir(cik)) else { return false };
        entries.flatten().any(|e| e.metadata().is_ok_and(|m| m.is_dir() && m.modified().is_ok_and(|t| t > since)))
    }

    /// The company's name and where its shares trade, from its filing list.
    pub fn company(&self, cik: u64) -> Result<Company, Box<dyn Error>> {
        read_json(&self.company_dir(cik).join("submissions.json"))
    }

    /// The company's Central Index Key, from the SEC's ticker map.
    /// `ticker` can also be a CIK, written `CIK1018724`: a company without a
    /// ticker (acquired, bankrupt, delisted, or never listed), which the
    /// ticker map doesn't have.
    pub fn cik(&self, ticker: &str) -> Result<u64, Box<dyn Error>> {
        if let Some(cik) = parse_cik(ticker) {
            return Ok(cik);
        }
        let path = self.tickers_path();
        let map = read_json(&path)?;
        find_cik(&map, ticker).ok_or_else(|| EdgarError::TickerNotFound(ticker.to_string()).into())
    }

    /// The company's filings of the given forms, oldest first.
    pub fn filings(&self, cik: u64, forms: &[&str]) -> Result<Vec<Filing>, Box<dyn Error>> {
        let dir = self.company_dir(cik);
        let subs: Submissions = read_json(&dir.join("submissions.json"))?;
        let mut pages = vec![subs.filings.recent];
        for file in &subs.filings.files {
            pages.push(read_json(&dir.join(&file.name))?);
        }
        Ok(filings_of(cik, pages, forms))
    }

    /// The directory holding the filing's `FilingSummary.xml` and pages.
    pub fn filing(&self, f: &Filing) -> Result<PathBuf, Box<dyn Error>> {
        let dir = self.filing_path(f);
        if dir.join(NO_SUMMARY).exists() {
            return Err(EdgarError::NoFilingSummary.into());
        }
        if !dir.join("FilingSummary.xml").exists() {
            return Err(EdgarError::NotFetched(dir).into());
        }
        Ok(dir)
    }
}

fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, Box<dyn Error>> {
    match fs::read(path) {
        Ok(bytes) => Ok(serde_json::from_slice(&bytes)?),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(EdgarError::NotFetched(path.to_path_buf()).into()),
        Err(e) => Err(e.into()),
    }
}

#[derive(Deserialize)]
struct TickerEntry {
    cik_str: u64,
    ticker: String,
}

type TickerMap = std::collections::HashMap<String, TickerEntry>;

/// The CIK for `ticker`. Class shares are "BF.B" in most lists, "BF-B" at the
/// SEC. A company listed only by its classes (Berkshire Hathaway: BRK-A,
/// BRK-B) is found by the bare symbol, "BRK", when all classes share one CIK.
/// "CIK1018724" or "CIK0001018724" (any case): the number.
pub fn parse_cik(s: &str) -> Option<u64> {
    let digits = s.get(..3).filter(|p| p.eq_ignore_ascii_case("CIK")).map(|_| &s[3..])?;
    if digits.is_empty() || !digits.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    digits.parse().ok()
}

fn find_cik(map: &TickerMap, ticker: &str) -> Option<u64> {
    let want = ticker.replace('.', "-");
    if let Some(e) = map.values().find(|e| e.ticker.replace('.', "-").eq_ignore_ascii_case(&want)) {
        return Some(e.cik_str);
    }
    let prefix = format!("{}-", want.to_uppercase());
    let mut ciks = map.values().filter(|e| e.ticker.to_uppercase().starts_with(&prefix)).map(|e| e.cik_str);
    let first = ciks.next()?;
    ciks.all(|c| c == first).then_some(first)
}

/// A company as its EDGAR filing list describes it.
#[derive(Clone, Debug, Deserialize)]
pub struct Company {
    /// "Apple Inc."
    pub name: String,
    #[serde(default)]
    pub tickers: Vec<String>,
    /// "Nasdaq", "NYSE", one per ticker.
    #[serde(default)]
    pub exchanges: Vec<Option<String>>,
}

#[derive(Deserialize)]
struct Submissions {
    filings: SubmissionFilings,
}

#[derive(Deserialize)]
struct SubmissionFilings {
    recent: FilingColumns,
    #[serde(default)]
    files: Vec<SubmissionFile>,
}

#[derive(Deserialize)]
struct SubmissionFile {
    name: String,
    /// How many filings the page holds, to tell a stale copy of it.
    #[serde(rename = "filingCount")]
    #[cfg_attr(not(feature = "fetch"), allow(dead_code))]
    filing_count: usize,
}

/// EDGAR lists filings column-wise: the i-th entry of each vector is filing i.
#[derive(Deserialize)]
struct FilingColumns {
    #[serde(rename = "accessionNumber")]
    accession_number: Vec<String>,
    form: Vec<String>,
    #[serde(rename = "reportDate")]
    report_date: Vec<String>,
    #[serde(rename = "filingDate")]
    filing_date: Vec<String>,
}

/// The given forms among a company's filings, oldest first: EDGAR lists
/// them in pages, newest first.
fn filings_of(cik: u64, pages: Vec<FilingColumns>, forms: &[&str]) -> Vec<Filing> {
    let mut filings = Vec::new();
    for page in pages {
        for i in 0..page.form.len() {
            if !forms.contains(&page.form[i].as_str()) {
                continue;
            }
            let (Ok(report_date), Ok(filing_date)) = (
                NaiveDate::parse_from_str(&page.report_date[i], "%Y-%m-%d"),
                NaiveDate::parse_from_str(&page.filing_date[i], "%Y-%m-%d"),
            ) else {
                continue;
            };
            filings.push(Filing {
                cik,
                accession: page.accession_number[i].clone(),
                form: page.form[i].clone(),
                report_date,
                filing_date,
            });
        }
    }
    filings.sort_by_key(|f| (f.filing_date, f.accession.clone()));
    filings.dedup_by(|a, b| a.accession == b.accession);
    filings
}

#[cfg(test)]
mod tests {
    #[test]
    fn ciks() {
        assert_eq!(super::parse_cik("CIK1018724"), Some(1018724));
        assert_eq!(super::parse_cik("cik0001018724"), Some(1018724));
        assert_eq!(super::parse_cik("CIEN"), None);
        assert_eq!(super::parse_cik("CIK"), None);
    }

    use super::*;

    fn report(short_name: &str, category: Option<&str>, kind: Option<&str>) -> Report {
        Report { file: String::new(), short_name: short_name.into(), category: category.map(Into::into), kind: kind.map(Into::into) }
    }

    #[test]
    fn statement_pages_by_kind_then_category() {
        let reports = [
            report("Document and Entity Information", None, Some("Document")),
            report("Statement of Financial Condition", None, Some("Statement")),
            report("Organization", None, Some("Disclosure")),
            report("Organization (Details)", None, Some("Disclosure")),
            // Listed after the notes.
            report("Consolidated Statements of Operations", None, Some("Statement")),
            report("Audit Information", Some("Notes"), None),
            report("Consolidated Balance Sheets", Some("Uncategorized"), None),
            report("Leases (Details)", Some("Details"), None),
        ];
        let kept: Vec<_> = statement_pages(&reports).iter().map(|r| r.short_name.as_str()).collect();
        assert_eq!(kept, ["Document and Entity Information", "Statement of Financial Condition",
            "Consolidated Statements of Operations", "Consolidated Balance Sheets"]);
    }

    #[test]
    fn class_shares() {
        let map: TickerMap = serde_json::from_str(r#"{
            "0": {"cik_str": 1067983, "ticker": "BRK-B", "title": "BERKSHIRE HATHAWAY INC"},
            "1": {"cik_str": 1067983, "ticker": "BRK-A", "title": "BERKSHIRE HATHAWAY INC"},
            "2": {"cik_str": 14693, "ticker": "BF-B", "title": "BROWN FORMAN CORP"}
        }"#).unwrap();
        assert_eq!(find_cik(&map, "BRK"), Some(1067983));
        assert_eq!(find_cik(&map, "brk.b"), Some(1067983));
        assert_eq!(find_cik(&map, "BF.B"), Some(14693));
        assert_eq!(find_cik(&map, "BF"), Some(14693));
        assert_eq!(find_cik(&map, "XYZQ"), None);
    }
}

/// Tickers from a list file: separated by spaces, commas or newlines, with
/// `#` starting a comment.
pub fn read_list(path: impl AsRef<Path>) -> std::io::Result<Vec<String>> {
    Ok(fs::read_to_string(path)?.lines()
        .flat_map(|line| line.split('#').next().unwrap_or("").split([' ', '\t', ','])
            .filter(|t| !t.is_empty()).map(str::to_string).collect::<Vec<_>>())
        .collect())
}
