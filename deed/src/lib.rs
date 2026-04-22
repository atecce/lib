uniffi::setup_scaffolding!();

use source::Source;

#[derive(Clone, Copy, Debug)]
pub struct Deed<'a> {
    pub desc: &'a str,
    pub srcs: &'a [Source],
}

impl Deed<'_> {
    pub fn new(deed: Deed) -> BoxDeed {
        BoxDeed {
            desc: deed.desc.into(),
            srcs: deed.srcs.to_vec(),
        }
    }
}

#[derive(Clone, Debug, uniffi::Object)]
pub struct BoxDeed {
    pub desc: Box<str>,
    pub srcs: Vec<Source>,
}
