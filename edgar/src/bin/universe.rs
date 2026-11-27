//! Every company that filed annual or quarterly reports (10-K, 10-Q, 20-F)
//! since a date, by CIK, from EDGAR's bulk submissions.zip: the companies
//! with a ticker today and the ones without (acquired, bankrupt, delisted,
//! or never listed), which a list of today's tickers leaves out. For
//! backtests that would otherwise only see the survivors.
//!
//!     universe [--from YYYY-MM-DD] [--without-tickers] [--operating] [--cik-only]
//!
//! `--from` defaults to 2009-01-01, when filings began to carry the XBRL
//! statements the readers use. `--operating` keeps operating companies:
//! EDGAR's entityType "operating", less the industries (SIC) of vehicles
//! that file 10-Ks without being businesses a portfolio would hold
//! (`NOT_BUSINESSES`: asset-backed trusts, blank checks, funds, commodity
//! pools). Prints one company a line, most recently
//! active first:
//!
//!     CIK1018724<TAB>AMAZON COM INC<TAB>AMZN<TAB>5961<TAB>2009-01-30<TAB>2026-10-31
//!
//! (CIK, name, tickers today or empty, SIC industry code, first and last
//! such filing; the first is within the latest 1,000 filings, which for a
//! busy filer starts later than its first report), or just
//! the CIK with `--cik-only`, as `fetch --list` reads it. Needs the zip in
//! the cache: `fetch --bulk` downloads it (refreshed nightly by
//! edgar/nightly/all.sh).

use std::error::Error;
use std::io::Read;

use edgar::{Cache, FORMS};

/// SIC codes of filers that aren't operating businesses: asset-backed
/// securities (6189), commodity pools (6221), investment funds (6722, 6726),
/// blank checks, SPACs among them (6770).
const NOT_BUSINESSES: &[&str] = &["6189", "6221", "6722", "6726", "6770"];

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    let mut from = "2009-01-01".to_string();
    let (mut without_tickers, mut cik_only, mut operating) = (false, false, false);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--from" => from = args.next().ok_or("--from needs a date")?,
            "--without-tickers" => without_tickers = true,
            "--cik-only" => cik_only = true,
            "--operating" => operating = true,
            a => return Err(format!("unknown argument {}", a).into()),
        }
    }

    let cache = Cache::new(Cache::default_dir());
    let path = cache.dir().join("submissions.zip");
    let mut zip = zip::ZipArchive::new(std::fs::File::open(&path)
        .map_err(|e| format!("{}: {} (run fetch --bulk first)", path.display(), e))?)?;

    let mut companies = Vec::new();
    let mut bytes = Vec::new();
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i)?;
        // The main file per company; its older history is paged out to
        // CIK…-submissions-NNN.json, which a company filing since 2009 has
        // reports in the main file anyway.
        let name = entry.name().to_string();
        if !name.starts_with("CIK") || name.contains("-submissions-") {
            continue;
        }
        bytes.clear();
        entry.read_to_end(&mut bytes)?;
        let v: serde_json::Value = serde_json::from_slice(&bytes)?;
        let recent = &v["filings"]["recent"];
        let (Some(forms), Some(dates)) = (recent["form"].as_array(), recent["filingDate"].as_array()) else { continue };
        let mut filed: Vec<&str> = forms.iter().zip(dates)
            .filter(|(f, _)| f.as_str().is_some_and(|f| FORMS.contains(&f)))
            .filter_map(|(_, d)| d.as_str())
            .filter(|d| *d >= from.as_str())
            .collect();
        if filed.is_empty() {
            continue;
        }
        filed.sort();
        let tickers: Vec<&str> = v["tickers"].as_array().map(|t| t.iter().filter_map(|t| t.as_str()).collect()).unwrap_or_default();
        if without_tickers && !tickers.is_empty() {
            continue;
        }
        if operating && v["entityType"].as_str() != Some("operating") {
            continue;
        }
        let sic = v["sic"].as_str().unwrap_or("").to_string();
        if operating && NOT_BUSINESSES.contains(&sic.as_str()) {
            continue;
        }
        let cik = v["cik"].as_str().and_then(|c| c.parse::<u64>().ok())
            .or_else(|| v["cik"].as_u64())
            .or_else(|| name[3..13].parse().ok())
            .ok_or_else(|| format!("{}: no CIK", name))?;
        companies.push((cik, v["name"].as_str().unwrap_or("").to_string(), tickers.join(","), sic,
            filed[0].to_string(), filed[filed.len() - 1].to_string()));
    }

    companies.sort_by(|a, b| b.5.cmp(&a.5).then(a.0.cmp(&b.0)));
    let with = companies.iter().filter(|c| !c.2.is_empty()).count();
    eprintln!("{} companies filed {} since {}: {} with a ticker today, {} without",
        companies.len(), FORMS.join("/"), from, with, companies.len() - with);
    for (cik, name, tickers, sic, first, last) in companies {
        if cik_only {
            println!("CIK{}", cik);
        } else {
            println!("CIK{}\t{}\t{}\t{}\t{}\t{}", cik, name, tickers, sic, first, last);
        }
    }
    Ok(())
}
