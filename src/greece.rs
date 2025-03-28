use crate::name::Name::Cronus;
use crate::name::Name::Cronos;
use crate::name::Name::Kronos;
use crate::name::Name::Zeus;
use crate::name::Name::Hera;
use crate::name::Name::Hermes;
use crate::name::Name::Apollo;
use crate::name::Name::Artemis;
use crate::name::Name::Achilles;
use crate::name::Name::Socrates;
use crate::name::Name::Plato;
use crate::name::Name::Aristotle;
use crate::daemon::Daemon;

pub const CRONUS: &Daemon = &Daemon {
    names: &[
        Cronus,
        Cronos,
        Kronos,
    ],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,
};

pub const ZEUS: &Daemon = &Daemon {
    names: &[
        Zeus,
    ],
    words: &[],
    deeds: &[],

    father: Some(CRONUS),
    mother: None,
    teacher: None,
};

pub const HERA: &Daemon = &Daemon {
    names: &[
        Hera,
    ],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,
};

pub const HERMES: &Daemon = &Daemon {
    names: &[
        Hermes,
    ],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,
};

pub const APOLLO: &Daemon = &Daemon {
    names: &[
        Apollo,
    ],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,
};

pub const ARTEMIS: &Daemon = &Daemon {
    names: &[
        Artemis,
    ],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,
};

pub const ACHILLES: &Daemon = &Daemon {
    names: &[
        Achilles,
    ],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,
};

pub const SOCRATES: &Daemon = &Daemon {
    names: &[
        Socrates,
    ],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,
};

pub const PLATO: &Daemon = &Daemon {
    names: &[
        Plato,
    ],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: Some(SOCRATES),
};

pub const ARISTOTLE: &Daemon = &Daemon {
    names: &[
        Aristotle,
    ],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: Some(PLATO),
};
