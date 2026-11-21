//! A small client for SEC EDGAR: look up a company's CIK by ticker, list its
//! 10-Q/10-K filings, and download the pages `filing::Reader` reads.
//!
//! Filings never change once published, so their pages are cached on disk
//! under `<cache>/<cik>/<accession>/` and only fetched once. The filing list
//! (`<cache>/<cik>/submissions.json`) and ticker map
//! (`<cache>/company_tickers.json`) do change, and are fetched again once
//! their copies are older than `SUBMISSIONS_MAX_AGE` and `TICKERS_MAX_AGE`.
//! Delete them to fetch them now.
//!
//! With `bulk_submissions`, filing lists come instead from EDGAR's nightly
//! `submissions.zip` (every filer's, about 1.6 GB), saved as
//! `<cache>/submissions.zip` and fetched again when older than
//! `SUBMISSIONS_MAX_AGE`: one request rather than one or more per company,
//! which pays off across thousands of companies. A company missing from the
//! zip (its first filing since it was built) is fetched on its own.
//!
//! The SEC asks automated clients to send a User-Agent naming who they are
//! and to stay under 10 requests per second.

use std::cell::{Cell, RefCell};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::{Duration, Instant};

use chrono::NaiveDate;
use serde::Deserialize;

use crate::filing;

const MIN_INTERVAL: Duration = Duration::from_millis(125);

/// Under a day, so a nightly run sees the day's new filings, while a second
/// run the same day reads everything from disk.
const SUBMISSIONS_MAX_AGE: Duration = Duration::from_secs(20 * 3600);
/// Tickers change rarely, and a ticker missing from the copy fetches it again.
const TICKERS_MAX_AGE: Duration = Duration::from_secs(7 * 24 * 3600);

/// Written in a filing's cache directory when EDGAR has no `FilingSummary.xml`
/// for it (filings before XBRL), so it is not requested again.
const NO_SUMMARY: &str = "FilingSummary.missing";

pub struct Client {
    http: reqwest::blocking::Client,
    cache: PathBuf,
    last_request: Cell<Option<Instant>>,
    bulk: bool,
    /// The opened `submissions.zip`, once `bulk` has needed it.
    archive: RefCell<Option<zip::ZipArchive<fs::File>>>,
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
    #[serde(rename = "filingCount")]
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
            bulk: false,
            archive: RefCell::new(None),
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

    /// Read filing lists from EDGAR's nightly `submissions.zip`.
    pub fn bulk_submissions(mut self) -> Self {
        self.bulk = true;
        self
    }

    /// `name` from `submissions.zip`, downloading the zip first when there is
    /// no copy younger than `SUBMISSIONS_MAX_AGE`. `None` when the zip has
    /// no such entry.
    fn bulk_json<T: serde::de::DeserializeOwned>(&self, name: &str) -> Result<Option<T>, Box<dyn Error>> {
        let mut archive = self.archive.borrow_mut();
        if archive.is_none() {
            const URL: &str = "https://www.sec.gov/Archives/edgar/daily-index/bulkdata/submissions.zip";
            let path = self.cache.join("submissions.zip");
            let fresh = fs::metadata(&path).and_then(|m| m.modified()).ok()
                .is_some_and(|t| t.elapsed().unwrap_or_default() < SUBMISSIONS_MAX_AGE);
            if !fresh {
                eprintln!("downloading {}", URL);
                self.download(URL, &path)?;
            }
            *archive = Some(zip::ZipArchive::new(fs::File::open(&path)?)?);
        }
        let archive = archive.as_mut().unwrap();
        let entry = match archive.by_name(name) {
            Ok(entry) => entry,
            Err(zip::result::ZipError::FileNotFound) => return Ok(None),
            Err(e) => return Err(e.into()),
        };
        Ok(Some(serde_json::from_reader(std::io::BufReader::new(entry))?))
    }

    /// Stream a large file to `path`, through a temporary file so an
    /// interrupted download is never taken for a complete one.
    fn download(&self, url: &str, path: &Path) -> Result<(), Box<dyn Error>> {
        if let Some(last) = self.last_request.get() {
            sleep(MIN_INTERVAL.saturating_sub(last.elapsed()));
        }
        self.last_request.set(Some(Instant::now()));
        // The client's default 30-second timeout is far too short for 1.6 GB.
        let mut res = self.http.get(url).timeout(Duration::from_secs(3600)).send()?.error_for_status()?;
        if let Some(dir) = path.parent() { fs::create_dir_all(dir)?; }
        let tmp = path.with_extension("partial");
        let mut file = fs::File::create(&tmp)?;
        std::io::copy(&mut res, &mut file)?;
        file.sync_all()?;
        fs::rename(&tmp, path)?;
        Ok(())
    }

    /// `url` as JSON, from its copy at `path` while that is younger than
    /// `max_age` (`None`: any copy) and parses, else fetched and saved there.
    /// When the fetch fails, an older copy is better than none.
    fn cached_json<T: serde::de::DeserializeOwned>(&self, url: &str, path: &Path, max_age: Option<Duration>) -> Result<T, Box<dyn Error>> {
        let age = fs::metadata(path).and_then(|m| m.modified()).ok()
            .map(|t| t.elapsed().unwrap_or_default());
        let read = || -> Option<T> { serde_json::from_slice(&fs::read(path).ok()?).ok() };
        if age.is_some_and(|a| max_age.is_none_or(|m| a < m)) {
            if let Some(v) = read() { return Ok(v); }
        }
        let fetched = self.get(url).and_then(|r| Ok(r.error_for_status()?.bytes()?));
        match fetched {
            Ok(bytes) => {
                let v = serde_json::from_slice(&bytes)?;
                if let Some(dir) = path.parent() { fs::create_dir_all(dir)?; }
                write_atomic(path, &bytes)?;
                Ok(v)
            },
            Err(e) => match read() {
                Some(v) => { eprintln!("{}: {}; using the copy on disk", url, e); Ok(v) },
                None => Err(e),
            },
        }
    }

    /// The company's Central Index Key, from the SEC's ticker map.
    pub fn cik(&self, ticker: &str) -> Result<u64, Box<dyn Error>> {
        const URL: &str = "https://www.sec.gov/files/company_tickers.json";
        let path = self.cache.join("company_tickers.json");
        let find = |map: std::collections::HashMap<String, TickerEntry>| map.into_values()
            // Class shares: "BF.B" in most lists, "BF-B" at the SEC.
            .find(|e| e.ticker.replace('.', "-").eq_ignore_ascii_case(&ticker.replace('.', "-")))
            .map(|e| e.cik_str);
        if let Some(cik) = find(self.cached_json(URL, &path, Some(TICKERS_MAX_AGE))?) {
            return Ok(cik);
        }
        // Listed since the copy was saved: fetch the map again, once.
        find(self.cached_json(URL, &path, Some(Duration::ZERO))?)
            .ok_or_else(|| EdgarError::TickerNotFound(ticker.to_string()).into())
    }

    /// The company's filings of the given forms, oldest first, including the
    /// older history EDGAR pages out of the main submissions file.
    pub fn filings(&self, cik: u64, forms: &[&str]) -> Result<Vec<Filing>, Box<dyn Error>> {
        let dir = self.cache.join(cik.to_string());
        let main = format!("CIK{:010}.json", cik);
        if self.bulk {
            if let Some(subs) = self.bulk_json::<Submissions>(&main)? {
                let mut pages = vec![subs.filings.recent];
                for file in &subs.filings.files {
                    pages.push(self.bulk_json(&file.name)?
                        .ok_or_else(|| format!("{} not in submissions.zip", file.name))?);
                }
                return Ok(filings_of(cik, pages, forms));
            }
        }
        let subs: Submissions = self.cached_json(
            &format!("https://data.sec.gov/submissions/{}", main),
            &dir.join("submissions.json"), Some(SUBMISSIONS_MAX_AGE))?;

        // Older history, paged out of the main file. A page's copy is good as
        // long as it holds as many filings as the main file says it does.
        let mut pages = vec![subs.filings.recent];
        for file in &subs.filings.files {
            let path = dir.join(&file.name);
            let page: FilingColumns = self.cached_json(&format!("https://data.sec.gov/submissions/{}", file.name), &path, None)?;
            pages.push(if page.form.len() == file.filing_count { page } else {
                self.cached_json(&format!("https://data.sec.gov/submissions/{}", file.name), &path, Some(Duration::ZERO))?
            });
        }

        Ok(filings_of(cik, pages, forms))
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

/// partial file that later runs would treat as cached.
fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    let tmp = path.with_extension("part");
    fs::write(&tmp, bytes)?;
    fs::rename(&tmp, path)?;
    Ok(())
}
