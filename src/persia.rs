use crate::name::Name::Cyrus;
use crate::daemon::Daemon;

pub const CYRUS: &Daemon = &Daemon {
    names: &[
        Cyrus,
    ],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,
};
