use crate::daemon::Daemon;
use crate::greece::ARISTOTLE;

pub const ALEXANDER: &Daemon = &Daemon {
    names: &[
        "Alexander",
    ],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: Some(ARISTOTLE),
};
