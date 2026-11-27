//! A small client for SEC EDGAR: look up a company's CIK by ticker, list its
//! 10-Q/10-K filings, and download the pages `filing::Reader` reads.
//!
//! Filings never change once published, so their pages are cached on disk
//! under `<cache>/<cik>/<accession>/` and only fetched once. The filing list
//! and ticker map do change and are fetched on every run.
//!
//! The SEC asks automated clients to send a User-Agent naming who they are
//! and to stay under 10 requests per second.

use std::cell::Cell;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::{Duration, Instant};

use chrono::NaiveDate;
use serde::Deserialize;

use crate::filing;

const MIN_INTERVAL: Duration = Duration::from_millis(125);

/// Written in a filing's cache directory when EDGAR has no `FilingSummary.xml`
/// for it (filings before XBRL), so it is not requested again.
const NO_SUMMARY: &str = "FilingSummary.missing";

pub struct Client {
    http: reqwest::blocking::Client,
    cache: PathBuf,
    last_request: Cell<Option<Instant>>,
}

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

    fn url(&self, file: &str) -> String {
        format!("https://www.sec.gov/Archives/edgar/data/{}/{}/{}", self.cik, self.accession_dir(), file)
    }
}

#[derive(Debug)]
pub enum EdgarError {
    /// The filing predates EDGAR's rendered HTML statement pages: it has no
    /// `FilingSummary.xml`, or one listing only the older `R*.xml` renderings.
    NoFilingSummary,
    /// The summary lists no page recognized as this statement.
    StatementNotFound(&'static str),
    TickerNotFound(String),
}

impl std::fmt::Display for EdgarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EdgarError::NoFilingSummary => write!(f, "no rendered HTML statements"),
            EdgarError::StatementNotFound(s) => write!(f, "no {} page in FilingSummary.xml", s),
            EdgarError::TickerNotFound(t) => write!(f, "ticker {} not found", t),
        }
    }
}

impl Error for EdgarError {}

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

#[derive(Deserialize)]
struct TickerEntry {
    cik_str: u64,
    ticker: String,
}

impl Client {
    /// `user_agent` should identify the caller, e.g. "Name email@example.com".
    pub fn new(user_agent: &str, cache: impl Into<PathBuf>) -> Result<Client, Box<dyn Error>> {
        Ok(Client {
            http: reqwest::blocking::Client::builder().user_agent(user_agent).build()?,
            cache: cache.into(),
            last_request: Cell::new(None),
        })
    }

    /// GET with the SEC's rate limit, retrying server errors (EDGAR returns
    /// occasional 502s) with a growing delay.
    fn get(&self, url: &str) -> Result<reqwest::blocking::Response, Box<dyn Error>> {
        let mut attempt = 0;
        loop {
            if let Some(last) = self.last_request.get() {
                let elapsed = last.elapsed();
                if elapsed < MIN_INTERVAL {
                    sleep(MIN_INTERVAL - elapsed);
                }
            }
            self.last_request.set(Some(Instant::now()));
            let res = self.http.get(url).send();
            let retry = match &res {
                Ok(r) => r.status().is_server_error(),
                Err(e) => e.is_timeout() || e.is_connect(),
            };
            attempt += 1;
            if !retry || attempt == 4 {
                return Ok(res?);
            }
            sleep(Duration::from_secs(1 << attempt));
        }
    }

    fn get_json<T: serde::de::DeserializeOwned>(&self, url: &str) -> Result<T, Box<dyn Error>> {
        Ok(self.get(url)?.error_for_status()?.json()?)
    }

    /// The company's Central Index Key, from the SEC's ticker map.
    pub fn cik(&self, ticker: &str) -> Result<u64, Box<dyn Error>> {
        let map: std::collections::HashMap<String, TickerEntry> =
            self.get_json("https://www.sec.gov/files/company_tickers.json")?;
        map.into_values()
            .find(|e| e.ticker.eq_ignore_ascii_case(ticker))
            .map(|e| e.cik_str)
            .ok_or_else(|| EdgarError::TickerNotFound(ticker.to_string()).into())
    }

    /// The company's filings of the given forms, oldest first, including the
    /// older history EDGAR pages out of the main submissions file.
    pub fn filings(&self, cik: u64, forms: &[&str]) -> Result<Vec<Filing>, Box<dyn Error>> {
        let subs: Submissions = self.get_json(&format!("https://data.sec.gov/submissions/CIK{:010}.json", cik))?;

        let mut pages = vec![subs.filings.recent];
        for file in &subs.filings.files {
            pages.push(self.get_json(&format!("https://data.sec.gov/submissions/{}", file.name))?);
        }

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
        Ok(filings)
    }

    /// Download the filing's `FilingSummary.xml` and its balance sheet, income
    /// statement and cash flow statement pages into the cache (skipping files already there)
    /// and return the directory, ready for `filing::new_reader`.
    pub fn fetch_filing(&self, f: &Filing) -> Result<PathBuf, Box<dyn Error>> {
        let dir = self.cache.join(f.cik.to_string()).join(f.accession_dir());
        fs::create_dir_all(&dir)?;
        if dir.join(NO_SUMMARY).exists() {
            return Err(EdgarError::NoFilingSummary.into());
        }

        let summary_path = dir.join("FilingSummary.xml");
        if !summary_path.exists() {
            let res = self.get(&f.url("FilingSummary.xml"))?;
            if res.status() == reqwest::StatusCode::NOT_FOUND {
                fs::write(dir.join(NO_SUMMARY), "")?;
                return Err(EdgarError::NoFilingSummary.into());
            }
            write_atomic(&summary_path, &res.error_for_status()?.bytes()?)?;
        }

        let reports = filing::parse_filing_summary(&fs::read_to_string(&summary_path)?);
        if reports.is_empty() {
            fs::write(dir.join(NO_SUMMARY), "")?;
            return Err(EdgarError::NoFilingSummary.into());
        }
        let pages = [
            ("balance sheet", filing::balance_sheet(&reports)),
            ("income statement", filing::income_statement(&reports)),
            ("cash flow statement", filing::cash_flow_statement(&reports)),
        ];
        for (what, report) in pages {
            let report = report.ok_or(EdgarError::StatementNotFound(what))?;
            let path = dir.join(&report.file);
            if !path.exists() {
                let bytes = self.get(&f.url(&report.file))?.error_for_status()?.bytes()?;
                write_atomic(&path, &bytes)?;
            }
        }
        Ok(dir)
    }
}

/// Write via a temporary file so an interrupted download never leaves a
/// partial file that later runs would treat as cached.
fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    let tmp = path.with_extension("part");
    fs::write(&tmp, bytes)?;
    fs::rename(&tmp, path)?;
    Ok(())
}
