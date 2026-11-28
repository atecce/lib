//! Downloading into the cache.
//!
//! Filings never change once published, so a filing's pages are fetched once.
//! The filing lists and ticker map do change, and are fetched again once
//! their copies are older than `SUBMISSIONS_MAX_AGE` and `TICKERS_MAX_AGE`.
//!
//! With `bulk_submissions`, filing lists come instead from EDGAR's nightly
//! `submissions.zip` (every filer's, about 1.6 GB), kept as
//! `cache/submissions.zip`: one request rather than one or more per company,
//! which pays off across thousands of companies. Each company's lists are
//! copied out of it into the usual files, so the cache reads the same either
//! way. A company missing from the zip (its first filing since the zip was
//! built) is fetched on its own.
//!
//! The SEC asks automated clients to send a User-Agent naming who they are
//! and to stay under 10 requests per second. Its pages take a third to two
//! thirds of a second to arrive, so one request at a time manages only one
//! or two a second: the client is shared between threads (see the `fetch`
//! executable, which fetches several filings at once), and spaces the
//! *starts* of its requests `MIN_INTERVAL` apart across all of them.

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::{Cache, EdgarError, Filing, Submissions, FilingColumns, NO_SUMMARY};

/// Identifies this client to the SEC, as EDGAR's fair access policy requires.
pub const USER_AGENT: &str = "Alessandro Tecce root@atec.pub";

const MIN_INTERVAL: Duration = Duration::from_millis(125);

/// Under a day, so a nightly run sees the day's new filings, while a second
/// run the same day reads everything from disk.
const SUBMISSIONS_MAX_AGE: Duration = Duration::from_secs(20 * 3600);
/// Tickers change rarely, and a ticker missing from the copy fetches it again.
const TICKERS_MAX_AGE: Duration = Duration::from_secs(7 * 24 * 3600);

const TICKERS_URL: &str = "https://www.sec.gov/files/company_tickers.json";
const FUND_TICKERS_URL: &str = "https://www.sec.gov/files/company_tickers_mf.json";
const BULK_URL: &str = "https://www.sec.gov/Archives/edgar/daily-index/bulkdata/submissions.zip";

pub struct Client {
    http: reqwest::blocking::Client,
    cache: Cache,
    /// When the latest request was (or is due to be) sent.
    last_request: Mutex<Option<Instant>>,
    requests: AtomicUsize,
    bulk: bool,
    /// The opened `submissions.zip`, once `bulk` has needed it.
    archive: Mutex<Option<zip::ZipArchive<fs::File>>>,
}

impl Client {
    /// `user_agent` should identify the caller, e.g. "Name email@example.com".
    pub fn new(user_agent: &str, cache: Cache) -> Result<Client, Box<dyn Error>> {
        Ok(Client {
            http: reqwest::blocking::Client::builder().user_agent(user_agent).build()?,
            cache,
            last_request: Mutex::new(None),
            requests: AtomicUsize::new(0),
            bulk: false,
            archive: Mutex::new(None),
        })
    }

    /// Read filing lists from EDGAR's nightly `submissions.zip`.
    pub fn bulk_submissions(mut self) -> Self {
        self.bulk = true;
        self
    }

    pub fn cache(&self) -> &Cache {
        &self.cache
    }

    /// How many requests this client has sent.
    pub fn requests(&self) -> usize {
        self.requests.load(Ordering::Relaxed)
    }

    /// GET with the SEC's rate limit, retrying server errors (EDGAR returns
    /// occasional 502s) with a growing delay.
    fn get(&self, url: &str) -> Result<reqwest::blocking::Response, Box<dyn Error>> {
        let mut attempt = 0;
        loop {
            self.pace();
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

    /// Wait for this request's turn: `MIN_INTERVAL` after the one before,
    /// whichever thread sent it. Each caller books its slot under the lock
    /// and sleeps outside it.
    fn pace(&self) {
        let slot = {
            let mut last = self.last_request.lock().unwrap();
            let now = Instant::now();
            let slot = last.map_or(now, |l| (l + MIN_INTERVAL).max(now));
            *last = Some(slot);
            slot
        };
        sleep(slot.saturating_duration_since(Instant::now()));
        self.requests.fetch_add(1, Ordering::Relaxed);
    }

    /// Fetch `url` into `path` unless a copy younger than `max_age` (`None`:
    /// any copy) is there. When the fetch fails, an older copy is better
    /// than none.
    fn refresh(&self, url: &str, path: &Path, max_age: Option<Duration>) -> Result<(), Box<dyn Error>> {
        let age = fs::metadata(path).and_then(|m| m.modified()).ok()
            .map(|t| t.elapsed().unwrap_or_default());
        if age.is_some_and(|a| max_age.is_none_or(|m| a < m)) {
            return Ok(());
        }
        match self.get(url).and_then(|r| Ok(r.error_for_status()?.bytes()?)) {
            Ok(bytes) => {
                // Only JSON that parses replaces a copy.
                serde_json::from_slice::<serde_json::Value>(&bytes)?;
                write_atomic(path, &bytes)
            },
            Err(e) if age.is_some() => {
                eprintln!("{}: {}; keeping the copy on disk", url, e);
                Ok(())
            },
            Err(e) => Err(e),
        }
    }

    /// The company's Central Index Key, from the SEC's ticker map.
    pub fn cik(&self, ticker: &str) -> Result<u64, Box<dyn Error>> {
        let path = self.cache.tickers_path();
        self.refresh(TICKERS_URL, &path, Some(TICKERS_MAX_AGE))?;
        match self.cache.cik(ticker) {
            Err(e) if matches!(e.downcast_ref(), Some(EdgarError::TickerNotFound(_))) => {
                // Listed since the copy was saved: fetch the map again, once.
                self.refresh(TICKERS_URL, &path, Some(Duration::ZERO))?;
                self.cache.cik(ticker)
            },
            r => r,
        }
    }

    /// Bring the company's filing lists up to date, then list its filings of
    /// the given forms, oldest first.
    pub fn filings(&self, cik: u64, forms: &[&str]) -> Result<Vec<Filing>, Box<dyn Error>> {
        let dir = self.cache.company_dir(cik);
        fs::create_dir_all(&dir)?;
        let main = format!("CIK{:010}.json", cik);
        if !(self.bulk && self.from_bulk(&main, &dir)?) {
            self.refresh(&format!("https://data.sec.gov/submissions/{}", main),
                &dir.join("submissions.json"), Some(SUBMISSIONS_MAX_AGE))?;
            // Older history, paged out of the main file. A page's copy is
            // good as long as it holds as many filings as the main file says.
            let subs: Submissions = crate::read_json(&dir.join("submissions.json"))?;
            for file in &subs.filings.files {
                let path = dir.join(&file.name);
                let url = format!("https://data.sec.gov/submissions/{}", file.name);
                self.refresh(&url, &path, None)?;
                let page: FilingColumns = crate::read_json(&path)?;
                if page.form.len() != file.filing_count {
                    self.refresh(&url, &path, Some(Duration::ZERO))?;
                }
            }
        }
        self.cache.filings(cik, forms)
    }

    /// Copy the company's lists out of `submissions.zip` into `dir`; false
    /// when the zip does not have the company.
    fn from_bulk(&self, main: &str, dir: &Path) -> Result<bool, Box<dyn Error>> {
        let Some(bytes) = self.bulk_entry(main)? else { return Ok(false) };
        let subs: Submissions = serde_json::from_slice(&bytes)?;
        write_atomic(&dir.join("submissions.json"), &bytes)?;
        for file in &subs.filings.files {
            let page = self.bulk_entry(&file.name)?
                .ok_or_else(|| format!("{} not in submissions.zip", file.name))?;
            write_atomic(&dir.join(&file.name), &page)?;
        }
        Ok(true)
    }

    /// `name` from `submissions.zip`, downloading the zip first when there is
    /// no copy younger than `SUBMISSIONS_MAX_AGE`.
    fn bulk_entry(&self, name: &str) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
        let mut archive = self.archive.lock().unwrap();
        if archive.is_none() {
            let path = self.cache.dir().join("submissions.zip");
            let fresh = fs::metadata(&path).and_then(|m| m.modified()).ok()
                .is_some_and(|t| t.elapsed().unwrap_or_default() < SUBMISSIONS_MAX_AGE);
            if !fresh {
                eprintln!("downloading {}", BULK_URL);
                self.download(BULK_URL, &path)?;
            }
            *archive = Some(zip::ZipArchive::new(fs::File::open(&path)?)?);
        }
        let archive = archive.as_mut().unwrap();
        let mut entry = match archive.by_name(name) {
            Ok(entry) => entry,
            Err(zip::result::ZipError::FileNotFound) => return Ok(None),
            Err(e) => return Err(e.into()),
        };
        let mut bytes = Vec::new();
        std::io::copy(&mut entry, &mut bytes)?;
        Ok(Some(bytes))
    }

    /// Stream a large file to `path`, through a temporary file so an
    /// interrupted download is never taken for a complete one.
    fn download(&self, url: &str, path: &Path) -> Result<(), Box<dyn Error>> {
        self.pace();
        // The client's default 30-second timeout is far too short for 1.6 GB.
        let mut res = self.http.get(url).timeout(Duration::from_secs(3600)).send()?.error_for_status()?;
        let tmp = path.with_extension("partial");
        let mut file = fs::File::create(&tmp)?;
        std::io::copy(&mut res, &mut file)?;
        file.sync_all()?;
        fs::rename(&tmp, path)?;
        Ok(())
    }

    /// Download the filing's `FilingSummary.xml` and its cover and statement
    /// pages (`statement_pages`), skipping files already there, and return
    /// its directory.
    pub fn fetch_filing(&self, f: &Filing) -> Result<PathBuf, Box<dyn Error>> {
        let dir = self.cache.filing_path(f);
        fs::create_dir_all(&dir)?;
        if dir.join(NO_SUMMARY).exists() {
            return Err(EdgarError::NoFilingSummary.into());
        }

        let url = |file: &str| format!("https://www.sec.gov/Archives/edgar/data/{}/{}/{}", f.cik, f.accession_dir(), file);
        let summary_path = dir.join("FilingSummary.xml");
        if !summary_path.exists() {
            let res = self.get(&url("FilingSummary.xml"))?;
            if res.status() == reqwest::StatusCode::NOT_FOUND {
                fs::write(dir.join(NO_SUMMARY), "")?;
                return Err(EdgarError::NoFilingSummary.into());
            }
            write_atomic(&summary_path, &res.error_for_status()?.bytes()?)?;
        }

        let reports = crate::parse_filing_summary(&fs::read_to_string(&summary_path)?);
        if reports.is_empty() {
            fs::write(dir.join(NO_SUMMARY), "")?;
            return Err(EdgarError::NoFilingSummary.into());
        }
        for report in crate::statement_pages(&reports) {
            let path = dir.join(&report.file);
            if !path.exists() {
                let bytes = self.get(&url(&report.file))?.error_for_status()?.bytes()?;
                write_atomic(&path, &bytes)?;
            }
        }
        Ok(dir)
    }
}

/// Funds (see `crate::funds`).
impl Client {
    /// The fund ticker map, fetched again once a week.
    pub fn fetch_fund_tickers(&self) -> Result<(), Box<dyn Error>> {
        self.refresh(FUND_TICKERS_URL, &self.cache.fund_tickers_path(), Some(TICKERS_MAX_AGE))
    }

    /// The quarters the SEC lists for `set`, oldest first, each with the URL
    /// of its zip: usually "2026q3_nport.zip", but a republished quarter is
    /// "2024q1_ncen_0.zip" (the suffixed one wins where both are listed).
    pub fn data_set_quarters(&self, set: crate::DataSet) -> Result<Vec<(String, String)>, Box<dyn Error>> {
        let page = self.get(&set.page_url())?.error_for_status()?.text()?;
        let mut quarters: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
        for href in page.split('"').filter(|s| s.starts_with("/files/dera/") && s.ends_with(".zip")) {
            let file = href.rsplit('/').next().unwrap_or_default();
            let Some((quarter, rest)) = file.split_once('_') else { continue };
            if quarter.len() != 6 || quarter.as_bytes()[4] != b'q' || !rest.starts_with(set.name()) {
                continue;
            }
            let url = format!("https://www.sec.gov{}", href);
            if quarters.get(quarter).is_none_or(|u| u.len() < url.len()) {
                quarters.insert(quarter.to_string(), url);
            }
        }
        Ok(quarters.into_iter().collect())
    }

    /// Download `set`'s zip for `quarter` from `url` unless it is on disk (a quarter's
    /// set is final once published) or `force`. Returns whether it was
    /// downloaded.
    pub fn fetch_data_set(&self, set: crate::DataSet, quarter: &str, url: &str, force: bool) -> Result<bool, Box<dyn Error>> {
        let path = self.cache.data_set_path(set, quarter);
        if path.exists() && !force {
            return Ok(false);
        }
        fs::create_dir_all(self.cache.data_set_dir(set))?;
        self.download(url, &path)?;
        Ok(true)
    }
}

/// Write via a temporary file so an interrupted download never leaves a
/// partial file that later runs would treat as cached.
fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    if let Some(dir) = path.parent() { fs::create_dir_all(dir)?; }
    let tmp = path.with_extension("part");
    fs::write(&tmp, bytes)?;
    fs::rename(&tmp, path)?;
    Ok(())
}
