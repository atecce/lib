use crate::ARISTOTLE;

use daemon::Daemon;
use name::Name::Alexander;
use name::Name::Philip;

const PHILIP: &Daemon = &Daemon {
    names: &[Philip],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: None,
};

pub const ALEXANDER: &Daemon = &Daemon {
    names: &[name::Name::Ἀλέξανδρος, Alexander],
    words: &[],
    deeds: &[],

    father: Some(PHILIP),
    mother: None,
    teacher: Some(ARISTOTLE),

    predecessor: None,
};
