use crate::ARISTOTLE;

use std::sync::Arc;

use daemon::ArcDaemon;
use daemon::BoxDaemon;
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

#[uniffi::export]
pub fn arc_defender_of_men() -> ArcDaemon {
    Arc::unwrap_or_clone(ALEXANDER.new_arc().unwrap())
}

#[uniffi::export]
pub fn box_defender_of_men() -> BoxDaemon {
    *ALEXANDER.new_box().unwrap()
}
