use daemon::Daemon;
use deed::Deed;
use name::Name::Cyrus;
use name::Name::Ezra;

pub const CYRUS: &Daemon<bible::Source> = &Daemon {
    names: &[Cyrus],
    words: &[],
    deeds: &[Deed {
        desc: "allowed the Israelites to return to the Promised Land",
        srcs: &[bible::Source {
            book: Ezra,
            chapter: 1,
            start: 1,
            end: None,
        }],
    }],

    father: None,
    mother: None,
    teacher: None,

    predecessor: None,
};
