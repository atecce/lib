// Luke 3:23-38

#[derive(Debug)]
pub struct Daemon<'a> {
//    names: &[&str],
//    words: &[&str],
//    deeds: &[&str],

    father: Option<&'a Daemon<'a>>,
    mother: Option<&'a Daemon<'a>>,
}

const ALMIGHTY: &Daemon = &Daemon {
//    names: &[
//        "God",
//    ],
//    words: &[
//        "text is the universal interface",
//    ],
//    deeds: &[
//        "creation",
//    ],

    father: None,
    mother: None,
};

const MARY: &Daemon = &Daemon {
//    names: &str[
//        "Mary",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: None,
    mother: None,
};

const ADAM: &Daemon = &Daemon {
//    names: &str[
//        "Adam",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(ALMIGHTY),
    mother: None,
};

const SETH: &Daemon = &Daemon {
//    names: &str[
//        "Seth",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(ADAM),
    mother: None,
};

const ENOSH: &Daemon = &Daemon {
//    names: &str[
//        "Enosh",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(SETH),
    mother: None,
};

const KENAN: &Daemon = &Daemon {
//    names: &str[
//        "Kenan",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(ENOSH),
    mother: None,
};

const MAHALALEL: &Daemon = &Daemon {
//    names: &str[
//        "Mahalalel",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(KENAN),
    mother: None,
};

const JARED: &Daemon = &Daemon {
//    names: &str[
//        "Jared",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(MAHALALEL),
    mother: None,
};

const ENOCH: &Daemon = &Daemon {
//    names: &str[
//        "Enoch",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(JARED),
    mother: None,
};

const METHUSELAH: &Daemon = &Daemon {
//    names: &str[
//        "Methuselah",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(ENOCH),
    mother: None,
};

const LAMECH: &Daemon = &Daemon {
//    names: &str[
//        "Lamech",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(METHUSELAH),
    mother: None,
};

const NOAH: &Daemon = &Daemon {
//    names: &str[
//        "Noah",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(LAMECH),
    mother: None,
};

const SHEM: &Daemon = &Daemon {
//    names: &str[
//        "Shem",
//    ],
//    words: [String;0];,
//    deeds: [String;0];,

    father: Some(NOAH),
    mother: None,
};

const ARPHAXAD: &Daemon = &Daemon {
//    names: &str[
//        String::from("Arphaxad"),
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(SHEM),
    mother: None,
};

const CAINAN: &Daemon = &Daemon {
//    names: &str[
//        "Cainan",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(ARPHAXAD),
    mother: None,
};

const SHELAH: &Daemon = &Daemon {
//    names: &str[
//        "Eber",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(CAINAN),
    mother: None,
};

const EBER: &Daemon = &Daemon {
//    names: &str[
//        "Eber",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(SHELAH),
    mother: None,
};

const PELEG: &Daemon = &Daemon {
//    names: &str[
//        "Peleg",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(EBER),
    mother: None,
};

const REU: &Daemon = &Daemon {
//    names: &str[
//        "Reu",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(PELEG),
    mother: None,
};

const SERUG: &Daemon = &Daemon {
//    names: &str[
//        "Serug",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(REU),
    mother: None,
};

const NAHOR: &Daemon = &Daemon {
//    names: &str[
//        "Nahor",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(SERUG),
    mother: None,
};

const TERRAH: &Daemon = &Daemon {
//    names: &str[
//        "Terrah",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(NAHOR),
    mother: None,
};

const ABRAHAM: &Daemon = &Daemon {
//    names: &str[
//        "Abraham",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(TERRAH),
    mother: None,
};

const ISAAC: &Daemon = &Daemon {
//    names: &str[
//        "Isaac",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(ABRAHAM),
    mother: None,
};

const JACOB: &Daemon = &Daemon {
//    names: &str[
//        "Jacob",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(ISAAC),
    mother: None,
};

const JUDAH_SON_OF_JACOB: &Daemon = &Daemon {
//    names: &str[
//        "Judah",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(JACOB),
    mother: None,
};

const PEREZ: &Daemon = &Daemon {
//    names: &str[
//        "Perez",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(JUDAH_SON_OF_JACOB),
    mother: None,
};

const HEZRON: &Daemon = &Daemon {
//    names: &str[
//        "Hezron",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(PEREZ),
    mother: None,
};

const RAM: &Daemon = &Daemon {
//    names: &str[
//        "Ram",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(HEZRON),
    mother: None,
};

const AMMINADAB: &Daemon = &Daemon {
//    names: &str[
//        "Amminadab",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(RAM),
    mother: None,
};

const NASHON: &Daemon = &Daemon {
//    names: &str[
//        "Nashon",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(AMMINADAB),
    mother: None,
};

const SALMON: &Daemon = &Daemon {
//    names: &str[
//        "Salmon",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(NASHON),
    mother: None,
};

const BOAZ: &Daemon = &Daemon {
//    names: &str[
//        "Boaz",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(SALMON),
    mother: None,
};

const OBED: &Daemon = &Daemon {
//    names: &str[
//        "Obed",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(BOAZ),
    mother: None,
};

const JESSE: &Daemon = &Daemon {
//    names: &str[
//        "Jesse",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(OBED),
    mother: None,
};

const DAVID: &Daemon = &Daemon {
//    names: &str[
//        "David",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(JESSE),
    mother: None,
};

const NATHAN: &Daemon = &Daemon {
//    names: &str[
//        "Nathan",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(DAVID),
    mother: None,
};

const MATTATHA: &Daemon = &Daemon {
//    names: &str[
//        "Mattatha",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(NATHAN),
    mother: None,
};

const MENNA: &Daemon = &Daemon {
//    names: &str[
//        "Menna",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(MATTATHA),
    mother: None,
};

const MELEA: &Daemon = &Daemon {
//    names: &str[
//        "Melea",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(MENNA),
    mother: None,
};

const ELIAKIM: &Daemon = &Daemon {
//    names: &str[
//        "Eliakam",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(MELEA),
    mother: None,
};

const JONAM: &Daemon = &Daemon {
//    names: &str[
//        "Jonam",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(ELIAKIM),
    mother: None,
};

const JOSEPH_SON_OF_JONAM: &Daemon = &Daemon {
//    names: &str[
//        "Jonam",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(JONAM),
    mother: None,
};

const JUDAH: &Daemon = &Daemon {
//    names: &str[
//        "Judah",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(JOSEPH_SON_OF_JONAM),
    mother: None,
};

const SIMEON: &Daemon = &Daemon {
//    names: &str[
//        "Simeon",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(JUDAH),
    mother: None,
};

const LEVI_SON_OF_SIMEON: &Daemon = &Daemon {
//    names: &str[
//        "Levi",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(SIMEON),
    mother: None,
};

const MATTHAT_SON_OF_LEVI: &Daemon = &Daemon {
//    names: &str[
//        "Matthat",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(LEVI_SON_OF_SIMEON),
    mother: None,
};

const JORIM: &Daemon = &Daemon {
//    names: &str[
//        "Jorim",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(MATTHAT_SON_OF_LEVI),
    mother: None,
};

const ELIEZER: &Daemon = &Daemon {
//    names: &str[
//        "Eliezer",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(JORIM),
    mother: None,
};

const JOSHUA: &Daemon = &Daemon {
//    names: &str[
//        "Joshua",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(ELIEZER),
    mother: None,
};

const ER: &Daemon = &Daemon {
//    names: &str[
//        "Er",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(JOSHUA),
    mother: None,
};

const ELMADAM: &Daemon = &Daemon {
//    names: &str[
//        "Elmadam",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(ER),
    mother: None,
};

const COSAM: &Daemon = &Daemon {
//    names: &str[
//        "Cosam",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(ELMADAM),
    mother: None,
};

const ADDI: &Daemon = &Daemon {
//    names: &str[
//        "Addi",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(COSAM),
    mother: None,
};

const MELKI_SON_OF_ADDI: &Daemon = &Daemon {
//    names: &str[
//        "Melki",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(ADDI),
    mother: None,
};

const NERI: &Daemon = &Daemon {
//    names: &str[
//        "Neri",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(MELKI_SON_OF_ADDI),
    mother: None,
};

const SHEALTIEL: &Daemon = &Daemon {
//    names: &str[
//        "Shealtiel",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(NERI),
    mother: None,
};

const ZERUBBABEL: &Daemon = &Daemon {
//    names: &str[
//        "Zerubbabel",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(SHEALTIEL),
    mother: None,
};

const RHESA: &Daemon = &Daemon {
//    names: &str[
//        "Rhesa",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(ZERUBBABEL),
    mother: None,
};

const JOANAN: &Daemon = &Daemon {
//    names: &str[
//        "Joanan",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(RHESA),
    mother: None,
};

const JODA: &Daemon = &Daemon {
//    names: &str[
//        "Joda",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(JOANAN),
    mother: None,
};

const JOSEK: &Daemon = &Daemon {
//    names: &str[
//        "Josek",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(JODA),
    mother: None,
};

const SEMEIN: &Daemon = &Daemon {
//    names: &str[
//        "Semein",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(JOSEK),
    mother: None,
};

const MATTATHIAS_SON_OF_SEMEIN: &Daemon = &Daemon {
//    names: &str[
//        "Mattathias",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(SEMEIN),
    mother: None,
};

const MAATH: &Daemon = &Daemon {
//    names: &str[
//        "Maath",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(MATTATHIAS_SON_OF_SEMEIN),
    mother: None,
};

const NAGGAI: &Daemon = &Daemon {
//    names: &str[
//        "Naggai",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(MAATH),
    mother: None,
};

const ESLI: &Daemon = &Daemon {
//    names: &str[
//        "Esli",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(NAGGAI),
    mother: None,
};

const NAHUM: &Daemon = &Daemon {
//    names: &str[
//        "Nahum",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(ESLI),
    mother: None,
};

const AMOS: &Daemon = &Daemon {
//    names: &str[
//        "Amos",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(NAHUM),
    mother: None,
};

const MATTATHIAS: &Daemon = &Daemon {
//    names: &str[
//        "Mattathias",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(AMOS),
    mother: None,
};

const JOSEPH_SON_OF_MATTATHIAS: &Daemon = &Daemon {
//    names: &str[
//        "Joseph",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(MATTATHIAS),
    mother: None,
};

const JANNAI: &Daemon = &Daemon {
//    names: &str[
//        "Jannai",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(JOSEPH_SON_OF_MATTATHIAS),
    mother: None,
};

const MELKI: &Daemon = &Daemon {
//    names: &str[
//        "Melki",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(JANNAI),
    mother: None,
};

const LEVI: &Daemon = &Daemon {
//    names: &str[
//        "Levi",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(MELKI),
    mother: None,
};

const MATTHAT: &Daemon = &Daemon {
//    names: &str[
//        "Matthat",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(LEVI),
    mother: None,
};

const HELI: &Daemon = &Daemon {
//    names: &str[
//        "Heli",
//    ],
//    words: &str[],
//    deeds: &str[],

    father: Some(MATTHAT),
    mother: None,
};

const JOSEPH: &Daemon = &Daemon {
//    names: &str[
//        "Joseph",
//    ],
//    words: &str[],
//    deeds: &str["divorcing Mary without disgrace"],

    father: Some(HELI),
    mother: None,
};

pub const JESUS: &Daemon = &Daemon {
//    names: &str[
//        "Jesus",
//    ],
//    words: &str[
//        "render Caesar what is Caesar's",
//    ],
//    deeds: &str[
//        "walked on water",
//    ],

    father: Some(JOSEPH),
    mother: Some(MARY),
};
