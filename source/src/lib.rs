uniffi::setup_scaffolding!();

use std::fmt;

use name::Name;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Source {
    pub book: Name,
    pub chapter: u8,
    // TODO(atec): perhaps some enforcement of verses[0] <= verses[1]
    //             hack to avoid cast to Data. should use .udl file with sequence<u8>
    pub verses: [u16; 2],
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.verses[0] == self.verses[1] {
            write!(f, "{} {}:{}", self.book, self.chapter, self.verses[0])
        } else {
            write!(
                f,
                "{} {}:{}-{}",
                self.book, self.chapter, self.verses[0], self.verses[1]
            )
        }
    }
}

impl citation::Citation for Source {
    fn cite(&self) -> String {
        self.to_string()
    }
}
