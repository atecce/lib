//! Reads statements from an EDGAR filing's rendered XBRL pages.
//!
//! For each 10-Q/10-K, EDGAR renders every financial statement as an HTML page
//! (`R2.htm`, `R4.htm`, ...) and lists them in `FilingSummary.xml`. These pages
//! exist for filings the SEC never built a `Financial_Report.xlsx` for (before
//! mid-2013 and after mid-2025), and they have the same layout as the xlsx
//! sheets: a label column, an optional footnote column, then one column per
//! period. This reader converts a page's table into the same rows the xlsx
//! reader parses and reuses its parsing.
//!
//! A filing is read from a directory holding `FilingSummary.xml` and the `R`
//! pages it names, e.g. `edgar/<cik>/<accession>/`.

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use calamine::Data;
use regex::Regex;
use scraper::{ElementRef, Html, Selector};

use crate::item::Reported;
use crate::sheet_info::{new_sheet_info, SheetType};
use crate::Reader as R;
use crate::Ticker;

/// One statement page listed in `FilingSummary.xml`.
#[derive(Debug, Clone)]
pub struct Report {
    pub file: String,
    pub short_name: String,
}

pub struct Reader {
    dir: PathBuf,
    ticker: Ticker,
    reports: Vec<Report>,
}

pub fn new_reader(dir: &Path, ticker: Ticker) -> Result<Reader, Box<dyn Error>> {
    let summary = fs::read_to_string(dir.join("FilingSummary.xml"))?;
    Ok(Reader {
        dir: dir.to_path_buf(),
        ticker,
        reports: parse_filing_summary(&summary),
    })
}

static REPORT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?s)<Report\b.*?</Report>").unwrap());

fn tag(block: &str, name: &str) -> Option<String> {
    let open = format!("<{}>", name);
    let close = format!("</{}>", name);
    let start = block.find(&open)? + open.len();
    let end = block[start..].find(&close)? + start;
    Some(block[start..end].trim().to_string())
}

/// The pages listed in `FilingSummary.xml`, in filing order (cover page, then
/// statements, then notes).
pub fn parse_filing_summary(xml: &str) -> Vec<Report> {
    REPORT.find_iter(xml)
        .filter_map(|m| {
            let block = m.as_str();
            Some(Report {
                file: tag(block, "HtmlFileName")?,
                short_name: tag(block, "ShortName").unwrap_or_default(),
            })
        })
        .collect()
}

/// Pick the first page whose name matches, skipping parenthetical pages (share
/// counts and par values). Statements are listed before the notes, so a note
/// like "Balance Sheet Components" never wins over the statement itself.
fn find_report<'a>(reports: &'a [Report], matches: impl Fn(&str) -> bool) -> Option<&'a Report> {
    reports.iter().find(|r| {
        let name = r.short_name.to_lowercase();
        !name.contains("parenthetical") && matches(&name)
    })
}

pub fn balance_sheet(reports: &[Report]) -> Option<&Report> {
    find_report(reports, |n| {
        n.contains("balance sheet") || n.contains("financial condition") || n.contains("financial position")
    })
}

pub fn income_statement(reports: &[Report]) -> Option<&Report> {
    let is_income = |n: &str| {
        n.contains("statement") && (n.contains("operations") || n.contains("income") || n.contains("earnings"))
            && !n.contains("cash flow") && !n.contains("equity") && !n.contains("stockholders")
    };
    find_report(reports, |n| is_income(n) && !n.contains("comprehensive"))
        .or_else(|| find_report(reports, is_income))
}

/// The statement table of an `R` page as rows of cells, with `colspan` and
/// `rowspan` expanded so every period lines up in its own column. A spanning
/// cell's text goes in its first column and the rest are left empty, matching
/// how merged cells read from the xlsx.
pub fn table_rows(html: &str) -> Result<Vec<Vec<Data>>, Box<dyn Error>> {
    let doc = Html::parse_document(html);
    let table_sel = Selector::parse("table.report").unwrap();
    let table = doc.select(&table_sel).next().ok_or("no report table")?;

    let mut grid: Vec<Vec<Option<Data>>> = Vec::new();

    // Only the report table's own rows: footnotes sit in a nested table.
    let rows = table.children()
        .filter_map(ElementRef::wrap)
        .flat_map(|el| if el.value().name() == "tbody" {
            el.children().filter_map(ElementRef::wrap).collect::<Vec<_>>()
        } else {
            vec![el]
        })
        .filter(|el| el.value().name() == "tr");

    for (r, tr) in rows.enumerate() {
        if grid.len() <= r { grid.resize(r + 1, Vec::new()); }
        let mut c = 0;
        for cell in tr.children().filter_map(ElementRef::wrap) {
            let name = cell.value().name();
            if name != "td" && name != "th" { continue; }
            // Skip columns already filled by a rowspan from above.
            while grid[r].get(c).is_some_and(|v| v.is_some()) { c += 1; }

            let colspan: usize = cell.value().attr("colspan").and_then(|v| v.parse().ok()).unwrap_or(1);
            let rowspan: usize = cell.value().attr("rowspan").and_then(|v| v.parse().ok()).unwrap_or(1);
            let value = cell_value(&cell, name == "th");

            for dr in 0..rowspan {
                if grid.len() <= r + dr { grid.resize(r + dr + 1, Vec::new()); }
                let row = &mut grid[r + dr];
                if row.len() < c + colspan { row.resize(c + colspan, None); }
                for dc in 0..colspan {
                    row[c + dc] = Some(if dr == 0 && dc == 0 { value.clone() } else { Data::Empty });
                }
            }
            c += colspan;
        }
    }

    Ok(grid.into_iter()
        .map(|row| row.into_iter().map(|v| v.unwrap_or(Data::Empty)).collect())
        .collect())
}

/// A cell's text with whitespace collapsed. Header cells and labels stay
/// strings; amounts like "$ 8,771" or "(1,234)" become numbers.
fn cell_value(cell: &ElementRef, header: bool) -> Data {
    let mut parts = Vec::new();
    visible_text(cell, &mut parts);
    let text = parts.join(" ").split_whitespace().collect::<Vec<_>>().join(" ");
    if text.is_empty() {
        return Data::Empty;
    }
    if !header {
        if let Some(n) = parse_amount(&text) {
            return Data::Float(n);
        }
    }
    Data::String(text)
}

/// Text a browser would show. Some renderer versions put each value's XBRL tag
/// in a hidden span next to it (`<span style="display:none">us-gaap_...</span>`).
fn visible_text<'a>(el: &ElementRef<'a>, out: &mut Vec<&'a str>) {
    for child in el.children() {
        if let Some(text) = child.value().as_text() {
            out.push(text);
        } else if let Some(child) = ElementRef::wrap(child) {
            let hidden = child.value().attr("style")
                .is_some_and(|s| s.replace(' ', "").contains("display:none"));
            if !hidden {
                visible_text(&child, out);
            }
        }
    }
}

fn parse_amount(text: &str) -> Option<f64> {
    let t: String = text.chars().filter(|c| !matches!(c, '$' | ',' | ' ')).collect();
    let (negative, digits) = match t.strip_prefix('(').and_then(|t| t.strip_suffix(')')) {
        Some(inner) => (true, inner),
        None => (false, t.as_str()),
    };
    if digits.is_empty() || !digits.chars().all(|c| c.is_ascii_digit() || c == '.' || c == '-') {
        return None;
    }
    let n: f64 = digits.parse().ok()?;
    Some(if negative { -n } else { n })
}

impl Reader {
    pub fn reports(&self) -> &[Report] {
        &self.reports
    }

    fn process(&self, report: Option<&Report>, sheet_type: SheetType, what: &str) -> Result<Vec<Reported>, Box<dyn Error>> {
        let report = report.ok_or_else(|| format!("{} not found in {:?}", what, self.dir))?;
        let html = fs::read_to_string(self.dir.join(&report.file))?;
        let grid = table_rows(&html)?;
        let rows: Vec<&[Data]> = grid.iter()
            .map(|row| row.as_slice())
            .filter(|row| !row.iter().all(|c| matches!(c, Data::Empty)))
            .collect();
        crate::xlsx::Reader::reported_items(self.ticker, &rows, new_sheet_info(&rows, sheet_type)?)
    }
}

impl R for Reader {
    fn process_balance_sheet(&mut self) -> Result<Vec<Reported>, Box<dyn Error>> {
        self.process(balance_sheet(&self.reports), SheetType::BalanceSheet, "balance sheet")
    }

    fn process_income_statement(&mut self) -> Result<Vec<Reported>, Box<dyn Error>> {
        self.process(income_statement(&self.reports), SheetType::IncomeStatement, "income statement")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amounts() {
        assert_eq!(parse_amount("$ 8,771"), Some(8771.0));
        assert_eq!(parse_amount("(1,234)"), Some(-1234.0));
        assert_eq!(parse_amount("$ (0.45)"), Some(-0.45));
        assert_eq!(parse_amount("[1]"), None);
        assert_eq!(parse_amount("Sep. 30, 2020"), None);
    }

    #[test]
    fn spans() {
        let html = r#"<table class="report">
            <tr><th colspan="2" rowspan="2">Title - USD ($) $ in Millions</th><th colspan="2">3 Months Ended</th></tr>
            <tr><th>Sep. 30, 2020</th><th>Sep. 30, 2019</th></tr>
            <tr><td>Total revenues</td><td><sup></sup></td><td>$ 8,771</td><td>(6,303)</td></tr>
            <tr><td colspan="4"><table class="outerFootnotes"><tr><td>[1]</td><td>note</td></tr></table></td></tr>
        </table>"#;
        let rows = table_rows(html).unwrap();
        assert_eq!(rows[0], vec![Data::String("Title - USD ($) $ in Millions".into()), Data::Empty, Data::String("3 Months Ended".into()), Data::Empty]);
        assert_eq!(rows[1], vec![Data::Empty, Data::Empty, Data::String("Sep. 30, 2020".into()), Data::String("Sep. 30, 2019".into())]);
        assert_eq!(rows[2], vec![Data::String("Total revenues".into()), Data::Empty, Data::Float(8771.0), Data::Float(-6303.0)]);
        assert_eq!(rows.len(), 4);
    }

    #[test]
    fn hidden_xbrl_tags() {
        let html = r#"<table class="report"><tr><td>Cash</td><td class="nump"><a>$ 19,478</a><span style="display:none;white-space:normal;">us-gaap_Cash</span><span></span></td></tr></table>"#;
        assert_eq!(table_rows(html).unwrap()[0], vec![Data::String("Cash".into()), Data::Float(19478.0)]);
    }
}
