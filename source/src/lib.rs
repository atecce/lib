uniffi::setup_scaffolding!();

use book::Book;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, uniffi::Object)]
pub struct Source {
    pub book: Book,
    pub chapter: u8,
    // TODO(atec): perhaps some enforcement of verses[0] <= verses[1]
    pub verses: [u8; 2],
}
