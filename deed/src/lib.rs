uniffi::setup_scaffolding!();

use source::Source;
use source::UniffiSource;

#[derive(Clone, Copy, Debug)]
pub struct Deed<'a> {
    pub desc: &'a str,
    pub srcs: &'a [Source],
}

impl Deed<'_> {
    pub fn new(deed: Deed) -> UniffiDeed {
        UniffiDeed {
            desc: deed.desc.into(),
            srcs: deed
                .srcs
                .into_iter()
                .map(|src| Source::new(src.clone()))
                .collect(),
        }
    }
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct UniffiDeed {
    pub desc: String,
    pub srcs: Vec<UniffiSource>,
}
