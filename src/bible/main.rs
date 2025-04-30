use crate::Name;
use crate::src::Source;
use crate::deed::Deed;
use crate::book::Book;

// books
use crate::name::Name::Genesis;
use crate::name::Name::Exodus;
use crate::name::Name::Leviticus;
use crate::name::Name::Numbers;
use crate::name::Name::Deuteronomy;
use crate::name::Name::Joshua;
use crate::name::Name::Judges;
use crate::name::Name::Ruth;
use crate::name::Name::SamuelI;
use crate::name::Name::SamuelII;
use crate::name::Name::KingsI;
use crate::name::Name::KingsII;
use crate::name::Name::ChroniclesI;
use crate::name::Name::ChroniclesII;
use crate::name::Name::Ezra;
use crate::name::Name::Nehemiah;
use crate::name::Name::Esther;
use crate::name::Name::Job;
use crate::name::Name::Psalms;
use crate::name::Name::Proverbs;
use crate::name::Name::Ecclesiastes; 
use crate::name::Name::SongOfSolomon;
use crate::name::Name::Isaiah;
use crate::name::Name::Jeremiah;
use crate::name::Name::Ezekiel;
use crate::name::Name::Daniel;
use crate::name::Name::Hosea;
use crate::name::Name::Joel; 
use crate::name::Name::Amos;
use crate::name::Name::Obadiah;
use crate::name::Name::Jonah;
use crate::name::Name::Micah;
use crate::name::Name::Nahum;
use crate::name::Name::Habakkuk;
use crate::name::Name::Zephaniah;
use crate::name::Name::Haggai;
use crate::name::Name::Zechariah;
use crate::name::Name::Malachi;
use crate::name::Name::Matthew;
use crate::name::Name::Mark;
use crate::name::Name::Luke;
use crate::name::Name::John;
use crate::name::Name::Acts;
use crate::name::Name::Romans;
use crate::name::Name::CorinthiansI;
use crate::name::Name::CorinthiansII;
use crate::name::Name::Galatians;
use crate::name::Name::Ephesians;
use crate::name::Name::Philippians;
use crate::name::Name::Colossians;
use crate::name::Name::ThessaloniansI;
use crate::name::Name::ThessaloniansII;
use crate::name::Name::TimothyI;
use crate::name::Name::TimothyII;
use crate::name::Name::Titus;
use crate::name::Name::Philemon;
use crate::name::Name::Hebrews;
use crate::name::Name::James;
use crate::name::Name::PeterI;
use crate::name::Name::PeterII;
use crate::name::Name::JohnI;
use crate::name::Name::JohnII;
use crate::name::Name::JohnIII;
use crate::name::Name::Jude;
use crate::name::Name::Revelation;

// Luke 3:23-38
use crate::name::Name::God;
use crate::name::Name::Mary;
use crate::name::Name::Adam;
use crate::name::Name::Eve;
use crate::name::Name::Cain;
use crate::name::Name::Abel;
use crate::name::Name::Seth;
use crate::name::Name::Enosh;
use crate::name::Name::Kenan;
use crate::name::Name::Mahalalel;
use crate::name::Name::Jared;
use crate::name::Name::Enoch;
use crate::name::Name::Methuselah;
use crate::name::Name::Lamech;
use crate::name::Name::Noah;
use crate::name::Name::Shem;
use crate::name::Name::Arphaxad;
use crate::name::Name::Cainan;
use crate::name::Name::Shelah;
use crate::name::Name::Eber;
use crate::name::Name::Peleg;
use crate::name::Name::Reu;
use crate::name::Name::Serug;
use crate::name::Name::Nahor;
use crate::name::Name::Terrah;
use crate::name::Name::Abraham;
use crate::name::Name::Isaac;
use crate::name::Name::Jacob;
use crate::name::Name::Israel;
use crate::name::Name::Reuben;
use crate::name::Name::Amram;
use crate::name::Name::Jochebed;
use crate::name::Name::Moses;
use crate::name::Name::Aaron;
use crate::name::Name::Judah;
use crate::name::Name::Perez;
use crate::name::Name::Hezron;
use crate::name::Name::Ram;
use crate::name::Name::Amminadab;
use crate::name::Name::Nashon;
use crate::name::Name::Salmon;
use crate::name::Name::Boaz;
use crate::name::Name::Obed;
use crate::name::Name::Jesse;
use crate::name::Name::David;
use crate::name::Name::Nathan;
use crate::name::Name::Mattatha;
use crate::name::Name::Menna;
use crate::name::Name::Melea;
use crate::name::Name::Eliakim;
use crate::name::Name::Jonam;
use crate::name::Name::Joseph;
use crate::name::Name::Simeon;
use crate::name::Name::Levi;
use crate::name::Name::Matthat;
use crate::name::Name::Jorim;
use crate::name::Name::Eliezer;
use crate::name::Name::Er;
use crate::name::Name::Elmadam;
use crate::name::Name::Cosam;
use crate::name::Name::Addi;
use crate::name::Name::Melki;
use crate::name::Name::Neri;
use crate::name::Name::Shealtiel;
use crate::name::Name::Zerubbabel;
use crate::name::Name::Rhesa;
use crate::name::Name::Joanan;
use crate::name::Name::Joda;
use crate::name::Name::Josek;
use crate::name::Name::Semein;
use crate::name::Name::Mattathias;
use crate::name::Name::Maath;
use crate::name::Name::Naggai;
use crate::name::Name::Esli;
use crate::name::Name::Jannai;
use crate::name::Name::Heli;
use crate::name::Name::Jesus;
use crate::daemon::Daemon;

pub const BOOKS: [Name; 65] = [
    Genesis, Exodus, Leviticus, Numbers, Deuteronomy,
    Joshua, Judges, Ruth, SamuelI, SamuelII, KingsI, KingsII, ChroniclesI, ChroniclesII,
    Ezra, Nehemiah, Esther, Job,
    Psalms, Proverbs,
    Ecclesiastes, SongOfSolomon,
    Isaiah, Jeremiah, Ezekiel, Daniel, Hosea, Joel, Amos, Obadiah,
    Jonah,
    Micah, Nahum, Habakkuk, Zephaniah, Haggai, Zechariah, Malachi,

    Matthew, Mark, Luke, John,
    Acts, Romans, CorinthiansI, CorinthiansII, Galatians, Ephesians, Philippians,
    Colossians, ThessaloniansI, ThessaloniansII, TimothyI, TimothyII, Titus,
    Philemon, Hebrews, James, PeterI, PeterII, JohnI, JohnII, JohnIII, Jude,
    Revelation,
];

const ALMIGHTY: &Daemon = &Daemon {
    names: &[
        God,
    ],
    words: &[
        Source {
            book: Book {
                name: Leviticus,
            },
            chapter: 20,
            verses: [26, 26],
        },
    ],
    deeds: &[
        Deed {
            desc: "created the heavens and the earth",
            srcs: &[
                Source {
                    book: Book {
                        name: Genesis,
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
};

const MARY: &Daemon = &Daemon {
    names: &[
        Mary,
    ],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,
};

const ADAM: &Daemon = &Daemon {
    names: &[
        Adam,
    ],
    words: &[],
    deeds: &[],

    father: Some(ALMIGHTY),
    mother: None,
    teacher: None,
};

const EVE: &Daemon = &Daemon {
    names: &[
        Eve,
    ],
    words: &[],
    deeds: &[],

    father: Some(ALMIGHTY),
    mother: None,
    teacher: None,
};

const CAIN: &Daemon = &Daemon {
    names: &[
        Cain,
    ],
    words: &[],
    deeds: &[],

    father: Some(ADAM),
    mother: Some(EVE),
    teacher: None,
};

const ABEL: &Daemon = &Daemon {
    names: &[
        Abel,
    ],
    words: &[],
    deeds: &[],

    father: Some(ADAM),
    mother: Some(EVE),
    teacher: None,
};

const SETH: &Daemon = &Daemon {
    names: &[
        Seth,
    ],
    words: &[],
    deeds: &[],

    father: Some(ADAM),
    mother: None,
    teacher: None,
};

const ENOSH: &Daemon = &Daemon {
    names: &[
        Enosh,
    ],
    words: &[],
    deeds: &[],

    father: Some(SETH),
    mother: None,
    teacher: None,
};

const KENAN: &Daemon = &Daemon {
    names: &[
        Kenan,
    ],
    words: &[],
    deeds: &[],

    father: Some(ENOSH),
    mother: None,
    teacher: None,
};

const MAHALALEL: &Daemon = &Daemon {
    names: &[
        Mahalalel,
    ],
    words: &[],
    deeds: &[],

    father: Some(KENAN),
    mother: None,
    teacher: None,
};

const JARED: &Daemon = &Daemon {
    names: &[
        Jared,
    ],
    words: &[],
    deeds: &[],

    father: Some(MAHALALEL),
    mother: None,
    teacher: None,
};

const ENOCH: &Daemon = &Daemon {
    names: &[
        Enoch,
    ],
    words: &[],
    deeds: &[],

    father: Some(JARED),
    mother: None,
    teacher: None,
};

const METHUSELAH: &Daemon = &Daemon {
    names: &[
        Methuselah,
    ],
    words: &[],
    deeds: &[],

    father: Some(ENOCH),
    mother: None,
    teacher: None,
};

const LAMECH: &Daemon = &Daemon {
    names: &[
        Lamech,
    ],
    words: &[],
    deeds: &[],

    father: Some(METHUSELAH),
    mother: None,
    teacher: None,
};

const NOAH: &Daemon = &Daemon {
    names: &[
        Noah,
    ],
    words: &[],
    deeds: &[],

    father: Some(LAMECH),
    mother: None,
    teacher: None,
};

const SHEM: &Daemon = &Daemon {
    names: &[
        Shem,
    ],
    words: &[],
    deeds: &[],

    father: Some(NOAH),
    mother: None,
    teacher: None,
};

const ARPHAXAD: &Daemon = &Daemon {
    names: &[
        Arphaxad,
    ],
    words: &[],
    deeds: &[],

    father: Some(SHEM),
    mother: None,
    teacher: None,
};

const CAINAN: &Daemon = &Daemon {
    names: &[
        Cainan,
    ],
    words: &[],
    deeds: &[],

    father: Some(ARPHAXAD),
    mother: None,
    teacher: None,
};

const SHELAH: &Daemon = &Daemon {
    names: &[
        Shelah,
    ],
    words: &[],
    deeds: &[],

    father: Some(CAINAN),
    mother: None,
    teacher: None,
};

const EBER: &Daemon = &Daemon {
    names: &[
        Eber,
    ],
    words: &[],
    deeds: &[],

    father: Some(SHELAH),
    mother: None,
    teacher: None,
};

const PELEG: &Daemon = &Daemon {
    names: &[
        Peleg,
    ],
    words: &[],
    deeds: &[],

    father: Some(EBER),
    mother: None,
    teacher: None,
};

const REU: &Daemon = &Daemon {
    names: &[
        Reu,
    ],
    words: &[],
    deeds: &[],

    father: Some(PELEG),
    mother: None,
    teacher: None,
};

const SERUG: &Daemon = &Daemon {
    names: &[
        Serug,
    ],
    words: &[],
    deeds: &[],

    father: Some(REU),
    mother: None,
    teacher: None,
};

const NAHOR: &Daemon = &Daemon {
    names: &[
        Nahor,
    ],
    words: &[],
    deeds: &[],

    father: Some(SERUG),
    mother: None,
    teacher: None,
};

const TERRAH: &Daemon = &Daemon {
    names: &[
        Terrah,
    ],
    words: &[],
    deeds: &[],

    father: Some(NAHOR),
    mother: None,
    teacher: None,
};

const ABRAHAM: &Daemon = &Daemon {
    names: &[
        Abraham,
    ],
    words: &[],
    deeds: &[],

    father: Some(TERRAH),
    mother: None,
    teacher: None,
};

const ISAAC: &Daemon = &Daemon {
    names: &[
        Isaac,
    ],
    words: &[],
    deeds: &[],

    father: Some(ABRAHAM),
    mother: None,
    teacher: None,
};

const JACOB: &Daemon = &Daemon {
    names: &[
        Jacob,
        Israel,
    ],
    words: &[],
    deeds: &[],

    father: Some(ISAAC),
    mother: None,
    teacher: None,
};

const REUBEN: &Daemon = &Daemon {
    names: &[
        Reuben,
    ],
    words: &[],
    deeds: &[],

    father: Some(JACOB),
    mother: None,
    teacher: None,
};

const AMRAM: &Daemon = &Daemon {
    names: &[
        Amram,
    ],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,
};

const JOCHEBED: &Daemon = &Daemon {
    names: &[
        Jochebed,
    ],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,
};

const MOSES: &Daemon = &Daemon {
    names: &[
        Moses,
    ],
    words: &[],
    deeds: &[],

    father: Some(AMRAM),
    mother: Some(JOCHEBED),
    teacher: None,
};

const AARON: &Daemon = &Daemon {
    names: &[
        Aaron,
    ],
    words: &[],
    deeds: &[],

    father: Some(AMRAM),
    mother: Some(JOCHEBED),
    teacher: None,
};

const JUDAH_SON_OF_JACOB: &Daemon = &Daemon {
    names: &[
        Judah,
    ],
    words: &[],
    deeds: &[],

    father: Some(JACOB),
    mother: None,
    teacher: None,
};

const PEREZ: &Daemon = &Daemon {
    names: &[
        Perez,
    ],
    words: &[],
    deeds: &[],

    father: Some(JUDAH_SON_OF_JACOB),
    mother: None,
    teacher: None,
};

const HEZRON: &Daemon = &Daemon {
    names: &[
        Hezron,
    ],
    words: &[],
    deeds: &[],

    father: Some(PEREZ),
    mother: None,
    teacher: None,
};

const RAM: &Daemon = &Daemon {
    names: &[
        Ram,
    ],
    words: &[],
    deeds: &[],

    father: Some(HEZRON),
    mother: None,
    teacher: None,
};

const AMMINADAB: &Daemon = &Daemon {
    names: &[
        Amminadab,
    ],
    words: &[],
    deeds: &[],

    father: Some(RAM),
    mother: None,
    teacher: None,
};

const NASHON: &Daemon = &Daemon {
    names: &[
        Nashon,
    ],
    words: &[],
    deeds: &[],

    father: Some(AMMINADAB),
    mother: None,
    teacher: None,
};

const SALMON: &Daemon = &Daemon {
    names: &[
        Salmon,
    ],
    words: &[],
    deeds: &[],

    father: Some(NASHON),
    mother: None,
    teacher: None,
};

const BOAZ: &Daemon = &Daemon {
    names: &[
        Boaz,
    ],
    words: &[],
    deeds: &[],

    father: Some(SALMON),
    mother: None,
    teacher: None,
};

const OBED: &Daemon = &Daemon {
    names: &[
        Obed,
    ],
    words: &[],
    deeds: &[],

    father: Some(BOAZ),
    mother: None,
    teacher: None,
};

const JESSE: &Daemon = &Daemon {
    names: &[
        Jesse,
    ],
    words: &[],
    deeds: &[],

    father: Some(OBED),
    mother: None,
    teacher: None,
};

const DAVID: &Daemon = &Daemon {
    names: &[
        David,
    ],
    words: &[],
    deeds: &[],

    father: Some(JESSE),
    mother: None,
    teacher: None,
};

const NATHAN: &Daemon = &Daemon {
    names: &[
        Nathan,
    ],
    words: &[],
    deeds: &[],

    father: Some(DAVID),
    mother: None,
    teacher: None,
};

const MATTATHA: &Daemon = &Daemon {
    names: &[
        Mattatha,
    ],
    words: &[],
    deeds: &[],

    father: Some(NATHAN),
    mother: None,
    teacher: None,
};

const MENNA: &Daemon = &Daemon {
    names: &[
        Menna,
    ],
    words: &[],
    deeds: &[],

    father: Some(MATTATHA),
    mother: None,
    teacher: None,
};

const MELEA: &Daemon = &Daemon {
    names: &[
        Melea,
    ],
    words: &[],
    deeds: &[],

    father: Some(MENNA),
    mother: None,
    teacher: None,
};

const ELIAKIM: &Daemon = &Daemon {
    names: &[
        Eliakim,
    ],
    words: &[],
    deeds: &[],

    father: Some(MELEA),
    mother: None,
    teacher: None,
};

const JONAM: &Daemon = &Daemon {
    names: &[
        Jonam,
    ],
    words: &[],
    deeds: &[],

    father: Some(ELIAKIM),
    mother: None,
    teacher: None,
};

const JOSEPH_SON_OF_JONAM: &Daemon = &Daemon {
    names: &[
        Joseph,
    ],
    words: &[],
    deeds: &[],

    father: Some(JONAM),
    mother: None,
    teacher: None,
};

const JUDAH: &Daemon = &Daemon {
    names: &[
        Judah,
    ],
    words: &[],
    deeds: &[],

    father: Some(JOSEPH_SON_OF_JONAM),
    mother: None,
    teacher: None,
};

const SIMEON: &Daemon = &Daemon {
    names: &[
        Simeon,
    ],
    words: &[],
    deeds: &[],

    father: Some(JUDAH),
    mother: None,
    teacher: None,
};

const LEVI_SON_OF_SIMEON: &Daemon = &Daemon {
    names: &[
        Levi,
    ],
    words: &[],
    deeds: &[],

    father: Some(SIMEON),
    mother: None,
    teacher: None,
};

const MATTHAT_SON_OF_LEVI: &Daemon = &Daemon {
    names: &[
        Matthat,
    ],
    words: &[],
    deeds: &[],

    father: Some(LEVI_SON_OF_SIMEON),
    mother: None,
    teacher: None,
};

const JORIM: &Daemon = &Daemon {
    names: &[
        Jorim,
    ],
    words: &[],
    deeds: &[],

    father: Some(MATTHAT_SON_OF_LEVI),
    mother: None,
    teacher: None,
};

const ELIEZER: &Daemon = &Daemon {
    names: &[
        Eliezer,
    ],
    words: &[],
    deeds: &[],

    father: Some(JORIM),
    mother: None,
    teacher: None,
};

const JOSHUA: &Daemon = &Daemon {
    names: &[
        Joshua,
    ],
    words: &[],
    deeds: &[],

    father: Some(ELIEZER),
    mother: None,
    teacher: None,
};

const ER: &Daemon = &Daemon {
    names: &[
        Er,
    ],
    words: &[],
    deeds: &[],

    father: Some(JOSHUA),
    mother: None,
    teacher: None,
};

const ELMADAM: &Daemon = &Daemon {
    names: &[
        Elmadam,
    ],
    words: &[],
    deeds: &[],

    father: Some(ER),
    mother: None,
    teacher: None,
};

const COSAM: &Daemon = &Daemon {
    names: &[
        Cosam,
    ],
    words: &[],
    deeds: &[],

    father: Some(ELMADAM),
    mother: None,
    teacher: None,
};

const ADDI: &Daemon = &Daemon {
    names: &[
        Addi,
    ],
    words: &[],
    deeds: &[],

    father: Some(COSAM),
    mother: None,
    teacher: None,
};

const MELKI_SON_OF_ADDI: &Daemon = &Daemon {
    names: &[
        Melki,
    ],
    words: &[],
    deeds: &[],

    father: Some(ADDI),
    mother: None,
    teacher: None,
};

const NERI: &Daemon = &Daemon {
    names: &[
        Neri,
    ],
    words: &[],
    deeds: &[],

    father: Some(MELKI_SON_OF_ADDI),
    mother: None,
    teacher: None,
};

const SHEALTIEL: &Daemon = &Daemon {
    names: &[
        Shealtiel,
    ],
    words: &[],
    deeds: &[],

    father: Some(NERI),
    mother: None,
    teacher: None,
};

const ZERUBBABEL: &Daemon = &Daemon {
    names: &[
        Zerubbabel,
    ],
    words: &[],
    deeds: &[],

    father: Some(SHEALTIEL),
    mother: None,
    teacher: None,
};

const RHESA: &Daemon = &Daemon {
    names: &[
        Rhesa,
    ],
    words: &[],
    deeds: &[],

    father: Some(ZERUBBABEL),
    mother: None,
    teacher: None,
};

const JOANAN: &Daemon = &Daemon {
    names: &[
        Joanan,
    ],
    words: &[],
    deeds: &[],

    father: Some(RHESA),
    mother: None,
    teacher: None,
};

const JODA: &Daemon = &Daemon {
    names: &[
        Joda,
    ],
    words: &[],
    deeds: &[],

    father: Some(JOANAN),
    mother: None,
    teacher: None,
};

const JOSEK: &Daemon = &Daemon {
    names: &[
        Josek,
    ],
    words: &[],
    deeds: &[],

    father: Some(JODA),
    mother: None,
    teacher: None,
};

const SEMEIN: &Daemon = &Daemon {
    names: &[
        Semein,
    ],
    words: &[],
    deeds: &[],

    father: Some(JOSEK),
    mother: None,
    teacher: None,
};

const MATTATHIAS_SON_OF_SEMEIN: &Daemon = &Daemon {
    names: &[
        Mattathias,
    ],
    words: &[],
    deeds: &[],

    father: Some(SEMEIN),
    mother: None,
    teacher: None,
};

const MAATH: &Daemon = &Daemon {
    names: &[
        Maath,
    ],
    words: &[],
    deeds: &[],

    father: Some(MATTATHIAS_SON_OF_SEMEIN),
    mother: None,
    teacher: None,
};

const NAGGAI: &Daemon = &Daemon {
    names: &[
        Naggai,
    ],
    words: &[],
    deeds: &[],

    father: Some(MAATH),
    mother: None,
    teacher: None,
};

const ESLI: &Daemon = &Daemon {
    names: &[
        Esli,
    ],
    words: &[],
    deeds: &[],

    father: Some(NAGGAI),
    mother: None,
    teacher: None,
};

const NAHUM: &Daemon = &Daemon {
    names: &[
        Nahum,
    ],
    words: &[],
    deeds: &[],

    father: Some(ESLI),
    mother: None,
    teacher: None,
};

const AMOS: &Daemon = &Daemon {
    names: &[
        Amos,
    ],
    words: &[],
    deeds: &[],

    father: Some(NAHUM),
    mother: None,
    teacher: None,
};

const MATTATHIAS: &Daemon = &Daemon {
    names: &[
        Mattathias,
    ],
    words: &[],
    deeds: &[],

    father: Some(AMOS),
    mother: None,
    teacher: None,
};

const JOSEPH_SON_OF_MATTATHIAS: &Daemon = &Daemon {
    names: &[
        Joseph,
    ],
    words: &[],
    deeds: &[],

    father: Some(MATTATHIAS),
    mother: None,
    teacher: None,
};

const JANNAI: &Daemon = &Daemon {
    names: &[
        Jannai,
    ],
    words: &[],
    deeds: &[],

    father: Some(JOSEPH_SON_OF_MATTATHIAS),
    mother: None,
    teacher: None,
};

const MELKI: &Daemon = &Daemon {
    names: &[
        Melki,
    ],
    words: &[],
    deeds: &[],

    father: Some(JANNAI),
    mother: None,
    teacher: None,
};

const LEVI: &Daemon = &Daemon {
    names: &[
        Levi,
    ],
    words: &[],
    deeds: &[],

    father: Some(MELKI),
    mother: None,
    teacher: None,
};

const MATTHAT: &Daemon = &Daemon {
    names: &[
        Matthat,
    ],
    words: &[],
    deeds: &[],

    father: Some(LEVI),
    mother: None,
    teacher: None,
};

const HELI: &Daemon = &Daemon {
    names: &[
        Heli,
    ],
    words: &[],
    deeds: &[],

    father: Some(MATTHAT),
    mother: None,
    teacher: None,
};

const JOSEPH: &Daemon = &Daemon {
    names: &[
        Joseph,
    ],
    words: &[],
    deeds: &[
        Deed {
            desc: "decided to divorce Mary secretely to avoid her disgrace",
            srcs: &[
                Source {
                    book: Book {
                        name: Matthew,
                    },
                    chapter: 1,
                    verses: [19, 19],
                },
            ],
        },
    ],

    father: Some(HELI),
    mother: None,
    teacher: None,
};

pub const JESUS: &Daemon = &Daemon {
    names: &[
        Jesus,
    ],
    words: &[
        Source {
            book: Book {
                name: Matthew,
            },
            chapter: 5,
            verses: [13, 16],
        },
        Source {
            book: Book {
                name: Matthew,
            },
            chapter: 7,
            verses: [6, 7],
        },
        Source {
            book: Book {
                name: Matthew,
            },
            chapter: 22,
            verses: [21, 21],
        },
        Source {
            book: Book {
                name: Revelation,
            },
            chapter: 22,
            verses: [16, 21],
        },
    ],
    deeds: &[
        Deed {
            desc: "walked on water",
            srcs: &[
                Source {
                    book: Book {
                        name: Matthew,
                    },
                    chapter: 14,
                    verses: [25, 25],
                },
            ],
        },
    ],

    father: Some(JOSEPH),
    mother: Some(MARY),
    teacher: None,
};
