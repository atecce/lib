uniffi::setup_scaffolding!();

pub mod macedon;

use daemon::Daemon;
use name::Name::Achilles;
use name::Name::Apollo;
use name::Name::Aristotle;
use name::Name::Artemis;
use name::Name::Hera;
use name::Name::Hermes;
use name::Name::Leto;
use name::Name::Plato;
use name::Name::Socrates;

#[derive(Debug)]
pub struct Source;
impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub const ΚΡΌΝΟΣ: &Daemon<Source> = &Daemon {
    names: &[name::Name::Κρόνος],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: None,
};

pub const ΖΕΥΣ: &Daemon<Source> = &Daemon {
    names: &[name::Name::Ζεύς],
    words: &[],
    deeds: &[],

    father: Some(ΚΡΌΝΟΣ),
    mother: None,
    teacher: None,

    predecessor: None,
};

pub const HERA: &Daemon<Source> = &Daemon {
    names: &[Hera],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: None,
};

const LETO: &Daemon<Source> = &Daemon {
    names: &[name::Name::Λατώ, Leto],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: None,
};

pub const HERMES: &Daemon<Source> = &Daemon {
    names: &[name::Name::Ἑρμῆς, Hermes],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: None,
};

pub const APOLLO: &Daemon<Source> = &Daemon {
    names: &[name::Name::Ἀπόλλων, Apollo],
    words: &[],
    deeds: &[],

    father: Some(ΖΕΥΣ),
    mother: Some(LETO),
    teacher: None,

    predecessor: None,
};

pub const ARTEMIS: &Daemon<Source> = &Daemon {
    names: &[name::Name::Ἄρτεμις, Artemis],
    words: &[],
    deeds: &[],

    father: Some(ΖΕΥΣ),
    mother: Some(LETO),
    teacher: None,

    predecessor: None,
};

pub const ACHILLES: &Daemon<Source> = &Daemon {
    names: &[Achilles],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: None,
};

pub const SOCRATES: &Daemon<Source> = &Daemon {
    names: &[Socrates],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: None,
};

pub const PLATO: &Daemon<Source> = &Daemon {
    names: &[Plato],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: Some(SOCRATES),

    predecessor: None,
};

pub const ARISTOTLE: &Daemon<Source> = &Daemon {
    names: &[Aristotle],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: Some(PLATO),

    predecessor: None,
};
