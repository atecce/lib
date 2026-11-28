//! One filing's statements, in whatever format its source keeps them: the
//! interface the batch job reads every source through.
//!
//! What a filing *is* (its id, form, period, the day it became public) is
//! plain data, `Filed`, the same for every source. What differs is the
//! format of its contents, so that is what implements `Statements`:
//!
//! - `edgar::filing::EdgarFiling` (in the edgar crate, which reads its own
//!   format): EDGAR's rendered statement pages, read as asked;
//! - `edinet::report::EdinetReport` (in the edinet crate): a report's CSV
//!   files (UTF-16, one fact a line);
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
