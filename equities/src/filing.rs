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
use std::sync::LazyLock;

use calamine::Data;
use regex::Regex;
use scraper::{ElementRef, Html, Selector};

use crate::item::{Item, Reported};
use crate::sheet_info::{new_sheet_info, SheetType};
use crate::Reader as R;
use crate::Ticker;

pub use edgar::{parse_filing_summary, Report};

/// A page's text by its file name (`R2.htm`), from wherever the caller keeps
/// the filing: the reader asks only for the pages it reads.
pub type Pages = Box<dyn Fn(&str) -> Result<String, Box<dyn Error>>>;

pub struct Reader {
    /// What the filing is called in messages (its folder, its accession).
    name: String,
    ticker: Ticker,
    reports: Vec<Report>,
    pages: Pages,
    /// What the reader left out and why, for the caller to report.
    warnings: std::cell::RefCell<Vec<String>>,
}

/// A filing from its index (FilingSummary.xml's text) and its pages: no
/// files, no network; `pages` is the caller's.
pub fn reader(name: impl Into<String>, summary: &str, ticker: Ticker, pages: Pages) -> Reader {
    Reader { name: name.into(), ticker, reports: parse_filing_summary(summary), pages, warnings: Default::default() }
}

/// A filing from a folder on disk (an EDGAR cache's accession folder, or a
/// test's): `reader` with its files. The `files` feature (on by default).
#[cfg(feature = "files")]
pub fn new_reader(dir: &std::path::Path, ticker: Ticker) -> Result<Reader, Box<dyn Error>> {
    let summary = std::fs::read_to_string(dir.join("FilingSummary.xml"))?;
    let folder = dir.to_path_buf();
    let pages: Pages = Box::new(move |file| Ok(std::fs::read_to_string(folder.join(file))?));
    Ok(reader(dir.display().to_string(), &summary, ticker, pages))
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

    /// What reading left out so far (columns under headings with no known
    /// period), for the caller to report: the reader prints nothing.
    pub fn warnings(&self) -> Vec<String> {
        self.warnings.borrow().clone()
    }

    /// Read one statement page. Each row's item comes from its XBRL element
    /// (see `row_item`); the statement only reports items that belong on it.
    fn process(&self, report: Option<&Report>, sheet_type: SheetType, what: &str) -> Result<Vec<Reported>, Box<dyn Error>> {
        let report = report.ok_or_else(|| format!("{} not found in {}", what, self.name))?;
        let table = table(&(self.pages)(&report.file)?)?;
        let cash_flow = sheet_type == SheetType::CashFlowStatement;
        let (rows, (tags, mut items)): (Vec<&[Data]>, (Vec<Option<&str>>, Vec<Option<(Item, usize)>>)) = table.rows.iter()
            .zip(&table.tags)
            .filter(|(row, _)| !row.iter().all(|c| matches!(c, Data::Empty)))
            .map(|(row, tag)| {
                let item = row_item(self.ticker, tag.as_deref(), label(row)).filter(|(i, _)| i.is_cash_flow() == cash_flow);
                (row.as_slice(), (tag.as_deref(), item))
            })
            .unzip();
        drop_segment_rows(&rows, &tags, &mut items);
        let balance_sheet = sheet_type == SheetType::BalanceSheet;
        let sheet_info = new_sheet_info(&rows, sheet_type)?;
        if !sheet_info.skipped.is_empty() {
            self.warnings.borrow_mut().push(format!("{} ({}): skipped columns under unrecognized period heading(s) {:?}",
                report.file, what, sheet_info.skipped));
        }
        let mut debt = if balance_sheet { total_debt(self.ticker, &rows, &tags, &sheet_info) } else { Vec::new() };
        let mut reported = crate::xlsx::Reader::reported_items_with(self.ticker, &rows, sheet_info, |r, _| items[r])?;
        reported.append(&mut debt);
        crate::calibrate_shares(&mut reported);
        Ok(reported)
    }

    /// Shares outstanding from the cover page, every class with a claim on
    /// the company summed (not special voting shares), as of the
    /// date the cover gives (a 20-F's: the fiscal year end). Berkshire's
    /// Class A shares count as the 1,500 Class B shares each converts into,
    /// so the total prices at the Class B quote.
    pub fn process_cover(&self) -> Result<Vec<Reported>, Box<dyn Error>> {
        let report = self.reports.iter()
            .find(|r| r.category.as_deref() == Some("Cover") || r.kind.as_deref() == Some("Document"))
            .ok_or("no cover page")?;
        let table = table(&(self.pages)(&report.file)?)?;
        // The header's dates by column ("Oct. 15, 2026", or "Mar. 31, 2026
        // shares" in a 20-F), and its share scale ("shares in Millions").
        let mut dates = std::collections::HashMap::new();
        let mut multiplier = 1.0;
        for row in table.rows.iter().take(3) {
            for (c, cell) in row.iter().enumerate() {
                if let Data::String(s) = cell {
                    let s = s.trim_end_matches(" shares").trim();
                    if let Some(d) = crate::sheet_info::parse_date_str(s) {
                        dates.insert(c, d);
                    }
                    if let Some(m) = crate::sheet_info::share_scale(&s.to_lowercase()) {
                        multiplier = m;
                    }
                }
            }
        }
        let mut total: Option<(chrono::NaiveDate, f64)> = None;
        let mut member = "";
        for (row, tag) in table.rows.iter().zip(&table.tags) {
            let tag = tag.as_deref().unwrap_or("");
            if tag.ends_with("Axis") {
                member = label(row).unwrap_or("");
                continue;
            }
            if tag != "dei_EntityCommonStockSharesOutstanding" { continue; }
            // Loyalty shares (Ferrari's, Stellantis's) vote but own nothing.
            if member.to_lowercase().contains("special voting") { continue; }
            for (c, cell) in row.iter().enumerate() {
                let (Data::Float(v), Some(date)) = (cell, dates.get(&c)) else { continue };
                let per_share = if self.ticker == Ticker::BRK && member.contains("Class A") { 1500.0 } else { 1.0 };
                let (d, sum) = total.get_or_insert((*date, 0.0));
                if *d == *date {
                    *sum += v * multiplier * per_share;
                }
            }
        }
        let (date, val) = total.ok_or("no shares outstanding on the cover page")?;
        Ok(vec![Reported { ticker: self.ticker, date, p: crate::Period::PointInTime, item: Item::SharesOutstanding, currency: crate::Currency::USD, val, filing: None, derived: false }])
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
///
/// A section starts at a row labeled "… [Member]" (the xlsx), or one whose
/// element is the axis ("srt_ProductOrServiceAxis", labeled with the member,
/// "Automotive Revenues"). Where the member names a part of an item
/// (`Item::part_of`), only that item's rows report the part.
fn drop_segment_rows(rows: &[&[Data]], tags: &[Option<&str>], items: &mut [Option<(Item, usize)>]) {
    use std::collections::{HashMap, HashSet};
    let mut segment: Vec<Option<usize>> = Vec::with_capacity(rows.len());
    let mut member_items: HashMap<usize, Item> = HashMap::new();
    let mut current = None;
    for (r, row) in rows.iter().enumerate() {
        let axis = tags.get(r).copied().flatten().is_some_and(|t| t.ends_with("Axis"));
        if let Some(l) = label(row).filter(|l| l.ends_with("[Member]") || l.ends_with("[member]") || axis) {
            current = Some(r);
            // "[Member]", or "[member]" (Honda).
            let member = l.strip_suffix("[Member]").or_else(|| l.strip_suffix("[member]")).unwrap_or(l).trim();
            if let Ok(item) = l.parse::<Item>().or_else(|_| member.parse::<Item>()) {
                member_items.insert(r, item);
            }
        }
        segment.push(current);
    }
    for (i, s) in items.iter_mut().zip(&segment) {
        if let (Some((row_item, _)), Some(item)) = (*i, s.and_then(|s| member_items.get(&s))) {
            match item.part_of() {
                None => *i = Some((*item, 0)),
                Some(whole) if whole == row_item => *i = Some((*item, 0)),
                Some(_) => (),
            }
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

/// Elements for money borrowed, on the face of a balance sheet: due within a
/// year, after, or either. Not debt securities the company holds.
const DEBT: &[&str] = &[
    "us-gaap_LoansPayable", "us-gaap_NotesAndLoansPayable", "us-gaap_NotesAndLoansPayableCurrent", "us-gaap_LongTermNotesAndLoans",
    "us-gaap_ConvertibleDebtCurrent", "us-gaap_ConvertibleDebtNoncurrent", "us-gaap_ConvertibleNotesPayable",
    "us-gaap_SeniorLongTermNotes", "us-gaap_UnsecuredLongTermDebt", "us-gaap_SecuredDebtCurrent",
    "us-gaap_OtherLongTermDebtCurrent", "us-gaap_OtherLongTermDebtNoncurrent",
    "us-gaap_LinesOfCreditCurrent", "us-gaap_LongTermLineOfCredit", "us-gaap_LineOfCredit",
    "us-gaap_DebtAndCapitalLeaseObligations", "us-gaap_DebtLongtermAndShorttermCombinedAmount",
    "us-gaap_JuniorSubordinatedNotes",
    "ifrs-full_ShorttermBorrowings", "ifrs-full_LongtermBorrowings", "ifrs-full_CurrentPortionOfLongtermBorrowings",
    "ifrs-full_CurrentBorrowingsAndCurrentPortionOfNoncurrentBorrowings",
    "ifrs-full_NoncurrentPortionOfNoncurrentBondsIssued", "ifrs-full_CurrentPortionOfNoncurrentBondsIssued",
    "us-gaap_LongTermDebtNoncurrent", "us-gaap_LongTermDebtCurrent", "us-gaap_LongTermDebt",
    "us-gaap_LongTermDebtAndCapitalLeaseObligations", "us-gaap_LongTermDebtAndCapitalLeaseObligationsCurrent",
    "us-gaap_DebtCurrent", "us-gaap_ShortTermBorrowings", "us-gaap_OtherShortTermBorrowings",
    "us-gaap_CommercialPaper", "us-gaap_NotesPayable", "us-gaap_NotesPayableCurrent",
    "us-gaap_SecuredDebt", "us-gaap_UnsecuredDebt", "us-gaap_SeniorNotes", "us-gaap_ConvertibleLongTermNotesPayable",
    "us-gaap_FinanceLeaseLiabilityCurrent", "us-gaap_FinanceLeaseLiabilityNoncurrent",
    "us-gaap_LongTermDebtAndCapitalLeaseObligationsIncludingCurrentMaturities",
    "ifrs-full_CurrentBorrowings", "ifrs-full_NoncurrentBorrowings", "ifrs-full_Borrowings",
];

/// A borrowing line: a standard element in `DEBT`, or a company's own whose
/// name says so (Tesla's "LongTermDebtAndFinanceLeasesCurrent", Kroger's
/// "LongTermDebtAndFinanceLease", W. R. Berkley's "SeniorNotesAndOtherDebt",
/// Bloom Energy's "LongTermPortionOfRecourseDebt…"),
/// but not debt it holds or costs of it.
fn is_debt(tag: &str) -> bool {
    if DEBT.contains(&tag) { return true; }
    if !is_extension(tag) { return false; }
    let name = tag.split_once('_').map(|(_, n)| n).unwrap_or(tag);
    ["Debt", "Borrowings", "SeniorNotes"].iter().any(|w| name.contains(w))
        // Not an asset, a reserve in equity, or a liability named for not
        // being debt (Procter & Gamble's "ReserveForEsopDebtRetirement",
        // AES's "DebtServiceReserves…Deposits", Ecolab's "OperatingLease…
        // ExcludedFromLongTermDebt", Xcel's "…OtherThanLongTermDebt…").
        && !["Securities", "Receivable", "Investment", "Issuance", "Interest", "Repayment", "Proceeds", "FairValue",
             "Reserve", "Deposits", "OperatingLease", "Excluded", "OtherThan"].iter().any(|w| name.contains(w))
}

/// Total debt: every borrowing line on the face of the balance sheet, current
/// and non-current, summed per column. Companies split debt across a dozen
/// elements (long-term debt, its current portion, short-term borrowings,
/// commercial paper, senior notes, finance leases), so no one element is it.
/// Where a total that includes current maturities (`LongTermDebt`,
/// `Borrowings`) sits beside the split, only the split counts.
///
/// Some companies state debt only per business, in segment sections: Ford
/// ("Company excluding Ford Credit", "Ford Credit"), GM ("Automotive", "GM
/// Financial"), Berkshire (by business group). Where the face has none, the
/// sections together are the company; a car maker's automotive and lending
/// sections are also reported on their own (AutomotiveDebt,
/// FinancialServicesDebt).
fn total_debt(ticker: Ticker, rows: &[&[Data]], tags: &[Option<&str>], info: &crate::sheet_info::SheetInfo) -> Vec<Reported> {
    use std::collections::HashMap;
    // Each row's section: None on the face, else the member's label.
    let mut section: Option<&str> = None;
    let mut lines: Vec<(Option<&str>, &str, &[Data])> = Vec::new();
    for (row, tag) in rows.iter().zip(tags) {
        let l = label(row);
        if tag.is_some_and(|t| t.ends_with("Axis")) || l.is_some_and(|l| l.ends_with("[Member]") || l.ends_with("[member]")) {
            section = l;
            continue;
        }
        if let Some(t) = tag.filter(|t| is_debt(t)) {
            lines.push((section, t, row));
        }
    }
    let on_face = lines.iter().any(|(s, _, _)| s.is_none());
    // Sums by (section, column) over the lines that count.
    let mut sums: HashMap<(Option<&str>, usize), f64> = HashMap::new();
    let sections: Vec<Option<&str>> = {
        let mut v: Vec<_> = lines.iter().map(|(s, _, _)| *s).collect();
        v.dedup();
        v
    };
    for s in &sections {
        if on_face && s.is_some() { continue; }
        let here: Vec<_> = lines.iter().filter(|(ls, _, _)| ls == s).collect();
        let has = |t: &str| here.iter().any(|(_, tag, _)| *tag == t);
        let split = has("us-gaap_LongTermDebtCurrent") || has("us-gaap_LongTermDebtNoncurrent")
            || has("ifrs-full_CurrentBorrowings") || has("ifrs-full_NoncurrentBorrowings")
            || has("ifrs-full_LongtermBorrowings");
        for (_, tag, row) in &here {
            if split && (*tag == "us-gaap_LongTermDebt" || *tag == "ifrs-full_Borrowings") { continue; }
            for &col in info.dates_and_periods.keys() {
                if let Some(Data::Float(v)) = row.get(col) {
                    *sums.entry((*s, col)).or_default() += v;
                }
            }
        }
    }
    let make = |item: Item, col: usize, v: f64| {
        let (date, p) = info.dates_and_periods[&col];
        Reported { ticker, date, p, item, currency: info.currencies.get(&col).copied().unwrap_or(crate::Currency::USD), val: v * info.multiplier, filing: None, derived: false }
    };
    let mut totals: HashMap<usize, f64> = HashMap::new();
    let mut out = Vec::new();
    for ((s, col), v) in &sums {
        *totals.entry(*col).or_default() += v;
        // A car maker's automotive or lending section, by the item its
        // member names (as for revenue).
        let part = s.map(|l| l.strip_suffix("[Member]").or_else(|| l.strip_suffix("[member]")).unwrap_or(l).trim())
            .and_then(|m| m.parse::<Item>().ok());
        match part {
            Some(Item::AutomotiveRevenue) => out.push(make(Item::AutomotiveDebt, *col, *v)),
            Some(Item::FinancialServicesRevenue) => out.push(make(Item::FinancialServicesDebt, *col, *v)),
            _ => (),
        }
    }
    out.extend(totals.into_iter().map(|(col, v)| make(Item::TotalDebt, col, v)));
    out.sort_by_key(|r| (r.date, r.item));
    out
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
///
/// One element needs its label too: contract revenue including taxes
/// collected is some companies' "Total revenue" but a bank's "Fee and
/// commission income", a part of its income. It reports revenue only on a
/// row that says revenue or sales.
///
/// Visible to the crate, so a company's module can test how its own rows
/// read (gs).
pub(crate) fn row_item(ticker: Ticker, tag: Option<&str>, label: Option<&str>) -> Option<(Item, usize)> {
    let by_label = || label.and_then(|l| l.parse::<Item>().ok()).map(|i| (i, 0));
    match tag {
        None => by_label(),
        Some("us-gaap_RevenueFromContractWithCustomerIncludingAssessedTax")
            if !label.is_some_and(|l| { let l = l.to_lowercase(); l.contains("revenue") || l.contains("sales") }) => None,
        // The company's own elements first (its module), then the shared.
        Some(tag) => match crate::company::item_ranked(ticker, tag) {
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
    fn contract_revenue_including_taxes_needs_a_revenue_label() {
        let tag = Some("us-gaap_RevenueFromContractWithCustomerIncludingAssessedTax");
        assert_eq!(row_item(Ticker::AAPL, tag, Some("Total revenue")).map(|r| r.0), Some(Item::Revenue));
        assert_eq!(row_item(Ticker::AAPL, tag, Some("Net sales")).map(|r| r.0), Some(Item::Revenue));
        // A bank's fee income: part of its income, not its revenue.
        assert_eq!(row_item(Ticker::AAPL, tag, Some("Fee and commission income")), None);
        // Other elements don't need it.
        assert_eq!(row_item(Ticker::GS, Some("us-gaap_RevenuesNetOfInterestExpense"), Some("Total net revenues")).map(|r| r.0),
                   Some(Item::Revenue));
    }

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
