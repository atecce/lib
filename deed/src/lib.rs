uniffi::setup_scaffolding!();

use source::Source;

#[derive(Clone, Copy, Debug)]
pub struct Deed<'a> {
    pub desc: &'a str,
    pub srcs: &'a [Source],
}

impl Deed<'_> {
    pub fn new(deed: Deed) -> SwiftDeed {
        SwiftDeed {
            desc: deed.desc.into(),
            srcs: deed.srcs.to_vec(),
        }
    }
}

#[derive(Debug, uniffi::Object)]
pub struct SwiftDeed {
    pub desc: Box<str>,
    pub srcs: Vec<Source>,
}
