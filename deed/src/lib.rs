use source::Source;

#[derive(Debug)]
pub struct Deed<'a> {
    pub desc: &'a str,
    pub srcs: &'a [Source],
}
