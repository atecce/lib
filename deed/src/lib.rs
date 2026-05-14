uniffi::setup_scaffolding!();

#[derive(Clone, Copy, Debug)]
pub struct Deed<'a, C> {
    pub desc: &'a str,
    pub srcs: &'a [C],
}
