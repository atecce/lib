use crate::book::Book;

#[derive(Debug)]
pub struct Source {
    pub book: Book,
    pub chapter: u8,
    // TODO(atec): perhaps some enforcement of verses[0] <= verses[1]
    pub verses: [u8;2],
}
