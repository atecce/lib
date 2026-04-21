uniffi::setup_scaffolding!();

use name::Name;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, uniffi::Record)]
pub struct Book {
    pub name: Name,
    // TODO(atec)
    // pub text: str,
}
