uniffi::setup_scaffolding!();

use source::Source;

#[derive(Clone, Copy, Debug)]
pub struct Deed<'a, C> {
    pub desc: &'a str,
    pub srcs: &'a [C],
}
