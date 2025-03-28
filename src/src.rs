use crate::book::Book;

#[derive(Debug)]
pub struct Source {
    pub book: Book,
    pub chapter: u8,
    pub verse: u8,
}
