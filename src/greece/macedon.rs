use crate::name::Name::Philip;
use crate::name::Name::Alexander;
use crate::daemon::Daemon;
use crate::greece::main::ARISTOTLE;

const PHILIP: &Daemon = &Daemon {
    names: &[
        Philip,
    ],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: None,
};

pub const ALEXANDER: &Daemon = &Daemon {
    names: &[
        Alexander,
    ],
    words: &[],
    deeds: &[],

    father: Some(PHILIP),
    mother: None,
    teacher: Some(ARISTOTLE),

    predecessor: None,
};
