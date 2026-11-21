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
//! pages it names, e.g. `edgar/cache/<cik>/<accession>/` (see the `edgar` crate).

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use calamine::Data;
use regex::Regex;
use scraper::{ElementRef, Html, Selector};

use crate::item::{Item, Reported};
use crate::sheet_info::{new_sheet_info, SheetType};
use crate::Reader as R;
use crate::Ticker;

pub use edgar::{parse_filing_summary, Report};

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

/// Pick the first page whose name matches, skipping parenthetical pages (share
/// counts and par values). Statements are listed before the notes, so a note
/// like "Balance Sheet Components" never wins over the statement itself.
/// Pages the summary puts in a notes category are skipped: they can have
/// statement-like names ("Income Tax - ... in Consolidated Statements of
/// Financial Position (Details)"). Other categories are not trusted to mark
/// statements: 10-Ks filed in early 2022 list the balance sheet as
/// "Uncategorized".
fn find_report<'a>(reports: &'a [Report], matches: impl Fn(&str) -> bool) -> Option<&'a Report> {
    const NOT_STATEMENTS: [&str; 4] = ["Notes", "Details", "Policies", "Tables"];
    reports.iter().find(|r| {
        let name = r.short_name.to_lowercase();
        !r.category.as_deref().is_some_and(|c| NOT_STATEMENTS.contains(&c))
            && !name.contains("parenthetical") && matches(&name)
    })
}

pub fn balance_sheet(reports: &[Report]) -> Option<&Report> {
    find_report(reports, |n| {
        n.contains("balance sheet") || n.contains("financial condition") || n.contains("financial position")
    })
}

pub fn cash_flow_statement(reports: &[Report]) -> Option<&Report> {
    find_report(reports, |n| n.contains("cash flow"))
}

pub fn income_statement(reports: &[Report]) -> Option<&Report> {
    let is_income = |n: &str| {
        // Sysco titles it "Consolidated Results of Operations", without "statement".
        (n.contains("statement") && (n.contains("operations") || n.contains("income") || n.contains("earnings"))
            || n.starts_with("consolidated results of operations"))
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
    Ok(table(html)?.rows)
}

/// A statement table: its rows (see `table_rows`) and, for each row, the XBRL
/// element it reports, e.g. "us-gaap_NetCashProvidedByUsedInOperatingActivities".
pub struct Table {
    pub rows: Vec<Vec<Data>>,
    pub tags: Vec<Option<String>>,
}

static XBRL_TAG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"defref_([A-Za-z0-9-]+_[A-Za-z0-9]+)").unwrap());

pub fn table(html: &str) -> Result<Table, Box<dyn Error>> {
    let doc = Html::parse_document(html);
    let link_sel = Selector::parse("a[onclick]").unwrap();
    let mut tags: Vec<Option<String>> = Vec::new();
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
        if tags.len() <= r { tags.resize(r + 1, None); }
        // The row's label links to its XBRL definition: Show.showAR(this, 'defref_us-gaap_X', ...).
        tags[r] = tr.select(&link_sel)
            .filter_map(|a| a.value().attr("onclick"))
            .find_map(|js| XBRL_TAG.captures(js).map(|c| c[1].to_string()));
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

    tags.resize(grid.len(), None);
    Ok(Table {
        rows: grid.into_iter()
            .map(|row| row.into_iter().map(|v| v.unwrap_or(Data::Empty)).collect())
            .collect(),
        tags,
    })
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
    // The currency sign on a column's first row: "$ 20,431", "¥ 1,679,200".
    let t: String = text.chars().filter(|c| !matches!(c, '$' | '¥' | '€' | '£' | ',' | ' ')).collect();
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

    /// Read one statement page. Each row's item comes from its XBRL element
    /// (see `row_item`); the statement only reports items that belong on it.
    fn process(&self, report: Option<&Report>, sheet_type: SheetType, what: &str) -> Result<Vec<Reported>, Box<dyn Error>> {
        let report = report.ok_or_else(|| format!("{} not found in {:?}", what, self.dir))?;
        let table = table(&fs::read_to_string(self.dir.join(&report.file))?)?;
        let cash_flow = sheet_type == SheetType::CashFlowStatement;
        let (rows, mut items): (Vec<&[Data]>, Vec<Option<(Item, usize)>>) = table.rows.iter()
            .zip(&table.tags)
            .filter(|(row, _)| !row.iter().all(|c| matches!(c, Data::Empty)))
            .map(|(row, tag)| {
                let item = row_item(tag.as_deref(), label(row)).filter(|(i, _)| i.is_cash_flow() == cash_flow);
                (row.as_slice(), item)
            })
            .unzip();
        drop_segment_rows(&rows, &mut items);
        let sheet_info = new_sheet_info(&rows, sheet_type)?;
        let mut reported = crate::xlsx::Reader::reported_items_with(self.ticker, &rows, sheet_info, |r, _| items[r])?;
        crate::calibrate_shares(&mut reported);
        Ok(reported)
    }

    /// 10-Qs only report cash flows year to date (3, 6, 9 months); see
    /// `derive_quarters` for the quarterly values.
    pub fn process_cash_flow_statement(&mut self) -> Result<Vec<Reported>, Box<dyn Error>> {
        self.process(cash_flow_statement(&self.reports), SheetType::CashFlowStatement, "cash flow statement")
    }
}

fn label<'a>(row: &'a [Data]) -> Option<&'a str> {
    row.first().and_then(|c| match c { Data::String(s) => Some(s.as_str()), _ => None })
}

/// After the statement itself, EDGAR's pages repeat lines per segment, under
/// headers like "Insurance and Other [Member]": the same element, qualified
/// by a member of some axis.
///
/// Where the member itself names an item (TSLA's "Operating Lease Vehicles
/// [Member]" and "Solar Energy Systems [Member]", whose rows are both labeled
/// "Operating lease net"), the section's rows report that item. Otherwise a
/// segment row is only kept for an item the statement does not report itself
/// and that appears in just one segment. Where an item is split across
/// segments (Berkshire's cash, by business group) no single row is the
/// company's total, so none is used.
fn drop_segment_rows(rows: &[&[Data]], items: &mut [Option<(Item, usize)>]) {
    use std::collections::{HashMap, HashSet};
    let mut segment: Vec<Option<usize>> = Vec::with_capacity(rows.len());
    let mut member_items: HashMap<usize, Item> = HashMap::new();
    let mut current = None;
    for (r, row) in rows.iter().enumerate() {
        if let Some(l) = label(row).filter(|l| l.ends_with("[Member]")) {
            current = Some(r);
            if let Ok(item) = l.parse::<Item>() {
                member_items.insert(r, item);
            }
        }
        segment.push(current);
    }
    for (i, s) in items.iter_mut().zip(&segment) {
        if let (Some(_), Some(item)) = (*i, s.and_then(|s| member_items.get(&s))) {
            *i = Some((*item, 0));
        }
    }
    let face: HashSet<Item> = items.iter().zip(&segment)
        .filter(|(_, s)| s.is_none())
        .filter_map(|(i, _)| i.map(|(item, _)| item))
        .collect();
    let mut sections: HashMap<Item, HashSet<usize>> = HashMap::new();
    for (i, s) in items.iter().zip(&segment) {
        if let (Some((item, _)), Some(s)) = (i, s) {
            sections.entry(*item).or_default().insert(*s);
        }
    }
    for (i, s) in items.iter_mut().zip(&segment) {
        if let (Some((item, _)), Some(_)) = (*i, s) {
            if face.contains(&item) || sections[&item].len() > 1 {
                *i = None;
            }
        }
    }
}

/// A company's own element rather than one from a standard taxonomy (US GAAP,
/// or IFRS for foreign filers such as TSMC).
fn is_extension(tag: &str) -> bool {
    !tag.starts_with("us-gaap_") && !tag.starts_with("ifrs-full_")
}

/// The item a statement row reports. Its XBRL element decides when it is a
/// mapped one: labels vary between companies and years, elements do not, and
/// labels are ambiguous (AAPL's current and non-current marketable securities
/// are both "Marketable securities"). An unmapped us-gaap element reports
/// nothing we track. Company extension elements and dimension rows (segment
/// axes and members) fall back to the label.
fn row_item(tag: Option<&str>, label: Option<&str>) -> Option<(Item, usize)> {
    let by_label = || label.and_then(|l| l.parse::<Item>().ok()).map(|i| (i, 0));
    match tag {
        None => by_label(),
        Some(tag) => match Item::from_xbrl_tag_ranked(tag) {
            Some(item) => Some(item),
            None if is_extension(tag) || tag.ends_with("Axis") || tag.ends_with("Member") => by_label(),
            None => None,
        },
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
