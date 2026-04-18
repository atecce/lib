pub mod macedon;

use daemon::Daemon;
use name::Name::Achilles;
use name::Name::Apollo;
use name::Name::Aristotle;
use name::Name::Artemis;
use name::Name::Cronos;
use name::Name::Cronus;
use name::Name::Hera;
use name::Name::Hermes;
use name::Name::Kronos;
use name::Name::Leto;
use name::Name::Plato;
use name::Name::Socrates;
use name::Name::Zeus;

pub const CRONUS: &Daemon = &Daemon {
    names: &[Cronus, Cronos, Kronos],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: None,
};

pub const ZEUS: &Daemon = &Daemon {
    names: &[Zeus],
    words: &[],
    deeds: &[],

    father: Some(CRONUS),
    mother: None,
    teacher: None,

    predecessor: None,
};

pub const HERA: &Daemon = &Daemon {
    names: &[Hera],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: None,
};

const LETO: &Daemon = &Daemon {
    names: &[Leto],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: None,
};

pub const HERMES: &Daemon = &Daemon {
    names: &[Hermes],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: None,
};

pub const APOLLO: &Daemon = &Daemon {
    names: &[Apollo],
    words: &[],
    deeds: &[],

    father: Some(ZEUS),
    mother: Some(LETO),
    teacher: None,

    predecessor: None,
};

pub const ARTEMIS: &Daemon = &Daemon {
    names: &[Artemis],
    words: &[],
    deeds: &[],

    father: Some(ZEUS),
    mother: Some(LETO),
    teacher: None,

    predecessor: None,
};

pub const ACHILLES: &Daemon = &Daemon {
    names: &[Achilles],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: None,
};

pub const SOCRATES: &Daemon = &Daemon {
    names: &[Socrates],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: None,
};

pub const PLATO: &Daemon = &Daemon {
    names: &[Plato],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: Some(SOCRATES),

    predecessor: None,
};

pub const ARISTOTLE: &Daemon = &Daemon {
    names: &[Aristotle],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: Some(PLATO),

    predecessor: None,
};
