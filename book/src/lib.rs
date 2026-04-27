uniffi::setup_scaffolding!();

use name::Name;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize, uniffi::Record)]
pub struct Book {
    pub name: Name,
    // TODO(atec)
    // pub text: str,
}

impl Book {
    pub fn new(book: Book) -> UniffiBook {
        UniffiBook {
            name: book.name.to_string(),
        }
    }
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct UniffiBook {
    pub name: String
}
