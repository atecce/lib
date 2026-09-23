fn main() {
    for src in [
        bible::Source {
            book: name::Name::Genesis,
            chapter: 28,
            start: 10,
            end: Some(22),
        },
        bible::Source {
            book: name::Name::Exodus,
            chapter: 18,
            start: 14,
            end: Some(27),
        },
        bible::Source {
            book: name::Name::Leviticus,
            chapter: 20,
            start: 26,
            end: None,
        },
        bible::Source {
            book: name::Name::Numbers,
            chapter: 23,
            start: 9,
            end: None,
        },
        bible::Source {
            book: name::Name::Deuteronomy,
            chapter: 8,
            start: 3,
            end: None,
        },
        bible::Source {
            book: name::Name::Joshua,
            chapter: 7,
            start: 11,
            end: Some(26),
        },
        bible::Source {
            book: name::Name::Judges,
            chapter: 21,
            start: 25,
            end: None,
        },
        bible::Source {
            book: name::Name::SamuelI,
            chapter: 8,
            start: 19,
            end: Some(20),
        },
        bible::Source {
            book: name::Name::SamuelII,
            chapter: 2,
            start: 10,
            end: None,
        },
        bible::Source {
            book: name::Name::SamuelII,
            chapter: 22,
            start: 26,
            end: Some(28),
        },
        bible::Source {
            book: name::Name::SamuelII,
            chapter: 22,
            start: 50,
            end: Some(51),
        },
        bible::Source {
            book: name::Name::SamuelII,
            chapter: 23,
            start: 1,
            end: Some(4),
        },
        bible::Source {
            book: name::Name::SamuelII,
            chapter: 23,
            start: 13,
            end: Some(17),
        },
        bible::Source {
            book: name::Name::KingsI,
            chapter: 16,
            start: 30,
            end: Some(33),
        },
        bible::Source {
            book: name::Name::KingsI,
            chapter: 17,
            start: 15,
            end: Some(24),
        },
        bible::Source {
            book: name::Name::KingsII,
            chapter: 17,
            start: 18,
            end: Some(23),
        },
        bible::Source {
            book: name::Name::KingsII,
            chapter: 21,
            start: 1,
            end: Some(2),
        },
        bible::Source {
            book: name::Name::KingsII,
            chapter: 21,
            start: 7,
            end: Some(16),
        },
        bible::Source {
            book: name::Name::KingsII,
            chapter: 23,
            start: 21,
            end: Some(27),
        },
        bible::Source {
            book: name::Name::KingsII,
            chapter: 24,
            start: 1,
            end: Some(4),
        },
        bible::Source {
            book: name::Name::KingsII,
            chapter: 24,
            start: 8,
            end: Some(13),
        },
        bible::Source {
            book: name::Name::KingsII,
            chapter: 25,
            start: 1,
            end: Some(3),
        },
        bible::Source {
            book: name::Name::KingsII,
            chapter: 25,
            start: 8,
            end: Some(9),
        },
        bible::Source {
            book: name::Name::KingsII,
            chapter: 25,
            start: 21,
            end: None,
        },
        bible::Source {
            book: name::Name::KingsII,
            chapter: 25,
            start: 27,
            end: Some(30),
        },
        bible::Source {
            book: name::Name::ChroniclesI,
            chapter: 5,
            start: 1,
            end: Some(2),
        },
        bible::Source {
            book: name::Name::ChroniclesI,
            chapter: 5,
            start: 23,
            end: Some(26),
        },
        bible::Source {
            book: name::Name::ChroniclesI,
            chapter: 17,
            start: 3,
            end: Some(5),
        },
        bible::Source {
            book: name::Name::ChroniclesI,
            chapter: 17,
            start: 9,
            end: Some(14),
        },
        bible::Source {
            book: name::Name::ChroniclesI,
            chapter: 28,
            start: 2,
            end: Some(3),
        },
        bible::Source {
            book: name::Name::ChroniclesI,
            chapter: 28,
            start: 6,
            end: Some(7),
        },
        bible::Source {
            book: name::Name::ChroniclesII,
            chapter: 10,
            start: 6,
            end: Some(7),
        },
        bible::Source {
            book: name::Name::ChroniclesII,
            chapter: 10,
            start: 12,
            end: Some(14),
        },
        bible::Source {
            book: name::Name::ChroniclesII,
            chapter: 10,
            start: 16,
            end: Some(19),
        },
        bible::Source {
            book: name::Name::ChroniclesII,
            chapter: 11,
            start: 13,
            end: Some(16),
        },
        bible::Source {
            book: name::Name::ChroniclesII,
            chapter: 12,
            start: 5,
            end: None,
        },
        bible::Source {
            book: name::Name::ChroniclesII,
            chapter: 12,
            start: 9,
            end: None,
        },
        bible::Source {
            book: name::Name::ChroniclesII,
            chapter: 12,
            start: 12,
            end: None,
        },
        bible::Source {
            book: name::Name::ChroniclesII,
            chapter: 13,
            start: 4,
            end: Some(5),
        },
        bible::Source {
            book: name::Name::ChroniclesII,
            chapter: 13,
            start: 9,
            end: Some(10),
        },
        bible::Source {
            book: name::Name::ChroniclesII,
            chapter: 13,
            start: 13,
            end: Some(18),
        },
        bible::Source {
            book: name::Name::ChroniclesII,
            chapter: 13,
            start: 20,
            end: None,
        },
        bible::Source {
            book: name::Name::ChroniclesII,
            chapter: 20,
            start: 35,
            end: Some(37),
        },
        bible::Source {
            book: name::Name::ChroniclesII,
            chapter: 21,
            start: 5,
            end: Some(7),
        },
        bible::Source {
            book: name::Name::ChroniclesII,
            chapter: 21,
            start: 11,
            end: Some(20),
        },
        bible::Source {
            book: name::Name::ChroniclesII,
            chapter: 36,
            start: 11,
            end: Some(21),
        },
        bible::Source {
            book: name::Name::Ezra,
            chapter: 1,
            start: 1,
            end: Some(5),
        },
        bible::Source {
            book: name::Name::Nehemiah,
            chapter: 1,
            start: 8,
            end: Some(9),
        },
        bible::Source {
            book: name::Name::Nehemiah,
            chapter: 6,
            start: 13,
            end: None,
        },
        bible::Source {
            book: name::Name::Job,
            chapter: 1,
            start: 6,
            end: Some(12),
        },
        bible::Source {
            book: name::Name::Job,
            chapter: 5,
            start: 13,
            end: Some(16),
        },
        bible::Source {
            book: name::Name::Job,
            chapter: 38,
            start: 4,
            end: None,
        },
        bible::Source {
            book: name::Name::Job,
            chapter: 38,
            start: 7,
            end: None,
        },
    ] {
        let verses = bible::kjv::word(src.book)[src.chapter as usize - 1];
        println!("{}", src);
        let text: &[&str];
        if let Some(end) = src.end {
            text = &verses[src.start as usize - 1..=end as usize - 1];
        } else {
            text = &verses[src.start as usize - 1..src.start as usize];
        }
        println!("{:#?}", text);
    }
}
