pub mod io;

use name::Name;
use book::Book;
use deed::Deed;
use source::Source;

// books
use name::Name::Acts;
use name::Name::Amos;
use name::Name::ChroniclesI;
use name::Name::ChroniclesII;
use name::Name::Colossians;
use name::Name::CorinthiansI;
use name::Name::CorinthiansII;
use name::Name::Daniel;
use name::Name::Deuteronomy;
use name::Name::Ecclesiastes;
use name::Name::Ephesians;
use name::Name::Esther;
use name::Name::Exodus;
use name::Name::Ezekiel;
use name::Name::Ezra;
use name::Name::Galatians;
use name::Name::Genesis;
use name::Name::Habakkuk;
use name::Name::Haggai;
use name::Name::Hebrews;
use name::Name::Hosea;
use name::Name::Isaiah;
use name::Name::James;
use name::Name::Jeremiah;
use name::Name::Job;
use name::Name::Joel;
use name::Name::John;
use name::Name::JohnI;
use name::Name::JohnII;
use name::Name::JohnIII;
use name::Name::Jonah;
use name::Name::Joshua;
use name::Name::Jude;
use name::Name::Judges;
use name::Name::KingsI;
use name::Name::KingsII;
use name::Name::Leviticus;
use name::Name::Luke;
use name::Name::Malachi;
use name::Name::Mark;
use name::Name::Matthew;
use name::Name::Micah;
use name::Name::Nahum;
use name::Name::Nehemiah;
use name::Name::Numbers;
use name::Name::Obadiah;
use name::Name::PeterI;
use name::Name::PeterII;
use name::Name::Philemon;
use name::Name::Philippians;
use name::Name::Proverbs;
use name::Name::Psalms;
use name::Name::Revelation;
use name::Name::Romans;
use name::Name::Ruth;
use name::Name::SamuelI;
use name::Name::SamuelII;
use name::Name::SongOfSolomon;
use name::Name::ThessaloniansI;
use name::Name::ThessaloniansII;
use name::Name::TimothyI;
use name::Name::TimothyII;
use name::Name::Titus;
use name::Name::Zechariah;
use name::Name::Zephaniah;

// Luke 3:23-38
use daemon::Daemon;
use name::Name::Aaron;
use name::Name::Abel;
use name::Name::Abraham;
use name::Name::Adam;
use name::Name::Addi;
use name::Name::Amminadab;
use name::Name::Amram;
use name::Name::Arphaxad;
use name::Name::Boaz;
use name::Name::Cain;
use name::Name::Cainan;
use name::Name::Cosam;
use name::Name::David;
use name::Name::Eber;
use name::Name::Eliakim;
use name::Name::Eliezer;
use name::Name::Elmadam;
use name::Name::Enoch;
use name::Name::Enosh;
use name::Name::Er;
use name::Name::Esli;
use name::Name::Eve;
use name::Name::God;
use name::Name::Heli;
use name::Name::Hezron;
use name::Name::Isaac;
use name::Name::Israel;
use name::Name::Jacob;
use name::Name::Jannai;
use name::Name::Jared;
use name::Name::Jesse;
use name::Name::Jesus;
use name::Name::Joanan;
use name::Name::Jochebed;
use name::Name::Joda;
use name::Name::Jonam;
use name::Name::Jorim;
use name::Name::Josek;
use name::Name::Joseph;
use name::Name::Judah;
use name::Name::Kenan;
use name::Name::Lamech;
use name::Name::Levi;
use name::Name::Maath;
use name::Name::Mahalalel;
use name::Name::Mary;
use name::Name::Mattatha;
use name::Name::Mattathias;
use name::Name::Matthat;
use name::Name::Melea;
use name::Name::Melki;
use name::Name::Menna;
use name::Name::Methuselah;
use name::Name::Moses;
use name::Name::Naggai;
use name::Name::Nahor;
use name::Name::Nashon;
use name::Name::Nathan;
use name::Name::Neri;
use name::Name::Noah;
use name::Name::Obed;
use name::Name::Peleg;
use name::Name::Perez;
use name::Name::Ram;
use name::Name::Reu;
use name::Name::Reuben;
use name::Name::Rhesa;
use name::Name::Salmon;
use name::Name::Semein;
use name::Name::Serug;
use name::Name::Seth;
use name::Name::Shealtiel;
use name::Name::Shelah;
use name::Name::Shem;
use name::Name::Simeon;
use name::Name::Terrah;
use name::Name::Zerubbabel;

pub const BOOKS: [Name; 65] = [
    Genesis,
    Exodus,
    Leviticus,
    Numbers,
    Deuteronomy,
    Joshua,
    Judges,
    Ruth,
    SamuelI,
    SamuelII,
    KingsI,
    KingsII,
    ChroniclesI,
    ChroniclesII,
    Ezra,
    Nehemiah,
    Esther,
    Job,
    Psalms,
    Proverbs,
    Ecclesiastes,
    SongOfSolomon,
    Isaiah,
    Jeremiah,
    Ezekiel,
    Daniel,
    Hosea,
    Joel,
    Amos,
    Obadiah,
    Jonah,
    Micah,
    Nahum,
    Habakkuk,
    Zephaniah,
    Haggai,
    Zechariah,
    Malachi,
    Matthew,
    Mark,
    Luke,
    John,
    Acts,
    Romans,
    CorinthiansI,
    CorinthiansII,
    Galatians,
    Ephesians,
    Philippians,
    Colossians,
    ThessaloniansI,
    ThessaloniansII,
    TimothyI,
    TimothyII,
    Titus,
    Philemon,
    Hebrews,
    James,
    PeterI,
    PeterII,
    JohnI,
    JohnII,
    JohnIII,
    Jude,
    Revelation,
];

const ALMIGHTY: &Daemon = &Daemon {
    names: &[God],
    words: &[Source {
        book: Book { name: Leviticus },
        chapter: 20,
        verses: [26, 26],
    }],
    deeds: &[Deed {
        desc: "created the heavens and the earth",
        srcs: &[Source {
            book: Book { name: Genesis },
            chapter: 1,
            verses: [1, 1],
        }],
    }],

    father: None,
    mother: None,
    teacher: None,

    predecessor: None,
};

const MARY: &Daemon = &Daemon {
    names: &[Mary],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: None,
};

const ADAM: &Daemon = &Daemon {
    names: &[Adam],
    words: &[],
    deeds: &[],

    father: Some(ALMIGHTY),
    mother: None,
    teacher: None,

    predecessor: None,
};

const EVE: &Daemon = &Daemon {
    names: &[Eve],
    words: &[],
    deeds: &[],

    father: Some(ALMIGHTY),
    mother: None,
    teacher: None,

    predecessor: None,
};

const CAIN: &Daemon = &Daemon {
    names: &[Cain],
    words: &[],
    deeds: &[],

    father: Some(ADAM),
    mother: Some(EVE),
    teacher: None,

    predecessor: None,
};

const ABEL: &Daemon = &Daemon {
    names: &[Abel],
    words: &[],
    deeds: &[],

    father: Some(ADAM),
    mother: Some(EVE),
    teacher: None,

    predecessor: None,
};

const SETH: &Daemon = &Daemon {
    names: &[Seth],
    words: &[],
    deeds: &[],

    father: Some(ADAM),
    mother: None,
    teacher: None,

    predecessor: None,
};

const ENOSH: &Daemon = &Daemon {
    names: &[Enosh],
    words: &[],
    deeds: &[],

    father: Some(SETH),
    mother: None,
    teacher: None,

    predecessor: None,
};

const KENAN: &Daemon = &Daemon {
    names: &[Kenan],
    words: &[],
    deeds: &[],

    father: Some(ENOSH),
    mother: None,
    teacher: None,

    predecessor: None,
};

const MAHALALEL: &Daemon = &Daemon {
    names: &[Mahalalel],
    words: &[],
    deeds: &[],

    father: Some(KENAN),
    mother: None,
    teacher: None,

    predecessor: None,
};

const JARED: &Daemon = &Daemon {
    names: &[Jared],
    words: &[],
    deeds: &[],

    father: Some(MAHALALEL),
    mother: None,
    teacher: None,

    predecessor: None,
};

const ENOCH: &Daemon = &Daemon {
    names: &[Enoch],
    words: &[],
    deeds: &[],

    father: Some(JARED),
    mother: None,
    teacher: None,

    predecessor: None,
};

const METHUSELAH: &Daemon = &Daemon {
    names: &[Methuselah],
    words: &[],
    deeds: &[],

    father: Some(ENOCH),
    mother: None,
    teacher: None,

    predecessor: None,
};

const LAMECH: &Daemon = &Daemon {
    names: &[Lamech],
    words: &[],
    deeds: &[],

    father: Some(METHUSELAH),
    mother: None,
    teacher: None,

    predecessor: None,
};

const NOAH: &Daemon = &Daemon {
    names: &[Noah],
    words: &[],
    deeds: &[],

    father: Some(LAMECH),
    mother: None,
    teacher: None,

    predecessor: None,
};

const SHEM: &Daemon = &Daemon {
    names: &[Shem],
    words: &[],
    deeds: &[],

    father: Some(NOAH),
    mother: None,
    teacher: None,

    predecessor: None,
};

const ARPHAXAD: &Daemon = &Daemon {
    names: &[Arphaxad],
    words: &[],
    deeds: &[],

    father: Some(SHEM),
    mother: None,
    teacher: None,

    predecessor: None,
};

const CAINAN: &Daemon = &Daemon {
    names: &[Cainan],
    words: &[],
    deeds: &[],

    father: Some(ARPHAXAD),
    mother: None,
    teacher: None,

    predecessor: None,
};

const SHELAH: &Daemon = &Daemon {
    names: &[Shelah],
    words: &[],
    deeds: &[],

    father: Some(CAINAN),
    mother: None,
    teacher: None,

    predecessor: None,
};

const EBER: &Daemon = &Daemon {
    names: &[Eber],
    words: &[],
    deeds: &[],

    father: Some(SHELAH),
    mother: None,
    teacher: None,

    predecessor: None,
};

const PELEG: &Daemon = &Daemon {
    names: &[Peleg],
    words: &[],
    deeds: &[],

    father: Some(EBER),
    mother: None,
    teacher: None,

    predecessor: None,
};

const REU: &Daemon = &Daemon {
    names: &[Reu],
    words: &[],
    deeds: &[],

    father: Some(PELEG),
    mother: None,
    teacher: None,

    predecessor: None,
};

const SERUG: &Daemon = &Daemon {
    names: &[Serug],
    words: &[],
    deeds: &[],

    father: Some(REU),
    mother: None,
    teacher: None,

    predecessor: None,
};

const NAHOR: &Daemon = &Daemon {
    names: &[Nahor],
    words: &[],
    deeds: &[],

    father: Some(SERUG),
    mother: None,
    teacher: None,

    predecessor: None,
};

const TERRAH: &Daemon = &Daemon {
    names: &[Terrah],
    words: &[],
    deeds: &[],

    father: Some(NAHOR),
    mother: None,
    teacher: None,

    predecessor: None,
};

const ABRAHAM: &Daemon = &Daemon {
    names: &[Abraham],
    words: &[],
    deeds: &[],

    father: Some(TERRAH),
    mother: None,
    teacher: None,

    predecessor: None,
};

const ISAAC: &Daemon = &Daemon {
    names: &[Isaac],
    words: &[],
    deeds: &[],

    father: Some(ABRAHAM),
    mother: None,
    teacher: None,

    predecessor: None,
};

const JACOB: &Daemon = &Daemon {
    names: &[Jacob, Israel],
    words: &[],
    deeds: &[],

    father: Some(ISAAC),
    mother: None,
    teacher: None,

    predecessor: None,
};

const REUBEN: &Daemon = &Daemon {
    names: &[Reuben],
    words: &[],
    deeds: &[],

    father: Some(JACOB),
    mother: None,
    teacher: None,

    predecessor: None,
};

const AMRAM: &Daemon = &Daemon {
    names: &[Amram],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: None,
};

const JOCHEBED: &Daemon = &Daemon {
    names: &[Jochebed],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: None,
};

const MOSES: &Daemon = &Daemon {
    names: &[Moses],
    words: &[],
    deeds: &[],

    father: Some(AMRAM),
    mother: Some(JOCHEBED),
    teacher: None,

    predecessor: None,
};

const AARON: &Daemon = &Daemon {
    names: &[Aaron],
    words: &[],
    deeds: &[],

    father: Some(AMRAM),
    mother: Some(JOCHEBED),
    teacher: None,

    predecessor: None,
};

const JUDAH_SON_OF_JACOB: &Daemon = &Daemon {
    names: &[Judah],
    words: &[],
    deeds: &[],

    father: Some(JACOB),
    mother: None,
    teacher: None,

    predecessor: None,
};

const PEREZ: &Daemon = &Daemon {
    names: &[Perez],
    words: &[],
    deeds: &[],

    father: Some(JUDAH_SON_OF_JACOB),
    mother: None,
    teacher: None,

    predecessor: None,
};

const HEZRON: &Daemon = &Daemon {
    names: &[Hezron],
    words: &[],
    deeds: &[],

    father: Some(PEREZ),
    mother: None,
    teacher: None,

    predecessor: None,
};

const RAM: &Daemon = &Daemon {
    names: &[Ram],
    words: &[],
    deeds: &[],

    father: Some(HEZRON),
    mother: None,
    teacher: None,

    predecessor: None,
};

const AMMINADAB: &Daemon = &Daemon {
    names: &[Amminadab],
    words: &[],
    deeds: &[],

    father: Some(RAM),
    mother: None,
    teacher: None,

    predecessor: None,
};

const NASHON: &Daemon = &Daemon {
    names: &[Nashon],
    words: &[],
    deeds: &[],

    father: Some(AMMINADAB),
    mother: None,
    teacher: None,

    predecessor: None,
};

const SALMON: &Daemon = &Daemon {
    names: &[Salmon],
    words: &[],
    deeds: &[],

    father: Some(NASHON),
    mother: None,
    teacher: None,

    predecessor: None,
};

const BOAZ: &Daemon = &Daemon {
    names: &[Boaz],
    words: &[],
    deeds: &[],

    father: Some(SALMON),
    mother: None,
    teacher: None,

    predecessor: None,
};

const OBED: &Daemon = &Daemon {
    names: &[Obed],
    words: &[],
    deeds: &[],

    father: Some(BOAZ),
    mother: None,
    teacher: None,

    predecessor: None,
};

const JESSE: &Daemon = &Daemon {
    names: &[Jesse],
    words: &[],
    deeds: &[],

    father: Some(OBED),
    mother: None,
    teacher: None,

    predecessor: None,
};

const DAVID: &Daemon = &Daemon {
    names: &[David],
    words: &[],
    deeds: &[],

    father: Some(JESSE),
    mother: None,
    teacher: None,

    predecessor: None,
};

const NATHAN: &Daemon = &Daemon {
    names: &[Nathan],
    words: &[],
    deeds: &[],

    father: Some(DAVID),
    mother: None,
    teacher: None,

    predecessor: None,
};

const MATTATHA: &Daemon = &Daemon {
    names: &[Mattatha],
    words: &[],
    deeds: &[],

    father: Some(NATHAN),
    mother: None,
    teacher: None,

    predecessor: None,
};

const MENNA: &Daemon = &Daemon {
    names: &[Menna],
    words: &[],
    deeds: &[],

    father: Some(MATTATHA),
    mother: None,
    teacher: None,

    predecessor: None,
};

const MELEA: &Daemon = &Daemon {
    names: &[Melea],
    words: &[],
    deeds: &[],

    father: Some(MENNA),
    mother: None,
    teacher: None,

    predecessor: None,
};

const ELIAKIM: &Daemon = &Daemon {
    names: &[Eliakim],
    words: &[],
    deeds: &[],

    father: Some(MELEA),
    mother: None,
    teacher: None,

    predecessor: None,
};

const JONAM: &Daemon = &Daemon {
    names: &[Jonam],
    words: &[],
    deeds: &[],

    father: Some(ELIAKIM),
    mother: None,
    teacher: None,

    predecessor: None,
};

const JOSEPH_SON_OF_JONAM: &Daemon = &Daemon {
    names: &[Joseph],
    words: &[],
    deeds: &[],

    father: Some(JONAM),
    mother: None,
    teacher: None,

    predecessor: None,
};

const JUDAH: &Daemon = &Daemon {
    names: &[Judah],
    words: &[],
    deeds: &[],

    father: Some(JOSEPH_SON_OF_JONAM),
    mother: None,
    teacher: None,

    predecessor: None,
};

const SIMEON: &Daemon = &Daemon {
    names: &[Simeon],
    words: &[],
    deeds: &[],

    father: Some(JUDAH),
    mother: None,
    teacher: None,

    predecessor: None,
};

const LEVI_SON_OF_SIMEON: &Daemon = &Daemon {
    names: &[Levi],
    words: &[],
    deeds: &[],

    father: Some(SIMEON),
    mother: None,
    teacher: None,

    predecessor: None,
};

const MATTHAT_SON_OF_LEVI: &Daemon = &Daemon {
    names: &[Matthat],
    words: &[],
    deeds: &[],

    father: Some(LEVI_SON_OF_SIMEON),
    mother: None,
    teacher: None,

    predecessor: None,
};

const JORIM: &Daemon = &Daemon {
    names: &[Jorim],
    words: &[],
    deeds: &[],

    father: Some(MATTHAT_SON_OF_LEVI),
    mother: None,
    teacher: None,

    predecessor: None,
};

const ELIEZER: &Daemon = &Daemon {
    names: &[Eliezer],
    words: &[],
    deeds: &[],

    father: Some(JORIM),
    mother: None,
    teacher: None,

    predecessor: None,
};

const JOSHUA: &Daemon = &Daemon {
    names: &[Joshua],
    words: &[],
    deeds: &[],

    father: Some(ELIEZER),
    mother: None,
    teacher: None,

    predecessor: None,
};

const ER: &Daemon = &Daemon {
    names: &[Er],
    words: &[],
    deeds: &[],

    father: Some(JOSHUA),
    mother: None,
    teacher: None,

    predecessor: None,
};

const ELMADAM: &Daemon = &Daemon {
    names: &[Elmadam],
    words: &[],
    deeds: &[],

    father: Some(ER),
    mother: None,
    teacher: None,

    predecessor: None,
};

const COSAM: &Daemon = &Daemon {
    names: &[Cosam],
    words: &[],
    deeds: &[],

    father: Some(ELMADAM),
    mother: None,
    teacher: None,

    predecessor: None,
};

const ADDI: &Daemon = &Daemon {
    names: &[Addi],
    words: &[],
    deeds: &[],

    father: Some(COSAM),
    mother: None,
    teacher: None,

    predecessor: None,
};

const MELKI_SON_OF_ADDI: &Daemon = &Daemon {
    names: &[Melki],
    words: &[],
    deeds: &[],

    father: Some(ADDI),
    mother: None,
    teacher: None,

    predecessor: None,
};

const NERI: &Daemon = &Daemon {
    names: &[Neri],
    words: &[],
    deeds: &[],

    father: Some(MELKI_SON_OF_ADDI),
    mother: None,
    teacher: None,

    predecessor: None,
};

const SHEALTIEL: &Daemon = &Daemon {
    names: &[Shealtiel],
    words: &[],
    deeds: &[],

    father: Some(NERI),
    mother: None,
    teacher: None,

    predecessor: None,
};

const ZERUBBABEL: &Daemon = &Daemon {
    names: &[Zerubbabel],
    words: &[],
    deeds: &[],

    father: Some(SHEALTIEL),
    mother: None,
    teacher: None,

    predecessor: None,
};

const RHESA: &Daemon = &Daemon {
    names: &[Rhesa],
    words: &[],
    deeds: &[],

    father: Some(ZERUBBABEL),
    mother: None,
    teacher: None,

    predecessor: None,
};

const JOANAN: &Daemon = &Daemon {
    names: &[Joanan],
    words: &[],
    deeds: &[],

    father: Some(RHESA),
    mother: None,
    teacher: None,

    predecessor: None,
};

const JODA: &Daemon = &Daemon {
    names: &[Joda],
    words: &[],
    deeds: &[],

    father: Some(JOANAN),
    mother: None,
    teacher: None,

    predecessor: None,
};

const JOSEK: &Daemon = &Daemon {
    names: &[Josek],
    words: &[],
    deeds: &[],

    father: Some(JODA),
    mother: None,
    teacher: None,

    predecessor: None,
};

const SEMEIN: &Daemon = &Daemon {
    names: &[Semein],
    words: &[],
    deeds: &[],

    father: Some(JOSEK),
    mother: None,
    teacher: None,

    predecessor: None,
};

const MATTATHIAS_SON_OF_SEMEIN: &Daemon = &Daemon {
    names: &[Mattathias],
    words: &[],
    deeds: &[],

    father: Some(SEMEIN),
    mother: None,
    teacher: None,

    predecessor: None,
};

const MAATH: &Daemon = &Daemon {
    names: &[Maath],
    words: &[],
    deeds: &[],

    father: Some(MATTATHIAS_SON_OF_SEMEIN),
    mother: None,
    teacher: None,

    predecessor: None,
};

const NAGGAI: &Daemon = &Daemon {
    names: &[Naggai],
    words: &[],
    deeds: &[],

    father: Some(MAATH),
    mother: None,
    teacher: None,

    predecessor: None,
};

const ESLI: &Daemon = &Daemon {
    names: &[Esli],
    words: &[],
    deeds: &[],

    father: Some(NAGGAI),
    mother: None,
    teacher: None,

    predecessor: None,
};

const NAHUM: &Daemon = &Daemon {
    names: &[Nahum],
    words: &[],
    deeds: &[],

    father: Some(ESLI),
    mother: None,
    teacher: None,

    predecessor: None,
};

const AMOS: &Daemon = &Daemon {
    names: &[Amos],
    words: &[],
    deeds: &[],

    father: Some(NAHUM),
    mother: None,
    teacher: None,

    predecessor: None,
};

const MATTATHIAS: &Daemon = &Daemon {
    names: &[Mattathias],
    words: &[],
    deeds: &[],

    father: Some(AMOS),
    mother: None,
    teacher: None,

    predecessor: None,
};

const JOSEPH_SON_OF_MATTATHIAS: &Daemon = &Daemon {
    names: &[Joseph],
    words: &[],
    deeds: &[],

    father: Some(MATTATHIAS),
    mother: None,
    teacher: None,

    predecessor: None,
};

const JANNAI: &Daemon = &Daemon {
    names: &[Jannai],
    words: &[],
    deeds: &[],

    father: Some(JOSEPH_SON_OF_MATTATHIAS),
    mother: None,
    teacher: None,

    predecessor: None,
};

const MELKI: &Daemon = &Daemon {
    names: &[Melki],
    words: &[],
    deeds: &[],

    father: Some(JANNAI),
    mother: None,
    teacher: None,

    predecessor: None,
};

const LEVI: &Daemon = &Daemon {
    names: &[Levi],
    words: &[],
    deeds: &[],

    father: Some(MELKI),
    mother: None,
    teacher: None,

    predecessor: None,
};

const MATTHAT: &Daemon = &Daemon {
    names: &[Matthat],
    words: &[],
    deeds: &[],

    father: Some(LEVI),
    mother: None,
    teacher: None,

    predecessor: None,
};

const HELI: &Daemon = &Daemon {
    names: &[Heli],
    words: &[],
    deeds: &[],

    father: Some(MATTHAT),
    mother: None,
    teacher: None,

    predecessor: None,
};

const JOSEPH: &Daemon = &Daemon {
    names: &[Joseph],
    words: &[],
    deeds: &[Deed {
        desc: "decided to divorce Mary secretely to avoid her disgrace",
        srcs: &[Source {
            book: Book { name: Matthew },
            chapter: 1,
            verses: [19, 19],
        }],
    }],

    father: Some(HELI),
    mother: None,
    teacher: None,

    predecessor: None,
};

pub const JESUS: &Daemon = &Daemon {
    names: &[Jesus],
    words: &[
        Source {
            book: Book { name: Matthew },
            chapter: 5,
            verses: [13, 16],
        },
        Source {
            book: Book { name: Matthew },
            chapter: 7,
            verses: [6, 7],
        },
        Source {
            book: Book { name: Matthew },
            chapter: 22,
            verses: [21, 21],
        },
        Source {
            book: Book { name: Revelation },
            chapter: 22,
            verses: [16, 16],
        },
    ],
    deeds: &[Deed {
        desc: "walked on water",
        srcs: &[Source {
            book: Book { name: Matthew },
            chapter: 14,
            verses: [25, 25],
        }],
    }],

    father: Some(JOSEPH),
    mother: Some(MARY),
    teacher: None,

    predecessor: None,
};

#[test]
fn yeshua() {
    let word = io::read_all();

    for src in JESUS.words {
        println!("book: {}", src.book.name);
        println!("chapter: {}", src.chapter);
        println!("verses: {:?}", src.verses);
        let text = &word[&src.book.name][src.chapter - 1][src.verses[0] - 1..=src.verses[1] - 1];
        println!("{:?}", text);
    }

    for deed in JESUS.deeds {
        println!("desc: {}", deed.desc);
        for src in deed.srcs {
            println!("book: {}", src.book.name);
            println!("chapter: {}", src.chapter);
            println!("verses: {:?}", src.verses);
            let text =
                &word[&src.book.name][src.chapter - 1][src.verses[0] - 1..=src.verses[1] - 1];
            println!("{:?}", text);
        }
    }

    for (book, chapter_and_verse) in &word {
        for (i, chapter) in chapter_and_verse.iter().enumerate() {
            for (j, verse) in chapter.iter().enumerate() {
                if verse.contains("Joshua") {
                    println!("{} {}:{}", book, i + 1, j + 1);
                    println!("{}", verse);
                }
            }
        }
    }

    let mut cur = JESUS.father;
    while let Some(node) = cur {
        // TODO(atec): probably some index check
        println!("{:#?}", node.names[0]);
        cur = node.father;
    }
}
