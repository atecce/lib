use crate::daemon::Daemon;

// Luke 3:23-38
const ALMIGHTY: &Daemon = &Daemon {
    names: &[
        "God",
    ],
    words: &[
        "text is the universal interface",
    ],
    deeds: &[
        "creation",
    ],

    father: None,
    mother: None,
    teacher: None,
};

const MARY: &Daemon = &Daemon {
    names: &[
        "Mary",
    ],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,
};

const ADAM: &Daemon = &Daemon {
    names: &[
        "Adam",
    ],
    words: &[],
    deeds: &[],

    father: Some(ALMIGHTY),
    mother: None,
    teacher: None,
};

const EVE: &Daemon = &Daemon {
    names: &[
        "Eve",
    ],
    words: &[],
    deeds: &[],

    father: Some(ALMIGHTY),
    mother: None,
    teacher: None,
};

const CAIN: &Daemon = &Daemon {
    names: &[
        "Cain",
    ],
    words: &[],
    deeds: &[],

    father: Some(ADAM),
    mother: Some(EVE),
    teacher: None,
};

const ABEL: &Daemon = &Daemon {
    names: &[
        "Abel",
    ],
    words: &[],
    deeds: &[],

    father: Some(ADAM),
    mother: Some(EVE),
    teacher: None,
};

const SETH: &Daemon = &Daemon {
    names: &[
        "Seth",
    ],
    words: &[],
    deeds: &[],

    father: Some(ADAM),
    mother: None,
    teacher: None,
};

const ENOSH: &Daemon = &Daemon {
    names: &[
        "Enosh",
    ],
    words: &[],
    deeds: &[],

    father: Some(SETH),
    mother: None,
    teacher: None,
};

const KENAN: &Daemon = &Daemon {
    names: &[
        "Kenan",
    ],
    words: &[],
    deeds: &[],

    father: Some(ENOSH),
    mother: None,
    teacher: None,
};

const MAHALALEL: &Daemon = &Daemon {
    names: &[
        "Mahalalel",
    ],
    words: &[],
    deeds: &[],

    father: Some(KENAN),
    mother: None,
    teacher: None,
};

const JARED: &Daemon = &Daemon {
    names: &[
        "Jared",
    ],
    words: &[],
    deeds: &[],

    father: Some(MAHALALEL),
    mother: None,
    teacher: None,
};

const ENOCH: &Daemon = &Daemon {
    names: &[
        "Enoch",
    ],
    words: &[],
    deeds: &[],

    father: Some(JARED),
    mother: None,
    teacher: None,
};

const METHUSELAH: &Daemon = &Daemon {
    names: &[
        "Methuselah",
    ],
    words: &[],
    deeds: &[],

    father: Some(ENOCH),
    mother: None,
    teacher: None,
};

const LAMECH: &Daemon = &Daemon {
    names: &[
        "Lamech",
    ],
    words: &[],
    deeds: &[],

    father: Some(METHUSELAH),
    mother: None,
    teacher: None,
};

const NOAH: &Daemon = &Daemon {
    names: &[
        "Noah",
    ],
    words: &[],
    deeds: &[],

    father: Some(LAMECH),
    mother: None,
    teacher: None,
};

const SHEM: &Daemon = &Daemon {
    names: &[
        "Shem",
    ],
    words: &[],
    deeds: &[],

    father: Some(NOAH),
    mother: None,
    teacher: None,
};

const ARPHAXAD: &Daemon = &Daemon {
    names: &[
        "Arphaxad",
    ],
    words: &[],
    deeds: &[],

    father: Some(SHEM),
    mother: None,
    teacher: None,
};

const CAINAN: &Daemon = &Daemon {
    names: &[
        "Cainan",
    ],
    words: &[],
    deeds: &[],

    father: Some(ARPHAXAD),
    mother: None,
    teacher: None,
};

const SHELAH: &Daemon = &Daemon {
    names: &[
        "Eber",
    ],
    words: &[],
    deeds: &[],

    father: Some(CAINAN),
    mother: None,
    teacher: None,
};

const EBER: &Daemon = &Daemon {
    names: &[
        "Eber",
    ],
    words: &[],
    deeds: &[],

    father: Some(SHELAH),
    mother: None,
    teacher: None,
};

const PELEG: &Daemon = &Daemon {
    names: &[
        "Peleg",
    ],
    words: &[],
    deeds: &[],

    father: Some(EBER),
    mother: None,
    teacher: None,
};

const REU: &Daemon = &Daemon {
    names: &[
        "Reu",
    ],
    words: &[],
    deeds: &[],

    father: Some(PELEG),
    mother: None,
    teacher: None,
};

const SERUG: &Daemon = &Daemon {
    names: &[
        "Serug",
    ],
    words: &[],
    deeds: &[],

    father: Some(REU),
    mother: None,
    teacher: None,
};

const NAHOR: &Daemon = &Daemon {
    names: &[
        "Nahor",
    ],
    words: &[],
    deeds: &[],

    father: Some(SERUG),
    mother: None,
    teacher: None,
};

const TERRAH: &Daemon = &Daemon {
    names: &[
        "Terrah",
    ],
    words: &[],
    deeds: &[],

    father: Some(NAHOR),
    mother: None,
    teacher: None,
};

const ABRAHAM: &Daemon = &Daemon {
    names: &[
        "Abraham",
    ],
    words: &[],
    deeds: &[],

    father: Some(TERRAH),
    mother: None,
    teacher: None,
};

const ISAAC: &Daemon = &Daemon {
    names: &[
        "Isaac",
    ],
    words: &[],
    deeds: &[],

    father: Some(ABRAHAM),
    mother: None,
    teacher: None,
};

const JACOB: &Daemon = &Daemon {
    names: &[
        "Jacob",
    ],
    words: &[],
    deeds: &[],

    father: Some(ISAAC),
    mother: None,
    teacher: None,
};

const JUDAH_SON_OF_JACOB: &Daemon = &Daemon {
    names: &[
        "Judah",
    ],
    words: &[],
    deeds: &[],

    father: Some(JACOB),
    mother: None,
    teacher: None,
};

const PEREZ: &Daemon = &Daemon {
    names: &[
        "Perez",
    ],
    words: &[],
    deeds: &[],

    father: Some(JUDAH_SON_OF_JACOB),
    mother: None,
    teacher: None,
};

const HEZRON: &Daemon = &Daemon {
    names: &[
        "Hezron",
    ],
    words: &[],
    deeds: &[],

    father: Some(PEREZ),
    mother: None,
    teacher: None,
};

const RAM: &Daemon = &Daemon {
    names: &[
        "Ram",
    ],
    words: &[],
    deeds: &[],

    father: Some(HEZRON),
    mother: None,
    teacher: None,
};

const AMMINADAB: &Daemon = &Daemon {
    names: &[
        "Amminadab",
    ],
    words: &[],
    deeds: &[],

    father: Some(RAM),
    mother: None,
    teacher: None,
};

const NASHON: &Daemon = &Daemon {
    names: &[
        "Nashon",
    ],
    words: &[],
    deeds: &[],

    father: Some(AMMINADAB),
    mother: None,
    teacher: None,
};

const SALMON: &Daemon = &Daemon {
    names: &[
        "Salmon",
    ],
    words: &[],
    deeds: &[],

    father: Some(NASHON),
    mother: None,
    teacher: None,
};

const BOAZ: &Daemon = &Daemon {
    names: &[
        "Boaz",
    ],
    words: &[],
    deeds: &[],

    father: Some(SALMON),
    mother: None,
    teacher: None,
};

const OBED: &Daemon = &Daemon {
    names: &[
        "Obed",
    ],
    words: &[],
    deeds: &[],

    father: Some(BOAZ),
    mother: None,
    teacher: None,
};

const JESSE: &Daemon = &Daemon {
    names: &[
        "Jesse",
    ],
    words: &[],
    deeds: &[],

    father: Some(OBED),
    mother: None,
    teacher: None,
};

const DAVID: &Daemon = &Daemon {
    names: &[
        "David",
    ],
    words: &[],
    deeds: &[],

    father: Some(JESSE),
    mother: None,
    teacher: None,
};

const NATHAN: &Daemon = &Daemon {
    names: &[
        "Nathan",
    ],
    words: &[],
    deeds: &[],

    father: Some(DAVID),
    mother: None,
    teacher: None,
};

const MATTATHA: &Daemon = &Daemon {
    names: &[
        "Mattatha",
    ],
    words: &[],
    deeds: &[],

    father: Some(NATHAN),
    mother: None,
    teacher: None,
};

const MENNA: &Daemon = &Daemon {
    names: &[
        "Menna",
    ],
    words: &[],
    deeds: &[],

    father: Some(MATTATHA),
    mother: None,
    teacher: None,
};

const MELEA: &Daemon = &Daemon {
    names: &[
        "Melea",
    ],
    words: &[],
    deeds: &[],

    father: Some(MENNA),
    mother: None,
    teacher: None,
};

const ELIAKIM: &Daemon = &Daemon {
    names: &[
        "Eliakam",
    ],
    words: &[],
    deeds: &[],

    father: Some(MELEA),
    mother: None,
    teacher: None,
};

const JONAM: &Daemon = &Daemon {
    names: &[
        "Jonam",
    ],
    words: &[],
    deeds: &[],

    father: Some(ELIAKIM),
    mother: None,
    teacher: None,
};

const JOSEPH_SON_OF_JONAM: &Daemon = &Daemon {
    names: &[
        "Jonam",
    ],
    words: &[],
    deeds: &[],

    father: Some(JONAM),
    mother: None,
    teacher: None,
};

const JUDAH: &Daemon = &Daemon {
    names: &[
        "Judah",
    ],
    words: &[],
    deeds: &[],

    father: Some(JOSEPH_SON_OF_JONAM),
    mother: None,
    teacher: None,
};

const SIMEON: &Daemon = &Daemon {
    names: &[
        "Simeon",
    ],
    words: &[],
    deeds: &[],

    father: Some(JUDAH),
    mother: None,
    teacher: None,
};

const LEVI_SON_OF_SIMEON: &Daemon = &Daemon {
    names: &[
        "Levi",
    ],
    words: &[],
    deeds: &[],

    father: Some(SIMEON),
    mother: None,
    teacher: None,
};

const MATTHAT_SON_OF_LEVI: &Daemon = &Daemon {
    names: &[
        "Matthat",
    ],
    words: &[],
    deeds: &[],

    father: Some(LEVI_SON_OF_SIMEON),
    mother: None,
    teacher: None,
};

const JORIM: &Daemon = &Daemon {
    names: &[
        "Jorim",
    ],
    words: &[],
    deeds: &[],

    father: Some(MATTHAT_SON_OF_LEVI),
    mother: None,
    teacher: None,
};

const ELIEZER: &Daemon = &Daemon {
    names: &[
        "Eliezer",
    ],
    words: &[],
    deeds: &[],

    father: Some(JORIM),
    mother: None,
    teacher: None,
};

const JOSHUA: &Daemon = &Daemon {
    names: &[
        "Joshua",
    ],
    words: &[],
    deeds: &[],

    father: Some(ELIEZER),
    mother: None,
    teacher: None,
};

const ER: &Daemon = &Daemon {
    names: &[
        "Er",
    ],
    words: &[],
    deeds: &[],

    father: Some(JOSHUA),
    mother: None,
    teacher: None,
};

const ELMADAM: &Daemon = &Daemon {
    names: &[
        "Elmadam",
    ],
    words: &[],
    deeds: &[],

    father: Some(ER),
    mother: None,
    teacher: None,
};

const COSAM: &Daemon = &Daemon {
    names: &[
        "Cosam",
    ],
    words: &[],
    deeds: &[],

    father: Some(ELMADAM),
    mother: None,
    teacher: None,
};

const ADDI: &Daemon = &Daemon {
    names: &[
        "Addi",
    ],
    words: &[],
    deeds: &[],

    father: Some(COSAM),
    mother: None,
    teacher: None,
};

const MELKI_SON_OF_ADDI: &Daemon = &Daemon {
    names: &[
        "Melki",
    ],
    words: &[],
    deeds: &[],

    father: Some(ADDI),
    mother: None,
    teacher: None,
};

const NERI: &Daemon = &Daemon {
    names: &[
        "Neri",
    ],
    words: &[],
    deeds: &[],

    father: Some(MELKI_SON_OF_ADDI),
    mother: None,
    teacher: None,
};

const SHEALTIEL: &Daemon = &Daemon {
    names: &[
        "Shealtiel",
    ],
    words: &[],
    deeds: &[],

    father: Some(NERI),
    mother: None,
    teacher: None,
};

const ZERUBBABEL: &Daemon = &Daemon {
    names: &[
        "Zerubbabel",
    ],
    words: &[],
    deeds: &[],

    father: Some(SHEALTIEL),
    mother: None,
    teacher: None,
};

const RHESA: &Daemon = &Daemon {
    names: &[
        "Rhesa",
    ],
    words: &[],
    deeds: &[],

    father: Some(ZERUBBABEL),
    mother: None,
    teacher: None,
};

const JOANAN: &Daemon = &Daemon {
    names: &[
        "Joanan",
    ],
    words: &[],
    deeds: &[],

    father: Some(RHESA),
    mother: None,
    teacher: None,
};

const JODA: &Daemon = &Daemon {
    names: &[
        "Joda",
    ],
    words: &[],
    deeds: &[],

    father: Some(JOANAN),
    mother: None,
    teacher: None,
};

const JOSEK: &Daemon = &Daemon {
    names: &[
        "Josek",
    ],
    words: &[],
    deeds: &[],

    father: Some(JODA),
    mother: None,
    teacher: None,
};

const SEMEIN: &Daemon = &Daemon {
    names: &[
        "Semein",
    ],
    words: &[],
    deeds: &[],

    father: Some(JOSEK),
    mother: None,
    teacher: None,
};

const MATTATHIAS_SON_OF_SEMEIN: &Daemon = &Daemon {
    names: &[
        "Mattathias",
    ],
    words: &[],
    deeds: &[],

    father: Some(SEMEIN),
    mother: None,
    teacher: None,
};

const MAATH: &Daemon = &Daemon {
    names: &[
        "Maath",
    ],
    words: &[],
    deeds: &[],

    father: Some(MATTATHIAS_SON_OF_SEMEIN),
    mother: None,
    teacher: None,
};

const NAGGAI: &Daemon = &Daemon {
    names: &[
        "Naggai",
    ],
    words: &[],
    deeds: &[],

    father: Some(MAATH),
    mother: None,
    teacher: None,
};

const ESLI: &Daemon = &Daemon {
    names: &[
        "Esli",
    ],
    words: &[],
    deeds: &[],

    father: Some(NAGGAI),
    mother: None,
    teacher: None,
};

const NAHUM: &Daemon = &Daemon {
    names: &[
        "Nahum",
    ],
    words: &[],
    deeds: &[],

    father: Some(ESLI),
    mother: None,
    teacher: None,
};

const AMOS: &Daemon = &Daemon {
    names: &[
        "Amos",
    ],
    words: &[],
    deeds: &[],

    father: Some(NAHUM),
    mother: None,
    teacher: None,
};

const MATTATHIAS: &Daemon = &Daemon {
    names: &[
        "Mattathias",
    ],
    words: &[],
    deeds: &[],

    father: Some(AMOS),
    mother: None,
    teacher: None,
};

const JOSEPH_SON_OF_MATTATHIAS: &Daemon = &Daemon {
    names: &[
        "Joseph",
    ],
    words: &[],
    deeds: &[],

    father: Some(MATTATHIAS),
    mother: None,
    teacher: None,
};

const JANNAI: &Daemon = &Daemon {
    names: &[
        "Jannai",
    ],
    words: &[],
    deeds: &[],

    father: Some(JOSEPH_SON_OF_MATTATHIAS),
    mother: None,
    teacher: None,
};

const MELKI: &Daemon = &Daemon {
    names: &[
        "Melki",
    ],
    words: &[],
    deeds: &[],

    father: Some(JANNAI),
    mother: None,
    teacher: None,
};

const LEVI: &Daemon = &Daemon {
    names: &[
        "Levi",
    ],
    words: &[],
    deeds: &[],

    father: Some(MELKI),
    mother: None,
    teacher: None,
};

const MATTHAT: &Daemon = &Daemon {
    names: &[
        "Matthat",
    ],
    words: &[],
    deeds: &[],

    father: Some(LEVI),
    mother: None,
    teacher: None,
};

const HELI: &Daemon = &Daemon {
    names: &[
        "Heli",
    ],
    words: &[],
    deeds: &[],

    father: Some(MATTHAT),
    mother: None,
    teacher: None,
};

const JOSEPH: &Daemon = &Daemon {
    names: &[
        "Joseph",
    ],
    words: &[],
    deeds: &["divorcing Mary without disgrace"],

    father: Some(HELI),
    mother: None,
    teacher: None,
};

pub const JESUS: &Daemon = &Daemon {
    names: &[
        "Jesus",
    ],
    words: &[
        "render Caesar what is Caesar's",
    ],
    deeds: &[
        "walked on water",
    ],

    father: Some(JOSEPH),
    mother: Some(MARY),
    teacher: None,
};
