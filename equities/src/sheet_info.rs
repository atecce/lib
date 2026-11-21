use std::collections::HashMap;
use std::error::Error;

use crate::date::{parse_date_str, parse_date_across_cells, parse_date_across_lines};
use crate::{Currency, Period};

static COLUMN_CURRENCY: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new(r"^(.*?\d{4})\s+([A-Z]{3}) \(.*$").unwrap()
});
use crate::item::Item;

use calamine::{Data, DataType};
use chrono::NaiveDate;

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

/// A header cell naming a period ("8 Months Ended", "52 Weeks Ended"), as
/// opposed to a date or a title.
fn is_period_heading(s: &str) -> bool {
    ["month", "week", "year", "quarter"].iter().any(|unit| s.contains(unit))
        && (s.contains("ended") || s.contains("months") || s.contains("weeks"))
}

pub fn new_sheet_info(rows: &[&[Data]], sheet_type: SheetType) -> Result<SheetInfo, Box<dyn Error>> {

    let mut is_10k = false;
    let mut multiplier = 0.0;

    let mut col0_count = 0;
    let mut col1_count = 0;

    // Period headings by column: a period, or None for a heading that names a
    // period we do not recognize ("2 Months Ended"). Columns under those are
    // dropped rather than guessed.
    let mut periods: HashMap<usize, Option<Period>> = HashMap::new();
    let mut unknown_headings: HashMap<usize, String> = HashMap::new();
    let mut unknown_cols: std::collections::HashSet<usize> = std::collections::HashSet::new();

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

                    if let Ok(p) = s.parse::<Period>() {
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

                    if let Some(m) = scale(&s) { if m != 1.0 || multiplier == 0.0 { multiplier = m; } }

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
        sheet_type: sheet_type,
        currencies: currencies,
    })
}

pub fn new_sheet_info_from_pdf(rows: &Vec<Vec<Option<String>>>, sheet_type: SheetType) -> Result<SheetInfo, Box<dyn Error>> {

    let mut is_10k = false;
    let mut multiplier = 0.0;

    let mut periods = HashMap::new();

    let mut dates_and_periods = HashMap::new();
    for (r, row) in rows.iter().enumerate() {

        for (c, cell) in row.iter().enumerate() {
            if let Some(s) = cell {
                let s = s.to_lowercase();

                if let Ok(p) = s.parse::<Period>() {
                    periods.insert(c, p);
                }

                if s.contains("form type: 10-k") || s.contains("12 months ended") {
                    is_10k = true;
                }

                if let Some(m) = scale(&s) { if m != 1.0 || multiplier == 0.0 { multiplier = m; } }

                let next_line_opt = rows.get(r + 1)
                    .and_then(|next_row| next_row.get(c))
                    .and_then(|cell| cell.as_ref())
                    .map(|s| s.as_str());

                if let Some(date) = parse_date_str(&s).or_else(|| {
                    let next_line = next_line_opt?; // Converts Result to Option
                    parse_date_across_lines(&s, next_line).ok()
                }) {

                    match sheet_type {
                        SheetType::BalanceSheet => {
                            dates_and_periods.entry(c).or_insert((date, Period::PointInTime));
                        },
                        _ => {
                            let mut p = None;
                            for offset in (0..=c).rev().take(4) {
                                if let Some(detected_p) = periods.get(&offset) {
                                    p = Some(*detected_p);
                                    break;
                                }
                            }
                            dates_and_periods.entry(c).or_insert((date, p.unwrap_or(if is_10k { Period::TwelveMonths } else { Period::ThreeMonths })));
                        },
                    }
                }
            }
        }
    }

    if dates_and_periods.is_empty() { return Err("no dates found".into()); }

    Ok(SheetInfo {
        dates_and_periods: dates_and_periods,
        labels: 0,
        multiplier: multiplier,
        sheet_type: sheet_type,
        currencies: HashMap::new(),
    })
}
