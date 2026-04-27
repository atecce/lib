uniffi::setup_scaffolding!();

use book::Book;
use book::UniffiBook;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Source {
    pub book: Book,
    pub chapter: u8,
    // TODO(atec): perhaps some enforcement of verses[0] <= verses[1]
    //             hack to avoid cast to Data. should use .udl file with sequence<u8>
    pub verses: [u16; 2],
}

impl Source {
    pub fn new(src: Source) -> UniffiSource {
        UniffiSource {
            book: Book::new(src.book),
            chapter: src.chapter,
            verses: src.verses.to_vec(),
        }
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct UniffiSource {
    pub book: UniffiBook,
    pub chapter: u8,
    // TODO(atec): hack to avoid cast to Data. should use .udl file with sequence<u8>
    pub verses: Vec<u16>,
}
