use crate::src::Source;
use crate::deed::Deed;
use crate::book::Book;
use crate::name::Name::Cyrus;
use crate::name::Name::Ezra;
use crate::daemon::Daemon;

pub const CYRUS: &Daemon = &Daemon {
    names: &[
        Cyrus,
    ],
    words: &[],
    deeds: &[
        Deed {
            desc: "allowed the Israelites to return to the Promised Land",
            srcs: &[
                Source {
                    book: Book {
                        name: Ezra,
                    },
                    chapter: 1,
                    verses: [1, 1],
                },
            ],
        },
    ],

    father: None,
    mother: None,
    teacher: None,

    predecessor: None,
};
