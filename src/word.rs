use crate::book::Book;

#[derive(Debug)]
pub struct Word {
    pub book: Book,
    pub chapter: u8,
    pub verse: u8,
}
