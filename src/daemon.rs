#[derive(Debug)]
pub struct Daemon<'a> {
//    names: &[&str],
//    words: &[&str],
//    deeds: &[&str],

    pub father: Option<&'a Daemon<'a>>,
    pub mother: Option<&'a Daemon<'a>>,
}
