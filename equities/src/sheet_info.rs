use std::collections::HashMap;
use std::error::Error;

use crate::{Currency, Period};

static COLUMN_CURRENCY: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new(r"^(.*?\d{4})\s+([A-Z]{3}) \(.*$").unwrap()
});
use crate::item::Item;

use calamine::{Data, DataType};
use chrono::{Datelike, NaiveDate};

#[derive(PartialEq)]
pub enum SheetType {
    BalanceSheet,
    IncomeStatement,
    CashFlowStatement,
}

pub struct SheetInfo {
    pub dates_and_periods: HashMap<usize, (NaiveDate, Period)>,
    pub labels: usize,
    pub multiplier: f64,
    /// Scale of share counts, which headers state separately from dollars.
    pub share_multiplier: f64,
    pub sheet_type: SheetType,
    /// Currency of each date column whose header names one ("Dec. 31, 2025
    /// TWD ($)"); columns without one are in US dollars.
    pub currencies: HashMap<usize, Currency>,
}

/// The dollar scale stated in a header cell. EDGAR headers name the share and
/// dollar scales separately ("shares in Thousands, $ in Millions"), so the
/// dollar one takes priority; plain "USD ($)" with no scale means dollars.
/// A scale that only applies to shares ("Share data in Thousands") is ignored.
fn scale(s: &str) -> Option<f64> {
    let mut s = s.to_lowercase();
    for share_scale in ["shares in thousands", "shares in millions", "share data in thousands", "share data in millions"] {
        s = s.replace(share_scale, "");
    }
    if s.contains("$ in millions") { Some(1_000_000.0) }
    else if s.contains("$ in thousands") { Some(1_000.0) }
    else if s.contains("in millions") { Some(1_000_000.0) }
    else if s.contains("in thousands") { Some(1_000.0) }
    else if s.contains("usd ($)") || s.contains("(usd $)") { Some(1.0) }
    else { None }
}

static SHARE_SCALE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new(r"shares? (?:data )?in (millions|thousands|billions)").unwrap()
});

fn named_scale(s: &str) -> Option<f64> {
    if s.contains("billions") { Some(1_000_000_000.0) }
    else if s.contains("millions") { Some(1_000_000.0) }
    else if s.contains("thousands") { Some(1_000.0) }
    else { None }
}

/// The scale of share counts stated in a header cell. EDGAR writes it in
/// two styles:
///   "shares in Thousands, $ in Millions": each unit's scale is named, and
///     shares not named ("$ in Millions") are in units;
///   "In Millions, except Per Share data": one scale for everything but
///     per-share amounts, so share counts are in millions too, unless they
///     are excepted as well ("except Share data", "except Share data in
///     Thousands").
fn share_scale(s: &str) -> Option<f64> {
    let s = s.to_lowercase();
    if let Some(c) = SHARE_SCALE.captures(&s) { return named_scale(&c[1]); }
    if s.contains("except share") || s.contains("except shares") { return Some(1.0); }
    if s.contains("$ in ") { return Some(1.0); }
    if s.contains(" in millions") || s.contains(" in thousands") || s.contains(" in billions")
        || s.starts_with("in millions") || s.starts_with("in thousands") { return named_scale(&s); }
    if s.contains("usd ($)") || s.contains("(usd $)") { return Some(1.0); }
    None
}

/// A header cell naming a period ("8 Months Ended", "52 Weeks Ended"), as
/// opposed to a date or a title.
fn is_period_heading(s: &str) -> bool {
    ["month", "week", "year", "quarter"].iter().any(|unit| s.contains(unit))
        && (s.contains("ended") || s.contains("months") || s.contains("weeks"))
}

/// A header date: "Oct. 26, 2025", "October 26 2025", "10/26/2025", "2025-10-26".
fn parse_date_str(s: &str) -> Option<NaiveDate> {
    let s = s.trim().replace(",", "");
    let formats = ["%B %d %Y", "%b %d %Y", "%m/%d/%Y", "%Y-%m-%d", "%b. %d %Y"];
    formats.iter().find_map(|fmt| NaiveDate::parse_from_str(&s, fmt).ok())
}

/// A date split over two rows, "October 26," above the year (NVDA's own
/// spreadsheets).
fn parse_date_across_cells(s: &str, next_cell: Option<&Data>) -> Option<NaiveDate> {
    if s.trim().ends_with(',') || s.trim().split_whitespace().count() >= 2 {
        if let Some(year) = next_cell.and_then(|c| match c {
            Data::Float(f) => Some(*f as i32),
            Data::Int(i) => Some(*i as i32),
            _ => None,
        }).filter(|&y| y > 1900 && y < 2100) {
            return parse_date_str(&format!("{} {}", s, year))
        }
    }
    None
}

pub fn new_sheet_info(rows: &[&[Data]], sheet_type: SheetType) -> Result<SheetInfo, Box<dyn Error>> {

    let mut is_10k = false;
    let mut multiplier = 0.0;
    let mut share_multiplier: Option<f64> = None;

    let mut col0_count = 0;
    let mut col1_count = 0;

    // Period headings by column: a period, or None for a heading that names a
    // period we do not recognize ("2 Months Ended"). Columns under those are
    // dropped rather than guessed.
    let mut periods: HashMap<usize, Option<Period>> = HashMap::new();
    let mut unknown_headings: HashMap<usize, String> = HashMap::new();
    let mut unknown_cols: std::collections::HashSet<usize> = std::collections::HashSet::new();
    // "4 Months Ended" headings, and the date columns under them: a 16-week
    // first quarter on a 52/53-week calendar (Kroger's), or a four-month stub
    // after a merger. Told apart below by the column dates.
    let mut four_month_heads: std::collections::HashSet<usize> = std::collections::HashSet::new();
    let mut four_month_cols: std::collections::HashSet<usize> = std::collections::HashSet::new();

    // Period headings ("Three Months Ended") with their row and column, and the
    // first row holding a date, to tell header cells from line items.
    let mut header_periods: Vec<(usize, usize, Option<Period>)> = Vec::new();
    let mut first_date_row: Option<usize> = None;

    let mut currencies: HashMap<usize, Currency> = HashMap::new();
    let mut skipped: std::collections::HashSet<usize> = std::collections::HashSet::new();

    let mut dates_and_periods = HashMap::new();
    for (r, row) in rows.iter().enumerate() {

        let row: &[Data] = row;
        if let Some(s) = row.get(0).and_then(|c| c.get_string()) {
            if s.parse::<Item>().is_ok() { col0_count += 1; }
        }
        if let Some(s) = row.get(1).and_then(|c| c.get_string()) {
            if s.parse::<Item>().is_ok() { col1_count += 1; }
        }

        for (c, cell) in row.iter().enumerate() {
            match cell {
                Data::String(s) => {
                    // A currency after a date: "Dec. 31, 2025 TWD ($)", possibly
                    // followed by other units ("... TWD ($) $ / shares").
                    let (s, currency) = match COLUMN_CURRENCY.captures(s) {
                        Some(c) => (c[1].to_string(), Some(c[2].to_string())),
                        None => (s.to_string(), None),
                    };
                    let s = s.to_lowercase();

                    if FOUR_MONTHS.is_match(&s) && !matches!(periods.get(&c), Some(Some(_))) {
                        periods.insert(c, Some(Period::ThreeMonths));
                        header_periods.push((r, c, Some(Period::ThreeMonths)));
                        unknown_headings.insert(c, s.clone());
                        four_month_heads.insert(c);
                    } else if let Ok(p) = s.parse::<Period>() {
                        periods.insert(c, Some(p));
                        header_periods.push((r, c, Some(p)));
                    } else if c > 0 && first_date_row.is_none_or(|d| r <= d) && is_period_heading(&s)
                        // A heading split over rows ("Three Months" / "Ended") is
                        // already recognized from its first part.
                        && !matches!(periods.get(&c), Some(Some(_))) {
                        periods.insert(c, None);
                        header_periods.push((r, c, None));
                        unknown_headings.insert(c, s.clone());
                    }

                    if s.contains("form type: 10-k") || s.contains("12 months ended") {
                        is_10k = true;
                    }

                    // Scales are stated in the header. Row labels mention units
                    // too ("Weighted average shares (in thousands)"), which must
                    // not rescale the dollars.
                    if first_date_row.is_none_or(|d| r <= d) {
                        if let Some(m) = scale(&s) { if m != 1.0 || multiplier == 0.0 { multiplier = m; } }
                        if share_multiplier.is_none() { share_multiplier = share_scale(&s); }
                    }

                    if let Some(date) = parse_date_str(&s).or_else(|| parse_date_across_cells(&s, rows.get(r+1).and_then(|next| next.get(c)))) {
                        first_date_row.get_or_insert(r);
                        match currency.as_deref().map(str::parse::<Currency>) {
                            Some(Ok(cur)) => { currencies.insert(c, cur); },
                            // A currency we do not know: leave the column out
                            // rather than read it as dollars.
                            Some(Err(())) => { skipped.insert(c); continue; },
                            None => (),
                        }
                        match sheet_type {
                            SheetType::BalanceSheet => {
                                dates_and_periods.entry(c).or_insert((date, Period::PointInTime));
                            },
                            _ => {
                                let mut p = None;
                                for offset in (0..=c).rev().take(4) {
                                    if let Some(detected_p) = periods.get(&offset) {
                                        p = Some(*detected_p);
                                        if four_month_heads.contains(&offset) { four_month_cols.insert(c); }
                                        break;
                                    }
                                }
                                // No heading at all (NVDA's own spreadsheets): the
                                // form's usual period. An unrecognized heading: drop.
                                let p = match p {
                                    Some(Some(p)) => p,
                                    Some(None) => { unknown_cols.insert(c); Period::ThreeMonths },
                                    None => if is_10k { Period::TwelveMonths } else { Period::ThreeMonths },
                                };
                                dates_and_periods.entry(c).or_insert((date, p));
                            },
                        }
                    }
                },
                _ => continue,
            }
        }
    }

    dates_and_periods.retain(|c, _| !skipped.contains(c));
    if dates_and_periods.is_empty() { return Err("no dates found".into()); }

    // Each period heading spans an equal group of date columns, in order, but
    // may sit over any column of its group: NVDA's 2022 sheets put "Six Months
    // Ended" over the second of its two columns, so looking left from the
    // first one finds "Three Months Ended". Where the date columns divide
    // evenly among the headings, assign them by group instead.
    if sheet_type != SheetType::BalanceSheet {
        if let Some(first_date_row) = first_date_row {
            let mut heads: Vec<(usize, Option<Period>)> = header_periods.iter()
                .filter(|(r, _, _)| *r <= first_date_row)
                .map(|&(_, c, p)| (c, p))
                .collect();
            heads.sort_by_key(|&(c, _)| c);
            heads.dedup_by_key(|&mut (c, _)| c);
            let mut cols: Vec<usize> = dates_and_periods.keys().copied().collect();
            cols.sort();
            // Only where headings span several columns: with one heading per
            // column, the nearest heading to the left is already exact (NVDA's
            // 1999-2002 sheets head every column, including a one-month
            // transition period).
            // And only when each heading sits within its group of columns:
            // NVDA's 2002 sheet heads columns 2-5 "Year Ended", 6 "Month
            // Ended" and 7 "Year Ended", which do not split evenly.
            let even = !heads.is_empty() && cols.len() % heads.len() == 0 && cols.len() / heads.len() > 1;
            let group = if even { cols.len() / heads.len() } else { 0 };
            let within = even && heads.iter().enumerate()
                .all(|(g, (hc, _))| cols[g * group] <= *hc && *hc <= cols[g * group + group - 1]);
            if within {
                for (i, c) in cols.iter().enumerate() {
                    if four_month_heads.contains(&heads[i / group].0) { four_month_cols.insert(*c); }
                    else { four_month_cols.remove(c); }
                    match heads[i / group].1 {
                        Some(p) => {
                            unknown_cols.remove(c);
                            if let Some(entry) = dates_and_periods.get_mut(c) { entry.1 = p; }
                        },
                        None => { unknown_cols.insert(*c); },
                    }
                }
            }
        }
    }

    // A 52/53-week year ends on a weekday, rarely the last of a month; a stub
    // period of a calendar-year company ends on a month end. Any month end
    // among the four-month columns makes them all a stub, which is no quarter.
    let month_end = |d: NaiveDate| d.succ_opt().is_some_and(|n| n.day() == 1);
    if four_month_cols.iter().any(|c| dates_and_periods.get(c).is_some_and(|(d, _)| month_end(*d))) {
        unknown_cols.extend(four_month_cols.iter().copied());
    } else {
        unknown_headings.retain(|c, _| !four_month_heads.contains(c));
    }

    if !unknown_cols.is_empty() {
        let mut headings: Vec<&str> = unknown_headings.values().map(String::as_str).collect();
        headings.sort();
        headings.dedup();
        eprintln!("skipping {} column(s) under unrecognized period heading(s) {:?}", unknown_cols.len(), headings);
        dates_and_periods.retain(|c, _| !unknown_cols.contains(c));
        if dates_and_periods.is_empty() { return Err("no columns with a recognized period".into()); }
    }

    let labels = if col0_count >= col1_count { 0 } else { 1 };

    Ok(SheetInfo {
        dates_and_periods: dates_and_periods,
        labels: labels,
        multiplier: multiplier,
        share_multiplier: share_multiplier.unwrap_or(1.0),
        sheet_type: sheet_type,
        currencies: currencies,
    })
}

#[cfg(test)]
mod tests {
    use super::{scale, share_scale};

    #[test]
    fn share_scales() {
        // (header, dollar scale, share scale), from EDGAR's statement pages
        let cases = [
            ("Consolidated Statements of Income - USD ($) shares in Millions, $ in Millions", 1e6, 1e6),
            ("Consolidated Statements of Income - USD ($) shares in Thousands, $ in Millions", 1e6, 1e3),
            ("Consolidated Statements of Income - USD ($) $ in Millions", 1e6, 1.0),
            ("Income Statement (USD $) In Millions, except Per Share data, unless otherwise specified", 1e6, 1e6),
            ("Income Statement (USD $) In Millions, except Share data in Thousands, unless otherwise specified", 1e6, 1e3),
            ("Income Statement (USD $) In Thousands, except Share data, unless otherwise specified", 1e3, 1.0),
            ("Income Statement (USD $) Share data in Thousands, except Per Share data", 1.0, 1e3),
            ("Income Statement (USD $)", 1.0, 1.0),
        ];
        for (header, dollars, shares) in cases {
            let h = header.to_lowercase();
            assert_eq!(scale(&h), Some(dollars), "{}", header);
            assert_eq!(share_scale(&h), Some(shares), "{}", header);
        }
    }
}

static FOUR_MONTHS: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new(r"^\s*(4|four) months? ended").unwrap()
});
