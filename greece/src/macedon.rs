use crate::ARISTOTLE;
use crate::Source;

use std::sync::Arc;

use daemon::ArcDaemon;
use daemon::BoxDaemon;
use daemon::Daemon;

use name::Name::Philip;

const PHILIP: &Daemon<Source> = &Daemon {
    names: &[Philip],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: None,
};

pub const ΑΛΕΞΑΝΔΡΟΣ: &Daemon<Source> = &Daemon {
    names: &[name::Name::Ἀλέξανδρος],
    words: &[],
    deeds: &[],

    father: Some(PHILIP),
    mother: None,
    teacher: Some(ARISTOTLE),

    predecessor: None,
};

#[uniffi::export]
pub fn arc_defender_of_men() -> ArcDaemon {
    Arc::unwrap_or_clone(ΑΛΕΞΑΝΔΡΟΣ.new_arc().unwrap())
}

#[uniffi::export]
pub fn box_defender_of_men() -> BoxDaemon {
    *ΑΛΕΞΑΝΔΡΟΣ.new_box().unwrap()
}
