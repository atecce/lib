//! One filing's statements, in whatever format its source keeps them: the
//! interface the batch job reads every source through.
//!
//! What a filing *is* (its id, form, period, the day it became public) is
//! plain data, `Filed`, the same for every source. What differs is the
//! format of its contents, so that is what implements `Statements`:
//!
//! - `EdgarFiling`: EDGAR's rendered statement pages (FilingSummary.xml and
//!   the R pages it lists), read as the reader asks for them;
//! - `EdinetReport`: an EDINET report's CSV files (UTF-16, one fact a line);
//! - `MopsReport`: a MOPS report's page (Big5 HTML).
//!
//! Each reads into `Reported`, every value carrying its filing. None opens a
//! file: the caller (store::sources) hands over the contents.

use crate::item::{Filed, Reported};
use crate::Ticker;

/// What reading a filing gave: its values, and what was left out and why,
/// for the caller to report.
#[derive(Debug, Default)]
pub struct Read {
    pub items: Vec<Reported>,
    pub notes: Vec<String>,
}

/// A filing whose statements can be read.
pub trait Statements {
    /// The filing.
    fn filed(&self) -> &Filed;
    /// Every value it states, each carrying `filed`, and what was skipped.
    fn read(&mut self) -> Read;
}

/// Stamps each item with the filing, where the reader didn't.
fn stamp(items: &mut [Reported], filed: &Filed) {
    for it in items {
        if it.filing.is_none() { it.filing = Some(filed.clone()) }
    }
}

/// An EDGAR filing: its index and the pages it lists.
pub struct EdgarFiling {
    filed: Filed,
    reader: crate::filing::Reader,
}

impl EdgarFiling {
    /// `summary`: FilingSummary.xml's text; `pages`: a page's text by file
    /// name, as the reader asks for it.
    pub fn new(filed: Filed, ticker: Ticker, summary: &str, pages: crate::filing::Pages) -> EdgarFiling {
        let name = format!("{} {} {} ({})", ticker, filed.form, filed.period_end.map(|d| d.to_string()).unwrap_or_default(), filed.accession);
        EdgarFiling { reader: crate::filing::reader(name, summary, ticker, pages), filed }
    }
}

impl Statements for EdgarFiling {
    fn filed(&self) -> &Filed { &self.filed }

    fn read(&mut self) -> Read {
        use crate::Reader;
        let mut read = Read::default();
        let parts: [(&str, Result<Vec<Reported>, Box<dyn std::error::Error>>); 3] = [
            ("balance sheet", self.reader.process_balance_sheet()),
            ("income statement", self.reader.process_income_statement()),
            ("cash flow statement", self.reader.process_cash_flow_statement()),
        ];
        for (what, got) in parts {
            match got {
                Ok(mut items) => read.items.append(&mut items),
                Err(e) => read.notes.push(format!("no {}: {}", what, e)),
            }
        }
        // Older filings' cover pages may not be fetched: nothing to note.
        if let Ok(mut items) = self.reader.process_cover() { read.items.append(&mut items) }
        read.notes.extend(self.reader.warnings());
        stamp(&mut read.items, &self.filed);
        read
    }
}

/// An EDINET report: its CSV files, each (name, bytes).
pub struct EdinetReport {
    filed: Filed,
    ticker: Ticker,
    kind: crate::edinet::Kind,
    files: Vec<(String, Vec<u8>)>,
}

impl EdinetReport {
    pub fn new(filed: Filed, ticker: Ticker, kind: crate::edinet::Kind, files: Vec<(String, Vec<u8>)>) -> EdinetReport {
        EdinetReport { filed, ticker, kind, files }
    }
}

impl Statements for EdinetReport {
    fn filed(&self) -> &Filed { &self.filed }

    fn read(&mut self) -> Read {
        match crate::edinet::read_csv(&self.filed, self.kind, self.ticker, &self.files) {
            Ok(items) => Read { items, notes: Vec::new() },
            Err(e) => Read { items: Vec::new(), notes: vec![format!("{}: {}", self.filed.accession, e)] },
        }
    }
}

/// A MOPS report: its page's bytes.
pub struct MopsReport {
    filed: Filed,
    ticker: Ticker,
    bytes: Vec<u8>,
}

impl MopsReport {
    pub fn new(filed: Filed, ticker: Ticker, bytes: Vec<u8>) -> MopsReport {
        MopsReport { filed, ticker, bytes }
    }
}

impl Statements for MopsReport {
    fn filed(&self) -> &Filed { &self.filed }

    fn read(&mut self) -> Read {
        Read { items: crate::mops::read_bytes(&self.filed, self.ticker, &self.bytes), notes: Vec::new() }
    }
}
