uniffi::setup_scaffolding!();

pub mod kjv;
pub mod genealogy;

use name::Name;

use std::fmt;

#[derive(Clone, Debug)]
pub struct Source {
    pub book: Name,
    pub chapter: usize,
    // TODO(atec): perhaps some enforcement of start <= end
    pub start: usize,
    pub end: Option<usize>,
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

impl citation::Citation for Source {
    fn cite(&self) -> String {
        self.to_string()
    }
}
