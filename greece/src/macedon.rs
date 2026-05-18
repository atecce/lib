use crate::ΑΡΙΣΤΟΤΕΛΗΣ;
use crate::Source;

use std::sync::Arc;

use daemon::ArcDaemon;
use daemon::BoxDaemon;
use daemon::Daemon;

const ΦΙΛΙΠΠΟΣ: &Daemon<Source> = &Daemon {
    names: &[name::Name::Φίλιππος],
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

    father: Some(ΦΙΛΙΠΠΟΣ),
    mother: None,
    teacher: Some(ΑΡΙΣΤΟΤΕΛΗΣ),

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
