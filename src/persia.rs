use book::Book;
use daemon::Daemon;
use deed::Deed;
use name::Name::Cyrus;
use name::Name::Ezra;
use source::Source;

pub const CYRUS: &Daemon = &Daemon {
    names: &[Cyrus],
    words: &[],
    deeds: &[Deed {
        desc: "allowed the Israelites to return to the Promised Land",
        srcs: &[Source {
            book: Book { name: Ezra },
            chapter: 1,
            verses: [1, 1],
        }],
    }],

    father: None,
    mother: None,
    teacher: None,

    predecessor: None,
};
