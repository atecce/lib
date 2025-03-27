#[derive(Debug)]
pub struct Daemon<'a> {
    pub names: &'a [&'a str],
    pub words: &'a [&'a str],
    pub deeds: &'a [&'a str],

    pub father: Option<&'a Daemon<'a>>,
    pub mother: Option<&'a Daemon<'a>>,
    pub teacher: Option<&'a Daemon<'a>>,
}
