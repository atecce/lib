use std::error::Error;
use std::io::Cursor;

use crate::Ticker;
use crate::Reader as R;
use crate::item::{Item, Reported};
use crate::sheet_info::{new_sheet_info, SheetInfo, SheetType};
use crate::sheet_info::SheetType::{BalanceSheet, IncomeStatement, CashFlowStatement};

use calamine::{open_workbook_auto_from_rs, Data, DataType, Reader as CalamineReader, Sheets};

pub struct Reader {
    workbook: Sheets<Cursor<Vec<u8>>>,
    ticker: Ticker,
}

/// A workbook from its bytes (an EDGAR Financial_Report.xlsx), as the
/// caller read them: no files.
pub fn reader(bytes: Vec<u8>, ticker: Ticker) -> Result<Reader, Box<dyn Error>> {
    Ok(Reader { workbook: open_workbook_auto_from_rs(Cursor::new(bytes))?, ticker })
}

/// A workbook from disk: `reader` with its file. The `files` feature.
#[cfg(feature = "files")]
pub fn new_reader(path: &std::path::Path, ticker: Ticker) -> Result<Reader, Box<dyn Error>> {
    reader(std::fs::read(path)?, ticker)
}

/// Non-empty rows of a statement sheet.
fn statement_rows(range: &calamine::Range<Data>) -> Vec<&[Data]> {
    range.rows()
        .filter(|row| !row.iter().all(|c| c.is_empty()))
        .collect()
}

impl R for Reader {
    fn process_balance_sheet(&mut self) -> Result<Vec<Reported>, Box<dyn Error>> {
        let sheet_name = self.find_sheet(&["BALANCE_SHEET", "Consolidated Balance"])
            .ok_or("Balance sheet not found")?;

        let range = self.workbook.worksheet_range(&sheet_name)?;

        let rows = statement_rows(&range);

        Self::reported_items(self.ticker, &rows, new_sheet_info(&rows, BalanceSheet)?)
    }

    fn process_income_statement(&mut self) -> Result<Vec<Reported>, Box<dyn Error>> {
        let sheet_name = self.find_sheet(&["INCOME_STATEMENT", "Consolidated Statements of Oper", "Consolidated Statements of Inco", "Consolidated Statem"])
            .ok_or("Income statement not found")?;

        let range = self.workbook.worksheet_range(&sheet_name)?;

        let rows = statement_rows(&range);

        Self::reported_items(self.ticker, &rows, new_sheet_info(&rows, IncomeStatement)?)
    }
}

impl Reader {
    pub fn process_cash_flow_statement(&mut self) -> Result<Vec<Reported>, Box<dyn Error>> {
        let range = self.workbook.worksheet_range("CASH_FLOW")?;
        let rows = statement_rows(&range);

        Self::reported_items(self.ticker, &rows, new_sheet_info(&rows, CashFlowStatement)?)
    }
    pub fn items(&mut self, sheet_type: SheetType) -> Result<Vec<Item>, Box<dyn Error>> {
        match sheet_type {
            BalanceSheet => {
                let range = self.workbook.worksheet_range("BALANCE_SHEET")?;
                let rows: Vec<&[Data]> = range.rows()
                    .filter(|row| !row.iter().all(|c| c.is_empty()))
                    .collect();

                let sheet_info = new_sheet_info(&rows, sheet_type)?;

                let mut items = Vec::new();
                for row in rows {
                    if let Some(label) = row.get(sheet_info.labels).and_then(|c| c.get_string()) {
                        if let Ok(item) = label.parse::<Item>() {
                            items.push(item);
                        }
                    }
                }

                Ok(items)
            },
            _ => Err("sheet type not supported".into()),
        }
    }

    fn find_sheet(&self, matches: &[&str]) -> Option<String> {
        let names = self.workbook.sheet_names();
        for m in matches {
            for name in &names {
                // Older EDGAR workbooks use underscores for spaces ("Condensed_Consolidated_Balance").
                if name.replace('_', " ").to_lowercase().contains(&m.replace('_', " ").to_lowercase()) {
                    return Some(name.clone());
                }
            }
        }
        None
    }

    /// Turn statement rows (label column plus one column per date) into reported items.
    /// Shared with `filing::Reader`, whose HTML tables are converted to the same rows.
    pub(crate) fn reported_items(ticker: Ticker, rows: &[&[Data]], sheet_info: SheetInfo) -> Result<Vec<Reported>, Box<dyn Error>> {
        let labels = sheet_info.labels;
        Self::reported_items_with(ticker, rows, sheet_info, |_, row| {
            row.get(labels).and_then(|c| c.get_string()).and_then(|label| label.parse::<Item>().ok()).map(|i| (i, 0))
        })
    }

    /// Like `reported_items`, but `item_of(row_index, row)` decides which item a
    /// row reports (e.g. by its XBRL tag rather than its label), with a rank:
    /// when two rows report the same item, the lower rank wins, and among
    /// equal ranks the first row.
    pub(crate) fn reported_items_with(ticker: Ticker, rows: &[&[Data]], sheet_info: SheetInfo, item_of: impl Fn(usize, &[Data]) -> Option<(Item, usize)>) -> Result<Vec<Reported>, Box<dyn Error>> {

        let mut reported_items = Vec::new();

        // EDGAR's Financial_Report.xlsx appends per-segment breakdowns (rows under
        // headers like "Automotive Sales [Member]") after the statement proper. Some
        // line items only appear there (TSLA's "Operating lease vehicles, net"), while
        // others repeat labels from the face of the statement ("Revenues"). Keep the
        // first value seen for each (item, date, period) so the face statement wins.
        // A zero filled in from an empty cell (below) is only provisional: a later
        // row with a real number for the same key replaces it.
        let mut seen: std::collections::HashMap<_, (usize, bool, usize)> = std::collections::HashMap::new();

        // The statement's own columns, where most line items have a number.
        // EDGAR's pages add a column for each other date any fact on the page
        // has (Target's Q1 cash flows: Feb. 1 and Aug. 3 beside the quarter,
        // for a row or two); an empty cell there is not reported, not zero.
        let numbers_in = |col: usize| rows.iter().enumerate()
            .filter(|(r, row)| item_of(*r, row).is_some())
            .filter(|(_, row)| matches!(row.get(col), Some(Data::Float(_)) | Some(Data::Int(_))))
            .count();
        let counts: std::collections::HashMap<usize, usize> = sheet_info.dates_and_periods.keys()
            .map(|&col| (col, numbers_in(col))).collect();
        let densest = counts.values().copied().max().unwrap_or(0);
        let main_column = |col: usize| counts.get(&col).is_some_and(|&n| n * 2 >= densest);

        for (r, row) in rows.iter().enumerate() {
            {
                if let Some((mut item, rank)) = item_of(r, row) {
                    // A line item with a number in some period but an empty cell in another
                    // is zero there (EDGAR leaves the cell empty where the filing shows "—").
                    // Rows with no numbers at all are section headers and are skipped.
                    let has_value = sheet_info.dates_and_periods.keys()
                        .any(|&col| matches!(row.get(col), Some(Data::Float(_)) | Some(Data::Int(_))));

                    // Left to right, so which column wins a tie is the same every run.
                    let mut columns: Vec<_> = sheet_info.dates_and_periods.iter().collect();
                    columns.sort_by_key(|(col, _)| **col);
                    for (&col, (date, period)) in columns {

                        let (val, filled) = match row.get(col) {
                            Some(Data::Float(f)) => (*f, false),
                            Some(Data::Int(i)) => (*i as f64, false),
                            // Only money: an EPS or share count left empty is unknown, not zero.
                            Some(Data::Empty) | None if has_value && item.unit() == crate::item::Unit::Money
                                && main_column(col) => (0.0, true),
                            _ => (f64::NAN, false),
                        };

                        match sheet_info.sheet_type {
                            crate::sheet_info::SheetType::CashFlowStatement => {
                                match item {
                                    crate::Item::Inventories => {
                                        item = crate::Item::ChangeInInventories;
                                    },
                                    crate::Item::AccountsPayable => {
                                        item = crate::Item::ChangeInAccountsPayable;
                                    },
                                    crate::Item::AccruedAndOtherCurrentLiabilities => {
                                        item = crate::Item::ChangeInAccruedAndOtherCurrentLiabilities;
                                    },
                                    crate::Item::OtherLongTermLiabilities => {
                                        item = crate::Item::ChangeInOtherLongTermLiabilities;
                                    },
                                    _ => (),
                                }
                            },
                            _ => (),
                        }

                        if val.is_nan() { continue; }
                        let val = val * match item.unit() {
                            crate::item::Unit::Money => sheet_info.multiplier,
                            crate::item::Unit::Shares => sheet_info.share_multiplier,
                            crate::item::Unit::PerShare => 1.0,
                        };
                        let currency = sheet_info.currencies.get(&col).copied().unwrap_or(crate::Currency::USD);
                        match seen.get_mut(&(item, *date, *period, currency)) {
                            None => {
                                seen.insert((item, *date, *period, currency), (reported_items.len(), filled, rank));
                                reported_items.push(Reported {
                                    ticker,
                                    date: *date,
                                    p: *period,
                                    item,
                                    currency,
                                    val,
                                    filing: None,
                                    derived: false,
                                });
                            },
                            Some((idx, was_filled, was_rank)) if !filled && (*was_filled || rank < *was_rank) => {
                                reported_items[*idx].val = val;
                                *was_filled = false;
                                *was_rank = rank;
                            },
                            Some(_) => (),
                        }
                    }
                }
            }
        }
        Ok(reported_items)
    }
}
