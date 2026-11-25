//! Reads a report from the EDINET cache (see the `edinet` crate) into the
//! same reported items as EDGAR filings.
//!
//! EDINET publishes each report's XBRL facts as tab-separated CSV in UTF-16,
//! one fact per line:
//!
//! | column                  | reading            | meaning                    |
//! |-------------------------|--------------------|----------------------------|
//! | 要素ID                  | yōso ID            | element ID                 |
//! | 項目名                  | kōmoku-mei         | item name (the label)      |
//! | コンテキストID          | kontekisuto ID     | context                    |
//! | 相対年度                | sōtai nendo        | relative year              |
//! | 連結・個別              | renketsu / kobetsu | consolidated / parent only |
//! | 期間・時点              | kikan / jiten      | duration / instant         |
//! | ユニットID              | yunitto ID         | unit                       |
//! | 単位                    | tan'i              | unit, in Japanese          |
//! | 値                      | atai               | value                      |
//!
//! A context names a period relative to the report's, e.g. "CurrentYTDDuration"
//! (当四半期累計期間, tō-shihanki ruikei kikan, this quarter's year to date) or
//! "Prior1YearInstant" (前期末, zenki-matsu, the end of last fiscal year).
//! Facts under a member ("…_NonConsolidatedMember", the parent company alone;
//! equity components; segments) are left out, as on EDGAR.

use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

use chrono::{Months, NaiveDate};

use crate::item::{Item, Reported, Unit};
use crate::{Currency, Period, Ticker};

/// One fact: element, label, context, unit and value.
struct Fact {
    element: String,
    label: String,
    context: String,
    unit: String,
    value: String,
}

/// The facts in an EDINET CSV file (UTF-16 little-endian, tab-separated,
/// every field quoted).
fn read_facts(path: &Path) -> Result<Vec<Fact>, Box<dyn Error>> {
    let bytes = std::fs::read(path)?;
    let units: Vec<u16> = bytes.chunks_exact(2).map(|b| u16::from_le_bytes([b[0], b[1]])).collect();
    let text = String::from_utf16_lossy(&units);
    let mut facts = Vec::new();
    for line in text.trim_start_matches('\u{feff}').lines().skip(1) {
        let f: Vec<&str> = line.split('\t').map(|c| c.trim().trim_matches('"')).collect();
        if f.len() < 9 { continue; }
        facts.push(Fact {
            element: f[0].to_string(),
            label: f[1].to_string(),
            context: f[2].to_string(),
            unit: f[6].to_string(),
            value: f[8].to_string(),
        });
    }
    Ok(facts)
}

/// The kind of report, from its EDINET document type.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Kind {
    /// 有価証券報告書 (yūka shōken hōkokusho)
    Annual,
    /// 半期報告書 (hanki hōkokusho)
    Semiannual,
    /// 四半期報告書 (shihanki hōkokusho), the nth quarter of the fiscal year.
    Quarterly(u32),
}

fn kind(d: &edinet::Document) -> Option<Kind> {
    match d.doc_type.as_deref()? {
        "120" | "130" => Some(Kind::Annual),
        "160" | "170" => Some(Kind::Semiannual),
        "140" | "150" => {
            // "四半期報告書－第120期第2四半期(…)": the 120th term, 2nd quarter
            // (dai ni shihanki). Digits may be full width.
            let desc = d.description.as_deref()?;
            let q = desc.split("期第").nth(1)?.chars().next()?;
            let q = q.to_digit(10).or_else(|| ('１'..='４').position(|c| c == q).map(|i| i as u32 + 1))?;
            Some(Kind::Quarterly(q))
        },
        _ => None,
    }
}

/// The last day `months` months before the day after `end`: a month-end
/// date moved back by whole months.
fn months_before(end: NaiveDate, months: u32) -> NaiveDate {
    (end.succ_opt().unwrap() - Months::new(months)).pred_opt().unwrap()
}

/// The date and period a context reports, for a report of `kind` ending `end`.
fn context_period(context: &str, kind: Kind, end: NaiveDate) -> Option<(NaiveDate, Period)> {
    // "Prior1YearInstant" -> (1, "Year", "Instant"); "CurrentQuarterDuration" -> (0, "Quarter", "Duration").
    let (back, rest) = if let Some(r) = context.strip_prefix("Current") {
        (0, r)
    } else {
        let r = context.strip_prefix("Prior")?;
        let n: String = r.chars().take_while(|c| c.is_ascii_digit()).collect();
        (n.parse::<u32>().ok()?, &r[n.len()..])
    };
    let (span, instant) = if let Some(s) = rest.strip_suffix("Duration") {
        (s, false)
    } else {
        (rest.strip_suffix("Instant")?, true)
    };
    // Months from the start of the fiscal year to the report's end.
    let into_year = match kind {
        Kind::Annual => 12,
        Kind::Semiannual => 6,
        Kind::Quarterly(q) => 3 * q,
    };
    let ytd = match into_year {
        3 => Period::ThreeMonths,
        6 => Period::SixMonths,
        9 => Period::NineMonths,
        _ => Period::TwelveMonths,
    };
    let year_end = months_before(end, into_year);
    match (span, instant) {
        // This report's period, or the same one n years back.
        ("Year", true) if kind == Kind::Annual => Some((months_before(end, 12 * back), Period::PointInTime)),
        ("Year", false) if kind == Kind::Annual => Some((months_before(end, 12 * back), Period::TwelveMonths)),
        ("Quarter" | "Interim", true) => Some((months_before(end, 12 * back), Period::PointInTime)),
        ("Quarter", false) => Some((months_before(end, 12 * back), Period::ThreeMonths)),
        ("YTD" | "Interim", false) => Some((months_before(end, 12 * back), ytd)),
        // In an interim report, "Year" is a whole fiscal year: the last one
        // ended before this report's year began.
        ("Year", true) if back >= 1 => Some((months_before(year_end, 12 * (back - 1)), Period::PointInTime)),
        ("Year", false) if back >= 1 => Some((months_before(year_end, 12 * (back - 1)), Period::TwelveMonths)),
        _ => None,
    }
}

/// An element ID as `item` lists it. A company's own elements carry the
/// report type in their prefix ("jpcrp030000-asr_E02144-000:" in an annual
/// report, "jpcrp040300-q2r_E02144-000:" in a Q2 report); they are listed by
/// the company's EDINET code alone: "E02144:TotalNetRevenuesIFRS".
fn element_tag(element: &str) -> std::borrow::Cow<'_, str> {
    match element.split_once(':') {
        Some((prefix, name)) if prefix.starts_with("jpcrp") && prefix.contains('_') => {
            let code = prefix.split('_').nth(1).unwrap_or("").split('-').next().unwrap_or("");
            format!("{}:{}", code, name).into()
        },
        _ => element.into(),
    }
}

/// A fact's value: a number, or nothing ("－" for none).
fn number(s: &str) -> Option<f64> {
    s.replace(',', "").parse::<f64>().ok()
}

/// The items one EDINET report states, for `ticker`.
pub fn read(cache: &edinet::Cache, d: &edinet::Document, ticker: Ticker) -> Result<Vec<Reported>, Box<dyn Error>> {
    let kind = kind(d).ok_or("not an annual, semiannual or quarterly report")?;
    let end = d.period_end.ok_or("report without a period")?;

    // Several elements can report one item (an IFRS and a summary element):
    // the lowest ranked wins, as on EDGAR.
    let mut best: HashMap<(Item, NaiveDate, Period, Currency), (usize, f64)> = HashMap::new();
    for file in cache.csv_files(d)? {
        // The report proper ("jpcrp…-asr-…", "-q2r-"…); auditors' reports
        // ("jpaud-…") state nothing we read.
        let name = file.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if !name.starts_with("jpcrp") { continue; }
        for fact in read_facts(&file)? {
            if fact.context.contains('_') { continue; }
            let Some((item, rank)) = crate::company::item_ranked(ticker, &element_tag(&fact.element))
                .or_else(|| fact.label.parse::<Item>().ok().map(|i| (i, usize::MAX))) else { continue };
            let Some((date, p)) = context_period(&fact.context, kind, end) else { continue };
            let Some(val) = number(&fact.value) else { continue };
            // Units: "JPY", "JPYPerShares", "shares", and others for foreign
            // currency filers ("USD").
            let currency = match fact.unit.trim_end_matches("PerShares").parse::<Currency>() {
                Ok(c) => c,
                Err(()) if item.unit() == Unit::Shares => Currency::JPY,
                Err(()) => continue,
            };
            let key = (item, date, p, currency);
            match best.get(&key) {
                Some((r, _)) if *r <= rank => (),
                _ => { best.insert(key, (rank, val)); },
            }
        }
    }
    // "2025-06-18 09:00", Japan time: the day is when the market learned of it.
    let filed = d.submitted.as_deref().and_then(|s| s.get(..10)).and_then(|s| s.parse::<NaiveDate>().ok())
        .ok_or("report without a submission date")?;
    let filing = crate::item::Filed {
        accession: d.doc_id.clone(),
        filed,
        source: crate::item::Source::Edinet,
        form: d.doc_type.clone().unwrap_or_default(),
        period_end: d.period_end,
    };
    let mut items: Vec<Reported> = best.into_iter()
        .map(|((item, date, p, currency), (_, val))| Reported {
            ticker, date, p, item, currency, val, filing: Some(filing.clone()), derived: false,
        })
        .collect();
    items.sort_by_key(|r| (r.date, r.item, r.p));
    Ok(items)
}

/// Every report of a company in the cache, read oldest first, with how many
/// were not fetched yet.
pub fn read_company(cache: &edinet::Cache, ticker: Ticker) -> Result<(Vec<Reported>, usize, usize), Box<dyn Error>> {
    let reports = cache.reports(ticker.as_str())?;
    let (mut items, mut count, mut missing) = (Vec::new(), 0, 0);
    for d in &reports {
        match read(cache, d, ticker) {
            Ok(mut r) => { items.append(&mut r); count += 1; },
            Err(e) => match e.downcast_ref::<edinet::EdinetError>() {
                Some(edinet::EdinetError::NotFetched(_)) => missing += 1,
                _ => eprintln!("failed to read {} {}: {}", ticker, d.doc_id, e),
            },
        }
    }
    Ok((items, count, missing))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(s: &str) -> NaiveDate { s.parse().unwrap() }

    #[test]
    fn contexts() {
        let q2 = Kind::Quarterly(2);
        let end = date("2023-09-30");
        assert_eq!(context_period("CurrentQuarterInstant", q2, end), Some((end, Period::PointInTime)));
        assert_eq!(context_period("CurrentQuarterDuration", q2, end), Some((end, Period::ThreeMonths)));
        assert_eq!(context_period("CurrentYTDDuration", q2, end), Some((end, Period::SixMonths)));
        assert_eq!(context_period("Prior1YTDDuration", q2, end), Some((date("2022-09-30"), Period::SixMonths)));
        // 前期末 (zenki-matsu): the fiscal year that ended in March.
        assert_eq!(context_period("Prior1YearInstant", q2, end), Some((date("2023-03-31"), Period::PointInTime)));
        assert_eq!(context_period("Prior2YearInstant", q2, end), Some((date("2022-03-31"), Period::PointInTime)));
        assert_eq!(context_period("Prior1YearDuration", q2, end), Some((date("2023-03-31"), Period::TwelveMonths)));

        let fy = date("2025-03-31");
        assert_eq!(context_period("CurrentYearDuration", Kind::Annual, fy), Some((fy, Period::TwelveMonths)));
        assert_eq!(context_period("Prior1YearInstant", Kind::Annual, fy), Some((date("2024-03-31"), Period::PointInTime)));
        assert_eq!(context_period("Prior4YearDuration", Kind::Annual, fy), Some((date("2021-03-31"), Period::TwelveMonths)));
        assert_eq!(context_period("FilingDateInstant", Kind::Annual, fy), None);

        assert_eq!(element_tag("jpcrp040300-q2r_E02144-000:TotalNetRevenuesIFRS"), "E02144:TotalNetRevenuesIFRS");
        assert_eq!(element_tag("jpigp_cor:AssetsIFRS"), "jpigp_cor:AssetsIFRS");
        // February year ends stay month ends.
        assert_eq!(months_before(date("2024-02-29"), 12), date("2023-02-28"));
    }
}
