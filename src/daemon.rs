use crate::name::Name;
use crate::src::Source;
use crate::deed::Deed;

#[derive(Debug)]
pub struct Daemon<'a> {
    pub names: &'a [Name],
    pub words: &'a [Source],
    pub deeds: &'a [Deed<'a>],

    pub father: Option<&'a Daemon<'a>>,
    pub mother: Option<&'a Daemon<'a>>,
    pub teacher: Option<&'a Daemon<'a>>,
}
