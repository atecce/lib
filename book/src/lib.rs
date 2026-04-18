use name::Name;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Book {
    pub name: Name,
    // TODO(atec)
    // pub text: str,
}
