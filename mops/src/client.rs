//! Fetching reports from MOPS: one page per company and quarter, at most one
//! request every two seconds (MOPS publishes no rate limit; this is polite).

use std::cell::Cell;
use std::error::Error;
use std::fs;
use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::Cache;

const VIEWER: &str = "https://mopsov.twse.com.tw/server-java/t164sb01";
const MIN_INTERVAL: Duration = Duration::from_secs(2);
/// Identifies us without personal details.
const USER_AGENT: &str = "Mozilla/5.0 (Macintosh) lib-mops";

/// What fetching a quarter found.
#[derive(Debug, PartialEq, Eq)]
pub enum Fetched {
    /// Downloaded now.
    New,
    /// On disk already, or known missing.
    Cached,
    /// MOPS has no report (not yet filed, or before the company listed).
    Missing,
}

pub struct Client {
    http: reqwest::blocking::Client,
    cache: Cache,
    last: Cell<Option<Instant>>,
    requests: Cell<usize>,
}

impl Client {
    pub fn new(cache: Cache) -> Result<Client, Box<dyn Error>> {
        Ok(Client {
            http: reqwest::blocking::Client::builder().user_agent(USER_AGENT)
                .timeout(Duration::from_secs(120)).build()?,
            cache,
            last: Cell::new(None),
            requests: Cell::new(0),
        })
    }

    pub fn cache(&self) -> &Cache {
        &self.cache
    }

    pub fn requests(&self) -> usize {
        self.requests.get()
    }

    fn get(&self, url: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        if let Some(last) = self.last.get() {
            sleep(MIN_INTERVAL.saturating_sub(last.elapsed()));
        }
        self.last.set(Some(Instant::now()));
        self.requests.set(self.requests.get() + 1);
        let mut attempt = 0;
        loop {
            let res = self.http.get(url).send().and_then(|r| r.error_for_status()).and_then(|r| r.bytes());
            attempt += 1;
            match res {
                Ok(bytes) => return Ok(bytes.to_vec()),
                Err(e) if attempt < 3 && (e.is_timeout() || e.is_connect() || e.status().is_some_and(|s| s.is_server_error())) => {
                    sleep(Duration::from_secs(10 * attempt));
                },
                Err(e) => return Err(e.into()),
            }
        }
    }

    /// A company's report for a quarter, unless on disk: consolidated
    /// statements, or the company's own where it has none. A quarter MOPS
    /// lacks is remembered as missing once its report is long overdue (six
    /// months after the quarter), so it isn't asked for again.
    pub fn fetch(&self, code: &str, year: i32, quarter: u32) -> Result<Fetched, Box<dyn Error>> {
        let html = self.cache.path(code, year, quarter, "html");
        let none = self.cache.path(code, year, quarter, "none");
        if html.exists() || none.exists() {
            return Ok(Fetched::Cached);
        }
        for report in ["C", "A"] {
            let url = format!("{}?step=1&CO_ID={}&SYEAR={}&SSEASON={}&REPORT_ID={}", VIEWER, code, year, quarter, report);
            let bytes = self.get(&url)?;
            if has_statements(&bytes) {
                fs::create_dir_all(html.parent().unwrap())?;
                let tmp = html.with_extension("part");
                fs::write(&tmp, &bytes)?;
                fs::rename(&tmp, &html)?;
                return Ok(Fetched::New);
            }
        }
        let overdue = crate::quarter_end(year, quarter) + chrono::Duration::days(183);
        if chrono::Local::now().date_naive() > overdue {
            fs::create_dir_all(none.parent().unwrap())?;
            fs::write(&none, "")?;
        }
        Ok(Fetched::Missing)
    }
}

/// Whether a page is a report: MOPS answers a missing one with a short page
/// ("不存在!", bù cúnzài, "doesn't exist"; or 查無資料, cháwú zīliào, "no
/// data found"). A report runs to hundreds of kilobytes, and names its
/// statement of comprehensive income (綜合損益表, zònghé sǔnyì biǎo), in Big5.
fn has_statements(bytes: &[u8]) -> bool {
    const INCOME_STATEMENT_BIG5: &[u8] = &[0xba, 0xee, 0xa6, 0x58, 0xb7, 0x6c, 0xaf, 0x71, 0xaa, 0xed]; // 綜合損益表
    bytes.len() > 20_000 && bytes.windows(INCOME_STATEMENT_BIG5.len()).any(|w| w == INCOME_STATEMENT_BIG5)
}
