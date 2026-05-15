uniffi::setup_scaffolding!();

pub mod kjv;
pub mod genealogy;

use name::Name;

use std::fmt;
use std::num;

use serde::Serialize;

#[derive(Clone, Debug, Serialize, uniffi::Record)]
pub struct Source {
    pub book: Name,
    pub chapter: i64,
    // TODO(atec): perhaps some enforcement of start <= end
    pub start: i64,
    pub end: Option<i64>,
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(end) = self.end {
            write!(
                f,
                "{} {}:{}-{}",
                self.book, self.chapter, self.start, end
            )
        } else {
            write!(f, "{} {}:{}", self.book, self.chapter, self.start)
        }
    }
}

impl std::str::FromStr for Source {
    type Err = SourceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        let last_space_idx = s.rfind(' ').ok_or(SourceError::FailedToSplitOnSpace)?;

        let book_str = &s[..last_space_idx];
        let chapter_and_verse_str = &s[last_space_idx + 1..];

        let book = book_str.parse::<Name>().map_err(|err| SourceError::FailedToParseName(err))?;

        let (chapter_str, verses_str) = chapter_and_verse_str.split_once(':')
            .ok_or(SourceError::FailedToSplitOnColon)?;

        let chapter = chapter_str.parse::<i64>()?;

        let start: i64;
        let end: Option<i64>;

        if let Some((start_raw, end_raw)) = verses_str.split_once('-') {
            start = start_raw.parse::<i64>()?;
            end = Some(end_raw.parse::<i64>()?);
        } else {
            start = verses_str.parse::<i64>()?;
            end = None;
        }

        Ok(Source { book, chapter, start, end })
    }
}

#[derive(Debug, PartialEq, Eq, uniffi::Error)]
pub enum SourceError {
    FailedToSplitOnSpace,
    FailedToParseName(name::NameError),
    FailedToSplitOnColon,
    FailedToParseNumber,
}

impl fmt::Display for SourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<num::ParseIntError> for SourceError {
    // TODO(atec): deal with particular cases
    fn from(_: num::ParseIntError) -> Self {
        SourceError::FailedToParseNumber
    }
}

#[uniffi::export]
pub fn parse_source(string: String) -> Result<Source, SourceError> {
    string.parse::<Source>()
}

#[uniffi::export]
pub fn fmt_source(src: Source) -> String {
    format!("{}", src)
}
