//! Funds: mutual funds, ETFs and the rest of the registered investment
//! companies, from two of EDGAR's sources, cached beside the filings:
//!
//! ```text
//! cache/company_tickers_mf.json           ticker -> CIK, series, class
//! cache/dera/ncen/2026q3_ncen.zip         N-CEN data set: the annual census
//! cache/dera/nport/2026q3_nport.zip       N-PORT data set: monthly portfolios
//! ```
//!
//! A registrant (a CIK, such as PIMCO ETF Trust or iShares, Inc.) offers one
//! or more *series* (the funds: MINT, EWT), each with one or more share
//! *classes*; a ticker names a class.
//!
//! The SEC's Division of Economic and Risk Analysis publishes the structured
//! parts of every public N-CEN and N-PORT filing as tab-separated tables, one
//! zip per quarter of filing dates
//! (<https://www.sec.gov/data-research/sec-markets-data/form-n-port-data-sets>):
//!
//! - **N-CEN** (from 2018), filed once a year per registrant: for each series,
//!   whether it is an ETF, an index fund, a money market fund; for index funds,
//!   the return against the index. `census` reads it.
//! - **N-PORT** (from late 2019), filed monthly and public for each fiscal
//!   quarter's last month: net assets, flows, every holding, and each class's
//!   total return for each of the three months to the report date.
//!   `monthly_returns` reads those.
//!
//! One zip per quarter instead of one request per filing: about 280,000
//! NPORT-P filings come down as 28 files.

use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;

use chrono::{Datelike, NaiveDate};
use serde::Deserialize;

use crate::{Cache, EdgarError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataSet {
    Ncen,
    Nport,
}

impl DataSet {
    pub const ALL: [DataSet; 2] = [DataSet::Ncen, DataSet::Nport];

    pub fn name(self) -> &'static str {
        match self {
            DataSet::Ncen => "ncen",
            DataSet::Nport => "nport",
        }
    }

    /// The page listing the quarters' zips.
    pub fn page_url(self) -> String {
        format!("https://www.sec.gov/data-research/sec-markets-data/form-n-{}-data-sets",
            match self { DataSet::Ncen => "cen", DataSet::Nport => "port" })
    }
}

/// One share class, from `company_tickers_mf.json`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FundClass {
    pub cik: u64,
    pub series_id: String,
    pub class_id: String,
    pub ticker: String,
}

#[derive(Deserialize)]
struct MfTickers {
    data: Vec<(u64, String, String, String)>,
}

/// One series in one N-CEN filing.
#[derive(Debug, Clone)]
pub struct Census {
    pub cik: u64,
    pub accession: String,
    pub series_id: String,
    pub name: String,
    /// The end of the fiscal year the filing covers.
    pub period: NaiveDate,
    pub is_etf: bool,
    pub is_index: bool,
    pub is_money_market: bool,
    pub is_fund_of_funds: bool,
    /// Index funds: the difference between the fund's return and its index's,
    /// in percent, before and after fees and expenses.
    pub tracking_before_fees: Option<f64>,
    pub tracking_after_fees: Option<f64>,
    pub monthly_avg_net_assets: Option<f64>,
}

/// One class's total return for one month, in percent, as the fund reported
/// it on N-PORT.
#[derive(Debug, Clone)]
pub struct MonthlyReturn {
    pub class_id: String,
    /// The month's last day.
    pub month: NaiveDate,
    pub percent: f64,
    pub accession: String,
}

/// A fund's portfolio on one N-PORT report.
#[derive(Debug, Clone)]
pub struct Portfolio {
    pub series_id: String,
    pub accession: String,
    pub report_date: NaiveDate,
    pub net_assets: f64,
    pub holdings: Vec<Holding>,
}

/// One position. `percent` is of net assets (negative for shorts and
/// liabilities); `value` is in `currency`, and `value_usd` converts it at
/// the reported exchange rate.
#[derive(Debug, Clone)]
pub struct Holding {
    pub issuer: String,
    pub title: String,
    pub cusip: String,
    pub value: f64,
    pub currency: String,
    pub value_usd: f64,
    pub percent: f64,
    /// N-PORT's asset category: "DBT" (debt), "EC" (common equity),
    /// "ABS-MBS", "ABS-CBDO", "STIV" (short-term investment vehicle), ...
    pub asset_category: String,
    /// "CORP", "UST" (Treasury), "USGA" (agency), "USGSE" (sponsored
    /// enterprise), "NUSS" (foreign sovereign), "RF" (registered fund), ...
    pub issuer_type: String,
    pub country: String,
    pub fair_value_level: String,
    /// Debt only.
    pub maturity: Option<NaiveDate>,
    pub coupon_type: Option<String>,
    pub coupon: Option<f64>,
}

impl Cache {
    /// The latest portfolio N-PORT reports for `series_id`, from the newest
    /// data set on disk that has one. Reads the quarter's holdings table
    /// (about 1 GB) as a stream.
    pub fn portfolio(&self, series_id: &str) -> Result<Option<Portfolio>, Box<dyn Error>> {
        for (_, path) in self.data_sets(DataSet::Nport).into_iter().rev() {
            let mut zip = zip::ZipArchive::new(fs::File::open(&path)?)?;
            let mut found: Option<(String, f64)> = None;
            for row in Table::open(&mut zip, "FUND_REPORTED_INFO.tsv")? {
                let row = row?;
                if row.get("SERIES_ID") == Some(series_id) {
                    let accession = row.get("ACCESSION_NUMBER").unwrap_or_default().to_string();
                    let net = row.get("NET_ASSETS").and_then(|v| v.parse().ok()).unwrap_or(0.0);
                    // Several reports a quarter: keep the latest (accessions sort by filing).
                    if found.as_ref().is_none_or(|f| accession > f.0) {
                        found = Some((accession, net));
                    }
                }
            }
            let Some((accession, net_assets)) = found else { continue };
            let mut report_date = None;
            for row in Table::open(&mut zip, "SUBMISSION.tsv")? {
                let row = row?;
                if row.get("ACCESSION_NUMBER") == Some(accession.as_str()) {
                    report_date = row.get("REPORT_DATE").and_then(date);
                }
            }
            let mut holdings = Vec::new();
            let mut ids: HashMap<String, usize> = HashMap::new();
            for row in Table::open(&mut zip, "FUND_REPORTED_HOLDING.tsv")? {
                let row = row?;
                if row.get("ACCESSION_NUMBER") != Some(accession.as_str()) {
                    continue;
                }
                let s = |c: &str| row.get(c).unwrap_or_default().to_string();
                let num = |c: &str| row.get(c).and_then(|v| v.parse::<f64>().ok());
                let value = num("CURRENCY_VALUE").unwrap_or(0.0);
                let rate = num("EXCHANGE_RATE").filter(|r| *r > 0.0).unwrap_or(1.0);
                ids.insert(s("HOLDING_ID"), holdings.len());
                holdings.push(Holding {
                    issuer: s("ISSUER_NAME"), title: s("ISSUER_TITLE"), cusip: s("ISSUER_CUSIP"),
                    value, currency: s("CURRENCY_CODE"), value_usd: value / rate,
                    percent: num("PERCENTAGE").unwrap_or(0.0),
                    asset_category: s("ASSET_CAT"), issuer_type: s("ISSUER_TYPE"),
                    country: s("INVESTMENT_COUNTRY"), fair_value_level: s("FAIR_VALUE_LEVEL"),
                    maturity: None, coupon_type: None, coupon: None,
                });
            }
            for row in Table::open(&mut zip, "DEBT_SECURITY.tsv")? {
                let row = row?;
                let Some(&i) = row.get("HOLDING_ID").and_then(|id| ids.get(id)) else { continue };
                let h = &mut holdings[i];
                h.maturity = row.get("MATURITY_DATE").and_then(date);
                h.coupon_type = row.get("COUPON_TYPE").filter(|c| !c.is_empty()).map(str::to_string);
                h.coupon = row.get("ANNUALIZED_RATE").and_then(|v| v.parse().ok());
            }
            return Ok(Some(Portfolio {
                series_id: series_id.to_string(), accession,
                report_date: report_date.ok_or("report without a date")?, net_assets, holdings,
            }));
        }
        Ok(None)
    }

    pub fn fund_tickers_path(&self) -> PathBuf {
        self.dir().join("company_tickers_mf.json")
    }

    pub fn data_set_dir(&self, set: DataSet) -> PathBuf {
        self.dir().join("dera").join(set.name())
    }

    pub fn data_set_path(&self, set: DataSet, quarter: &str) -> PathBuf {
        self.data_set_dir(set).join(format!("{}_{}.zip", quarter, set.name()))
    }

    /// Every fund share class with a ticker.
    pub fn fund_classes(&self) -> Result<Vec<FundClass>, Box<dyn Error>> {
        let path = self.fund_tickers_path();
        let bytes = fs::read(&path).map_err(|_| EdgarError::NotFetched(path))?;
        let mf: MfTickers = serde_json::from_slice(&bytes)?;
        Ok(mf.data.into_iter()
            .map(|(cik, series_id, class_id, ticker)| FundClass { cik, series_id, class_id, ticker })
            .collect())
    }

    pub fn fund_class(&self, ticker: &str) -> Result<FundClass, Box<dyn Error>> {
        self.fund_classes()?.into_iter()
            .find(|c| c.ticker.eq_ignore_ascii_case(ticker))
            .ok_or_else(|| EdgarError::TickerNotFound(ticker.to_string()).into())
    }

    /// The quarters of `set` on disk, oldest first.
    pub fn data_sets(&self, set: DataSet) -> Vec<(String, PathBuf)> {
        let suffix = format!("_{}.zip", set.name());
        let mut v: Vec<(String, PathBuf)> = fs::read_dir(self.data_set_dir(set)).into_iter().flatten()
            .filter_map(|e| e.ok())
            .filter_map(|e| {
                let name = e.file_name().into_string().ok()?;
                Some((name.strip_suffix(&suffix)?.to_string(), e.path()))
            })
            .collect();
        v.sort();
        v
    }

    /// Every series in every N-CEN filing on disk, oldest filing first.
    pub fn census(&self) -> Result<Vec<Census>, Box<dyn Error>> {
        let mut all = Vec::new();
        for (_, path) in self.data_sets(DataSet::Ncen) {
            let mut zip = zip::ZipArchive::new(fs::File::open(&path)?)?;
            let mut subs: HashMap<String, (u64, NaiveDate)> = HashMap::new();
            for row in Table::open(&mut zip, "SUBMISSION.tsv")? {
                let row = row?;
                if let (Some(cik), Some(period)) = (row.get("CIK").and_then(|c| c.parse().ok()), row.get("REPORT_ENDING_PERIOD").and_then(date)) {
                    subs.insert(row.get("ACCESSION_NUMBER").unwrap_or_default().to_string(), (cik, period));
                }
            }
            for row in Table::open(&mut zip, "FUND_REPORTED_INFO.tsv")? {
                let row = row?;
                let accession = row.get("ACCESSION_NUMBER").unwrap_or_default().to_string();
                let Some(&(cik, period)) = subs.get(&accession) else { continue };
                let Some(series_id) = row.get("SERIES_ID").filter(|s| !s.is_empty()) else { continue };
                let yes = |c: &str| row.get(c) == Some("Y");
                let num = |c: &str| row.get(c).and_then(|v| v.parse().ok());
                all.push(Census {
                    cik, accession: accession.clone(), series_id: series_id.to_string(),
                    name: row.get("FUND_NAME").unwrap_or_default().to_string(), period,
                    is_etf: yes("IS_ETF"), is_index: yes("IS_INDEX"),
                    is_money_market: yes("IS_MONEY_MARKET"), is_fund_of_funds: yes("IS_FUND_OF_FUND"),
                    tracking_before_fees: num("RETURN_B4_FEES_AND_EXPENSES"),
                    tracking_after_fees: num("RETURN_AFTR_FEES_AND_EXPENSES"),
                    monthly_avg_net_assets: num("MONTHLY_AVG_NET_ASSETS"),
                });
            }
        }
        Ok(all)
    }

    /// Each series' latest census.
    pub fn latest_census(&self) -> Result<HashMap<String, Census>, Box<dyn Error>> {
        let mut latest: HashMap<String, Census> = HashMap::new();
        for c in self.census()? {
            if latest.get(&c.series_id).is_none_or(|l| c.period >= l.period) {
                latest.insert(c.series_id.clone(), c);
            }
        }
        Ok(latest)
    }

    /// The monthly total returns of the classes `wanted` accepts, from every
    /// N-PORT data set on disk: one per class and month, the latest filing's
    /// where amendments or overlapping reports repeat a month.
    pub fn monthly_returns(&self, wanted: impl Fn(&str) -> bool) -> Result<Vec<MonthlyReturn>, Box<dyn Error>> {
        let mut by_month: HashMap<(String, NaiveDate), (NaiveDate, MonthlyReturn)> = HashMap::new();
        for (_, path) in self.data_sets(DataSet::Nport) {
            let mut zip = zip::ZipArchive::new(fs::File::open(&path)?)?;
            let mut rows = Vec::new();
            for row in Table::open(&mut zip, "MONTHLY_TOTAL_RETURN.tsv")? {
                let row = row?;
                let class = row.get("CLASS_ID").unwrap_or_default();
                if !wanted(class) {
                    continue;
                }
                let r: Vec<Option<f64>> = ["MONTHLY_TOTAL_RETURN1", "MONTHLY_TOTAL_RETURN2", "MONTHLY_TOTAL_RETURN3"]
                    .iter().map(|c| row.get(c).and_then(|v| v.parse().ok())).collect();
                rows.push((row.get("ACCESSION_NUMBER").unwrap_or_default().to_string(), class.to_string(), r));
            }
            if rows.is_empty() {
                continue;
            }
            let mut subs: HashMap<String, (NaiveDate, NaiveDate)> = HashMap::new();
            for row in Table::open(&mut zip, "SUBMISSION.tsv")? {
                let row = row?;
                if let (Some(report), Some(filed)) = (row.get("REPORT_DATE").and_then(date), row.get("FILING_DATE").and_then(date)) {
                    subs.insert(row.get("ACCESSION_NUMBER").unwrap_or_default().to_string(), (report, filed));
                }
            }
            for (accession, class, r) in rows {
                let Some(&(report, filed)) = subs.get(&accession) else { continue };
                // Return 3 is the report date's month, 1 two months before.
                for (i, v) in r.into_iter().enumerate() {
                    let Some(percent) = v else { continue };
                    let month = month_end(report, i as i32 - 2);
                    let key = (class.clone(), month);
                    if by_month.get(&key).is_none_or(|(f, _)| filed >= *f) {
                        by_month.insert(key, (filed, MonthlyReturn { class_id: class.clone(), month, percent, accession: accession.clone() }));
                    }
                }
            }
        }
        let mut v: Vec<MonthlyReturn> = by_month.into_values().map(|(_, r)| r).collect();
        v.sort_by(|a, b| (&a.class_id, a.month).cmp(&(&b.class_id, b.month)));
        Ok(v)
    }
}

/// DERA's dates: "31-AUG-2026".
fn date(s: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(s, "%d-%b-%Y").ok()
}

/// The last day of the month `offset` months from `d`'s.
fn month_end(d: NaiveDate, offset: i32) -> NaiveDate {
    let m = d.year() * 12 + d.month0() as i32 + offset + 1;
    NaiveDate::from_ymd_opt(m.div_euclid(12), m.rem_euclid(12) as u32 + 1, 1).unwrap().pred_opt().unwrap()
}

/// A tab-separated table inside a data set's zip, read row by row.
pub struct Table<R> {
    lines: std::io::Lines<BufReader<R>>,
    header: std::rc::Rc<HashMap<String, usize>>,
}

pub struct Row {
    fields: Vec<String>,
    header: std::rc::Rc<HashMap<String, usize>>,
}

impl Row {
    pub fn get(&self, column: &str) -> Option<&str> {
        self.header.get(column).and_then(|&i| self.fields.get(i)).map(|s| s.as_str())
    }
}

impl<'a> Table<zip::read::ZipFile<'a, fs::File>> {
    pub fn open(zip: &'a mut zip::ZipArchive<fs::File>, name: &str) -> Result<Self, Box<dyn Error>> {
        Table::new(zip.by_name(name)?)
    }
}

impl<R: Read> Table<R> {
    pub fn new(reader: R) -> Result<Self, Box<dyn Error>> {
        let mut lines = BufReader::with_capacity(1 << 20, reader).lines();
        let first = lines.next().ok_or("empty table")??;
        let header = first.trim_start_matches('\u{feff}').split('\t').enumerate()
            .map(|(i, c)| (c.to_string(), i)).collect();
        Ok(Table { lines, header: std::rc::Rc::new(header) })
    }
}

impl<R: Read> Iterator for Table<R> {
    type Item = std::io::Result<Row>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = match self.lines.next()? {
            Ok(l) => l,
            Err(e) => return Some(Err(e)),
        };
        Some(Ok(Row { fields: line.split('\t').map(str::to_string).collect(), header: self.header.clone() }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn month_ends() {
        let june = NaiveDate::from_ymd_opt(2026, 6, 30).unwrap();
        assert_eq!(month_end(june, -2), NaiveDate::from_ymd_opt(2026, 4, 30).unwrap());
        assert_eq!(month_end(june, 0), june);
        let jan = NaiveDate::from_ymd_opt(2026, 1, 31).unwrap();
        assert_eq!(month_end(jan, -2), NaiveDate::from_ymd_opt(2025, 11, 30).unwrap());
        assert_eq!(date("31-AUG-2026"), NaiveDate::from_ymd_opt(2026, 8, 31));
    }

    #[test]
    fn reads_a_table() {
        let t = Table::new("A\tB\n1\t\n2\tx\n".as_bytes()).unwrap();
        let rows: Vec<Row> = t.map(Result::unwrap).collect();
        assert_eq!(rows[0].get("A"), Some("1"));
        assert_eq!(rows[0].get("B"), Some(""));
        assert_eq!(rows[1].get("B"), Some("x"));
        assert_eq!(rows[1].get("C"), None);
    }
}
