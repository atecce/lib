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
};

const MARY: &Daemon = &Daemon {
    names: &[
        "Mary",
    ],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
};

const ADAM: &Daemon = &Daemon {
    names: &[
        "Adam",
    ],
    words: &[],
    deeds: &[],

    father: Some(ALMIGHTY),
    mother: None,
};

const SETH: &Daemon = &Daemon {
    names: &[
        "Seth",
    ],
    words: &[],
    deeds: &[],

    father: Some(ADAM),
    mother: None,
};

const ENOSH: &Daemon = &Daemon {
    names: &[
        "Enosh",
    ],
    words: &[],
    deeds: &[],

    father: Some(SETH),
    mother: None,
};

const KENAN: &Daemon = &Daemon {
    names: &[
        "Kenan",
    ],
    words: &[],
    deeds: &[],

    father: Some(ENOSH),
    mother: None,
};

const MAHALALEL: &Daemon = &Daemon {
    names: &[
        "Mahalalel",
    ],
    words: &[],
    deeds: &[],

    father: Some(KENAN),
    mother: None,
};

const JARED: &Daemon = &Daemon {
    names: &[
        "Jared",
    ],
    words: &[],
    deeds: &[],

    father: Some(MAHALALEL),
    mother: None,
};

const ENOCH: &Daemon = &Daemon {
    names: &[
        "Enoch",
    ],
    words: &[],
    deeds: &[],

    father: Some(JARED),
    mother: None,
};

const METHUSELAH: &Daemon = &Daemon {
    names: &[
        "Methuselah",
    ],
    words: &[],
    deeds: &[],

    father: Some(ENOCH),
    mother: None,
};

const LAMECH: &Daemon = &Daemon {
    names: &[
        "Lamech",
    ],
    words: &[],
    deeds: &[],

    father: Some(METHUSELAH),
    mother: None,
};

const NOAH: &Daemon = &Daemon {
    names: &[
        "Noah",
    ],
    words: &[],
    deeds: &[],

    father: Some(LAMECH),
    mother: None,
};

const SHEM: &Daemon = &Daemon {
    names: &[
        "Shem",
    ],
    words: &[],
    deeds: &[],

    father: Some(NOAH),
    mother: None,
};

const ARPHAXAD: &Daemon = &Daemon {
    names: &[
        "Arphaxad",
    ],
    words: &[],
    deeds: &[],

    father: Some(SHEM),
    mother: None,
};

const CAINAN: &Daemon = &Daemon {
    names: &[
        "Cainan",
    ],
    words: &[],
    deeds: &[],

    father: Some(ARPHAXAD),
    mother: None,
};

const SHELAH: &Daemon = &Daemon {
    names: &[
        "Eber",
    ],
    words: &[],
    deeds: &[],

    father: Some(CAINAN),
    mother: None,
};

const EBER: &Daemon = &Daemon {
    names: &[
        "Eber",
    ],
    words: &[],
    deeds: &[],

    father: Some(SHELAH),
    mother: None,
};

const PELEG: &Daemon = &Daemon {
    names: &[
        "Peleg",
    ],
    words: &[],
    deeds: &[],

    father: Some(EBER),
    mother: None,
};

const REU: &Daemon = &Daemon {
    names: &[
        "Reu",
    ],
    words: &[],
    deeds: &[],

    father: Some(PELEG),
    mother: None,
};

const SERUG: &Daemon = &Daemon {
    names: &[
        "Serug",
    ],
    words: &[],
    deeds: &[],

    father: Some(REU),
    mother: None,
};

const NAHOR: &Daemon = &Daemon {
    names: &[
        "Nahor",
    ],
    words: &[],
    deeds: &[],

    father: Some(SERUG),
    mother: None,
};

const TERRAH: &Daemon = &Daemon {
    names: &[
        "Terrah",
    ],
    words: &[],
    deeds: &[],

    father: Some(NAHOR),
    mother: None,
};

const ABRAHAM: &Daemon = &Daemon {
    names: &[
        "Abraham",
    ],
    words: &[],
    deeds: &[],

    father: Some(TERRAH),
    mother: None,
};

const ISAAC: &Daemon = &Daemon {
    names: &[
        "Isaac",
    ],
    words: &[],
    deeds: &[],

    father: Some(ABRAHAM),
    mother: None,
};

const JACOB: &Daemon = &Daemon {
    names: &[
        "Jacob",
    ],
    words: &[],
    deeds: &[],

    father: Some(ISAAC),
    mother: None,
};

const JUDAH_SON_OF_JACOB: &Daemon = &Daemon {
    names: &[
        "Judah",
    ],
    words: &[],
    deeds: &[],

    father: Some(JACOB),
    mother: None,
};

const PEREZ: &Daemon = &Daemon {
    names: &[
        "Perez",
    ],
    words: &[],
    deeds: &[],

    father: Some(JUDAH_SON_OF_JACOB),
    mother: None,
};

const HEZRON: &Daemon = &Daemon {
    names: &[
        "Hezron",
    ],
    words: &[],
    deeds: &[],

    father: Some(PEREZ),
    mother: None,
};

const RAM: &Daemon = &Daemon {
    names: &[
        "Ram",
    ],
    words: &[],
    deeds: &[],

    father: Some(HEZRON),
    mother: None,
};

const AMMINADAB: &Daemon = &Daemon {
    names: &[
        "Amminadab",
    ],
    words: &[],
    deeds: &[],

    father: Some(RAM),
    mother: None,
};

const NASHON: &Daemon = &Daemon {
    names: &[
        "Nashon",
    ],
    words: &[],
    deeds: &[],

    father: Some(AMMINADAB),
    mother: None,
};

const SALMON: &Daemon = &Daemon {
    names: &[
        "Salmon",
    ],
    words: &[],
    deeds: &[],

    father: Some(NASHON),
    mother: None,
};

const BOAZ: &Daemon = &Daemon {
    names: &[
        "Boaz",
    ],
    words: &[],
    deeds: &[],

    father: Some(SALMON),
    mother: None,
};

const OBED: &Daemon = &Daemon {
    names: &[
        "Obed",
    ],
    words: &[],
    deeds: &[],

    father: Some(BOAZ),
    mother: None,
};

const JESSE: &Daemon = &Daemon {
    names: &[
        "Jesse",
    ],
    words: &[],
    deeds: &[],

    father: Some(OBED),
    mother: None,
};

const DAVID: &Daemon = &Daemon {
    names: &[
        "David",
    ],
    words: &[],
    deeds: &[],

    father: Some(JESSE),
    mother: None,
};

const NATHAN: &Daemon = &Daemon {
    names: &[
        "Nathan",
    ],
    words: &[],
    deeds: &[],

    father: Some(DAVID),
    mother: None,
};

const MATTATHA: &Daemon = &Daemon {
    names: &[
        "Mattatha",
    ],
    words: &[],
    deeds: &[],

    father: Some(NATHAN),
    mother: None,
};

const MENNA: &Daemon = &Daemon {
    names: &[
        "Menna",
    ],
    words: &[],
    deeds: &[],

    father: Some(MATTATHA),
    mother: None,
};

const MELEA: &Daemon = &Daemon {
    names: &[
        "Melea",
    ],
    words: &[],
    deeds: &[],

    father: Some(MENNA),
    mother: None,
};

const ELIAKIM: &Daemon = &Daemon {
    names: &[
        "Eliakam",
    ],
    words: &[],
    deeds: &[],

    father: Some(MELEA),
    mother: None,
};

const JONAM: &Daemon = &Daemon {
    names: &[
        "Jonam",
    ],
    words: &[],
    deeds: &[],

    father: Some(ELIAKIM),
    mother: None,
};

const JOSEPH_SON_OF_JONAM: &Daemon = &Daemon {
    names: &[
        "Jonam",
    ],
    words: &[],
    deeds: &[],

    father: Some(JONAM),
    mother: None,
};

const JUDAH: &Daemon = &Daemon {
    names: &[
        "Judah",
    ],
    words: &[],
    deeds: &[],

    father: Some(JOSEPH_SON_OF_JONAM),
    mother: None,
};

const SIMEON: &Daemon = &Daemon {
    names: &[
        "Simeon",
    ],
    words: &[],
    deeds: &[],

    father: Some(JUDAH),
    mother: None,
};

const LEVI_SON_OF_SIMEON: &Daemon = &Daemon {
    names: &[
        "Levi",
    ],
    words: &[],
    deeds: &[],

    father: Some(SIMEON),
    mother: None,
};

const MATTHAT_SON_OF_LEVI: &Daemon = &Daemon {
    names: &[
        "Matthat",
    ],
    words: &[],
    deeds: &[],

    father: Some(LEVI_SON_OF_SIMEON),
    mother: None,
};

const JORIM: &Daemon = &Daemon {
    names: &[
        "Jorim",
    ],
    words: &[],
    deeds: &[],

    father: Some(MATTHAT_SON_OF_LEVI),
    mother: None,
};

const ELIEZER: &Daemon = &Daemon {
    names: &[
        "Eliezer",
    ],
    words: &[],
    deeds: &[],

    father: Some(JORIM),
    mother: None,
};

const JOSHUA: &Daemon = &Daemon {
    names: &[
        "Joshua",
    ],
    words: &[],
    deeds: &[],

    father: Some(ELIEZER),
    mother: None,
};

const ER: &Daemon = &Daemon {
    names: &[
        "Er",
    ],
    words: &[],
    deeds: &[],

    father: Some(JOSHUA),
    mother: None,
};

const ELMADAM: &Daemon = &Daemon {
    names: &[
        "Elmadam",
    ],
    words: &[],
    deeds: &[],

    father: Some(ER),
    mother: None,
};

const COSAM: &Daemon = &Daemon {
    names: &[
        "Cosam",
    ],
    words: &[],
    deeds: &[],

    father: Some(ELMADAM),
    mother: None,
};

const ADDI: &Daemon = &Daemon {
    names: &[
        "Addi",
    ],
    words: &[],
    deeds: &[],

    father: Some(COSAM),
    mother: None,
};

const MELKI_SON_OF_ADDI: &Daemon = &Daemon {
    names: &[
        "Melki",
    ],
    words: &[],
    deeds: &[],

    father: Some(ADDI),
    mother: None,
};

const NERI: &Daemon = &Daemon {
    names: &[
        "Neri",
    ],
    words: &[],
    deeds: &[],

    father: Some(MELKI_SON_OF_ADDI),
    mother: None,
};

const SHEALTIEL: &Daemon = &Daemon {
    names: &[
        "Shealtiel",
    ],
    words: &[],
    deeds: &[],

    father: Some(NERI),
    mother: None,
};

const ZERUBBABEL: &Daemon = &Daemon {
    names: &[
        "Zerubbabel",
    ],
    words: &[],
    deeds: &[],

    father: Some(SHEALTIEL),
    mother: None,
};

const RHESA: &Daemon = &Daemon {
    names: &[
        "Rhesa",
    ],
    words: &[],
    deeds: &[],

    father: Some(ZERUBBABEL),
    mother: None,
};

const JOANAN: &Daemon = &Daemon {
    names: &[
        "Joanan",
    ],
    words: &[],
    deeds: &[],

    father: Some(RHESA),
    mother: None,
};

const JODA: &Daemon = &Daemon {
    names: &[
        "Joda",
    ],
    words: &[],
    deeds: &[],

    father: Some(JOANAN),
    mother: None,
};

const JOSEK: &Daemon = &Daemon {
    names: &[
        "Josek",
    ],
    words: &[],
    deeds: &[],

    father: Some(JODA),
    mother: None,
};

const SEMEIN: &Daemon = &Daemon {
    names: &[
        "Semein",
    ],
    words: &[],
    deeds: &[],

    father: Some(JOSEK),
    mother: None,
};

const MATTATHIAS_SON_OF_SEMEIN: &Daemon = &Daemon {
    names: &[
        "Mattathias",
    ],
    words: &[],
    deeds: &[],

    father: Some(SEMEIN),
    mother: None,
};

const MAATH: &Daemon = &Daemon {
    names: &[
        "Maath",
    ],
    words: &[],
    deeds: &[],

    father: Some(MATTATHIAS_SON_OF_SEMEIN),
    mother: None,
};

const NAGGAI: &Daemon = &Daemon {
    names: &[
        "Naggai",
    ],
    words: &[],
    deeds: &[],

    father: Some(MAATH),
    mother: None,
};

const ESLI: &Daemon = &Daemon {
    names: &[
        "Esli",
    ],
    words: &[],
    deeds: &[],

    father: Some(NAGGAI),
    mother: None,
};

const NAHUM: &Daemon = &Daemon {
    names: &[
        "Nahum",
    ],
    words: &[],
    deeds: &[],

    father: Some(ESLI),
    mother: None,
};

const AMOS: &Daemon = &Daemon {
    names: &[
        "Amos",
    ],
    words: &[],
    deeds: &[],

    father: Some(NAHUM),
    mother: None,
};

const MATTATHIAS: &Daemon = &Daemon {
    names: &[
        "Mattathias",
    ],
    words: &[],
    deeds: &[],

    father: Some(AMOS),
    mother: None,
};

const JOSEPH_SON_OF_MATTATHIAS: &Daemon = &Daemon {
    names: &[
        "Joseph",
    ],
    words: &[],
    deeds: &[],

    father: Some(MATTATHIAS),
    mother: None,
};

const JANNAI: &Daemon = &Daemon {
    names: &[
        "Jannai",
    ],
    words: &[],
    deeds: &[],

    father: Some(JOSEPH_SON_OF_MATTATHIAS),
    mother: None,
};

const MELKI: &Daemon = &Daemon {
    names: &[
        "Melki",
    ],
    words: &[],
    deeds: &[],

    father: Some(JANNAI),
    mother: None,
};

const LEVI: &Daemon = &Daemon {
    names: &[
        "Levi",
    ],
    words: &[],
    deeds: &[],

    father: Some(MELKI),
    mother: None,
};

const MATTHAT: &Daemon = &Daemon {
    names: &[
        "Matthat",
    ],
    words: &[],
    deeds: &[],

    father: Some(LEVI),
    mother: None,
};

const HELI: &Daemon = &Daemon {
    names: &[
        "Heli",
    ],
    words: &[],
    deeds: &[],

    father: Some(MATTHAT),
    mother: None,
};

const JOSEPH: &Daemon = &Daemon {
    names: &[
        "Joseph",
    ],
    words: &[],
    deeds: &["divorcing Mary without disgrace"],

    father: Some(HELI),
    mother: None,
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
};
