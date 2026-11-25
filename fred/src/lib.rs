//! FRED (Federal Reserve Economic Data, from the Federal Reserve Bank of
//! St. Louis) as a source, the way `edgar` is one for filings: each series
//! kept on disk under `cache/` as FRED serves it,
//!
//! ```text
//! cache/GS10.csv      observation_date,GS10
//!                     1953-04-01,2.83
//!                     ...
//! ```
//!
//! one row per observation, with an empty value (or ".") where FRED has none.
//! `Cache` reads this and never touches the network; it is all a reader
//! (`equities`) needs. With the `fetch` feature, `Client` downloads into it,
//! and `fred-fetch` runs the client over a list of series.
//!
//! Series used so far:
//!
//! - `GS10`: market yield on 10-year Treasuries, constant maturity, monthly
//!   averages, percent (the Federal Reserve's H.15).

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use chrono::{Datelike, NaiveDate};

#[cfg(feature = "fetch")]
mod client;
#[cfg(feature = "fetch")]
pub use client::{Client, USER_AGENT};

#[derive(Debug)]
pub enum FredError {
    /// Not in the cache: run `fred-fetch` first.
    NotFetched(PathBuf),
    /// A line that isn't `date,value`.
    Malformed { series: String, line: usize },
}

impl std::fmt::Display for FredError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FredError::NotFetched(p) => write!(f, "not fetched yet: {} (run fred-fetch)", p.display()),
            FredError::Malformed { series, line } => write!(f, "{}: line {} is not date,value", series, line),
        }
    }
}

impl Error for FredError {}

/// One observation: the start of its period (a month's first day for a
/// monthly series) and its value, if FRED has one.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Observation {
    pub date: NaiveDate,
    pub value: Option<f64>,
}

/// A series' observations, oldest first.
#[derive(Debug, Clone)]
pub struct Series {
    pub id: String,
    pub observations: Vec<Observation>,
}

impl Series {
    /// Parse FRED's CSV (`fredgraph.csv`): a header, then `date,value` rows.
    pub fn parse(id: &str, csv: &str) -> Result<Series, FredError> {
        let mut observations = Vec::new();
        for (i, line) in csv.lines().enumerate().skip(1) {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let malformed = || FredError::Malformed { series: id.to_string(), line: i + 1 };
            let (d, v) = line.split_once(',').ok_or_else(malformed)?;
            let date = NaiveDate::parse_from_str(d, "%Y-%m-%d").map_err(|_| malformed())?;
            let value = match v.trim() {
                "" | "." => None,
                v => Some(v.parse().map_err(|_| malformed())?),
            };
            observations.push(Observation { date, value });
        }
        Ok(Series { id: id.to_string(), observations })
    }

    /// The value for `year`'s `month` (1–12), for a monthly series.
    pub fn month(&self, year: i32, month: u32) -> Option<f64> {
        self.observations.iter()
            .find(|o| o.date.year() == year && o.date.month() == month)
            .and_then(|o| o.value)
    }

    /// The mean of `year`'s observations, if it has values in at least
    /// `min` of them (12 for a full monthly year; fewer for a series that
    /// starts or ends mid-year).
    pub fn year_mean(&self, year: i32, min: usize) -> Option<f64> {
        let v: Vec<f64> = self.observations.iter()
            .filter(|o| o.date.year() == year).filter_map(|o| o.value).collect();
        (v.len() >= min.max(1)).then(|| v.iter().sum::<f64>() / v.len() as f64)
    }

    /// The latest observation with a value.
    pub fn latest(&self) -> Option<Observation> {
        self.observations.iter().rev().find(|o| o.value.is_some()).copied()
    }
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
    /// directory a program runs in; `FRED_CACHE` names another.
    pub fn default_dir() -> PathBuf {
        std::env::var_os("FRED_CACHE").map(PathBuf::from)
            .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("cache"))
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    pub fn path(&self, id: &str) -> PathBuf {
        self.dir.join(format!("{}.csv", id))
    }

    pub fn series(&self, id: &str) -> Result<Series, Box<dyn Error>> {
        let path = self.path(id);
        let csv = fs::read_to_string(&path).map_err(|_| FredError::NotFetched(path))?;
        Ok(Series::parse(id, &csv)?)
    }

    /// Every series in the cache, by ID, sorted.
    pub fn ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = fs::read_dir(&self.dir).into_iter().flatten().filter_map(|e| e.ok())
            .filter_map(|e| e.file_name().into_string().ok()?.strip_suffix(".csv").map(str::to_string))
            .collect();
        ids.sort();
        ids
    }
}

/// A list of series (`rates.txt`): IDs separated by spaces, commas or
/// newlines, `#` starting a comment. Each ID comes with the comment on its
/// line when it is the line's only ID (its description, for a reader).
pub fn read_list(path: impl AsRef<Path>) -> std::io::Result<Vec<(String, Option<String>)>> {
    let mut out = Vec::new();
    for line in fs::read_to_string(path)?.lines() {
        let (ids, note) = match line.split_once('#') {
            Some((ids, note)) => (ids, Some(note.trim().to_string()).filter(|n| !n.is_empty())),
            None => (line, None),
        };
        let ids: Vec<String> = ids.split(|c: char| c == ',' || c.is_whitespace())
            .filter(|s| !s.is_empty()).map(str::to_uppercase).collect();
        let one = ids.len() == 1;
        out.extend(ids.into_iter().map(|id| (id, if one { note.clone() } else { None })));
    }
    Ok(out)
}

/// The series list kept beside the crate, `rates.txt`.
pub fn default_list() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("rates.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_fredgraph_csv() {
        let s = Series::parse("GS10", "observation_date,GS10\n1981-12-01,13.72\n1982-01-01,.\n1982-02-01,\n").unwrap();
        assert_eq!(s.month(1981, 12), Some(13.72));
        assert_eq!(s.month(1982, 1), None);
        assert_eq!(s.latest().unwrap().date, NaiveDate::from_ymd_opt(1981, 12, 1).unwrap());
        assert!(Series::parse("X", "h\nnot a row\n").is_err());
    }

    #[test]
    fn reads_a_list() {
        let dir = std::env::temp_dir().join(format!("fred-list-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let p = dir.join("list.txt");
        fs::write(&p, "# heading\nGS10   # 10-year\ngs1 GS2\nAAA, BAA # Moody's\n").unwrap();
        let l = read_list(&p).unwrap();
        assert_eq!(l, vec![
            ("GS10".to_string(), Some("10-year".to_string())),
            ("GS1".to_string(), None), ("GS2".to_string(), None),
            ("AAA".to_string(), None), ("BAA".to_string(), None),
        ]);
        fs::remove_dir_all(dir).unwrap();
    }
}
