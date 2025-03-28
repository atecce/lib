use crate::name::Name;
use crate::word::Word;

#[derive(Debug)]
pub struct Daemon<'a> {
    pub names: &'a [Name],
    pub words: &'a [Word],
    pub deeds: &'a [&'a str],

    pub father: Option<&'a Daemon<'a>>,
    pub mother: Option<&'a Daemon<'a>>,
    pub teacher: Option<&'a Daemon<'a>>,
}
