uniffi::setup_scaffolding!();

pub mod kjv;
pub mod source;

use crate::source::Source;

use std::sync::Arc;

use deed::Deed;

use daemon::ArcDaemon;
use daemon::BoxDaemon;
use daemon::Daemon;

// Luke 3:23-38
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

const ALMIGHTY: &Daemon<Source> = &Daemon {
    names: &[God],
    words: &[Source {
        book: name::Name::Leviticus,
        chapter: 20,
        verses: [26, 26],
    }],
    deeds: &[Deed {
        desc: "created the heavens and the earth",
        srcs: &[Source {
            book: name::Name::Genesis,
            chapter: 1,
            verses: [1, 1],
        }],
    }],

    father: None,
    mother: None,
    teacher: None,

    predecessor: None,
};

const MARY: &Daemon<Source> = &Daemon {
    names: &[Mary],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: None,
};

const ADAM: &Daemon<Source> = &Daemon {
    names: &[Adam],
    words: &[],
    deeds: &[],

    father: Some(ALMIGHTY),
    mother: None,
    teacher: None,

    predecessor: None,
};

const EVE: &Daemon<Source> = &Daemon {
    names: &[Eve],
    words: &[],
    deeds: &[],

    father: Some(ALMIGHTY),
    mother: None,
    teacher: None,

    predecessor: None,
};

const CAIN: &Daemon<Source> = &Daemon {
    names: &[Cain],
    words: &[],
    deeds: &[],

    father: Some(ADAM),
    mother: Some(EVE),
    teacher: None,

    predecessor: None,
};

const ABEL: &Daemon<Source> = &Daemon {
    names: &[Abel],
    words: &[],
    deeds: &[],

    father: Some(ADAM),
    mother: Some(EVE),
    teacher: None,

    predecessor: None,
};

const SETH: &Daemon<Source> = &Daemon {
    names: &[Seth],
    words: &[],
    deeds: &[],

    father: Some(ADAM),
    mother: None,
    teacher: None,

    predecessor: None,
};

const ENOSH: &Daemon<Source> = &Daemon {
    names: &[Enosh],
    words: &[],
    deeds: &[],

    father: Some(SETH),
    mother: None,
    teacher: None,

    predecessor: None,
};

const KENAN: &Daemon<Source> = &Daemon {
    names: &[Kenan],
    words: &[],
    deeds: &[],

    father: Some(ENOSH),
    mother: None,
    teacher: None,

    predecessor: None,
};

const MAHALALEL: &Daemon<Source> = &Daemon {
    names: &[Mahalalel],
    words: &[],
    deeds: &[],

    father: Some(KENAN),
    mother: None,
    teacher: None,

    predecessor: None,
};

const JARED: &Daemon<Source> = &Daemon {
    names: &[Jared],
    words: &[],
    deeds: &[],

    father: Some(MAHALALEL),
    mother: None,
    teacher: None,

    predecessor: None,
};

const ENOCH: &Daemon<Source> = &Daemon {
    names: &[Enoch],
    words: &[],
    deeds: &[],

    father: Some(JARED),
    mother: None,
    teacher: None,

    predecessor: None,
};

const METHUSELAH: &Daemon<Source> = &Daemon {
    names: &[Methuselah],
    words: &[],
    deeds: &[],

    father: Some(ENOCH),
    mother: None,
    teacher: None,

    predecessor: None,
};

const LAMECH: &Daemon<Source> = &Daemon {
    names: &[Lamech],
    words: &[],
    deeds: &[],

    father: Some(METHUSELAH),
    mother: None,
    teacher: None,

    predecessor: None,
};

const NOAH: &Daemon<Source> = &Daemon {
    names: &[Noah],
    words: &[],
    deeds: &[],

    father: Some(LAMECH),
    mother: None,
    teacher: None,

    predecessor: None,
};

const SHEM: &Daemon<Source> = &Daemon {
    names: &[Shem],
    words: &[],
    deeds: &[],

    father: Some(NOAH),
    mother: None,
    teacher: None,

    predecessor: None,
};

const ARPHAXAD: &Daemon<Source> = &Daemon {
    names: &[Arphaxad],
    words: &[],
    deeds: &[],

    father: Some(SHEM),
    mother: None,
    teacher: None,

    predecessor: None,
};

const CAINAN: &Daemon<Source> = &Daemon {
    names: &[Cainan],
    words: &[],
    deeds: &[],

    father: Some(ARPHAXAD),
    mother: None,
    teacher: None,

    predecessor: None,
};

const SHELAH: &Daemon<Source> = &Daemon {
    names: &[Shelah],
    words: &[],
    deeds: &[],

    father: Some(CAINAN),
    mother: None,
    teacher: None,

    predecessor: None,
};

const EBER: &Daemon<Source> = &Daemon {
    names: &[Eber],
    words: &[],
    deeds: &[],

    father: Some(SHELAH),
    mother: None,
    teacher: None,

    predecessor: None,
};

const PELEG: &Daemon<Source> = &Daemon {
    names: &[Peleg],
    words: &[],
    deeds: &[],

    father: Some(EBER),
    mother: None,
    teacher: None,

    predecessor: None,
};

const REU: &Daemon<Source> = &Daemon {
    names: &[Reu],
    words: &[],
    deeds: &[],

    father: Some(PELEG),
    mother: None,
    teacher: None,

    predecessor: None,
};

const SERUG: &Daemon<Source> = &Daemon {
    names: &[Serug],
    words: &[],
    deeds: &[],

    father: Some(REU),
    mother: None,
    teacher: None,

    predecessor: None,
};

const NAHOR: &Daemon<Source> = &Daemon {
    names: &[Nahor],
    words: &[],
    deeds: &[],

    father: Some(SERUG),
    mother: None,
    teacher: None,

    predecessor: None,
};

const TERRAH: &Daemon<Source> = &Daemon {
    names: &[Terrah],
    words: &[],
    deeds: &[],

    father: Some(NAHOR),
    mother: None,
    teacher: None,

    predecessor: None,
};

const ABRAHAM: &Daemon<Source> = &Daemon {
    names: &[Abraham],
    words: &[],
    deeds: &[],

    father: Some(TERRAH),
    mother: None,
    teacher: None,

    predecessor: None,
};

const ISAAC: &Daemon<Source> = &Daemon {
    names: &[Isaac],
    words: &[],
    deeds: &[],

    father: Some(ABRAHAM),
    mother: None,
    teacher: None,

    predecessor: None,
};

const JACOB: &Daemon<Source> = &Daemon {
    names: &[Jacob, Israel],
    words: &[],
    deeds: &[],

    father: Some(ISAAC),
    mother: None,
    teacher: None,

    predecessor: None,
};

const REUBEN: &Daemon<Source> = &Daemon {
    names: &[Reuben],
    words: &[],
    deeds: &[],

    father: Some(JACOB),
    mother: None,
    teacher: None,

    predecessor: None,
};

const AMRAM: &Daemon<Source> = &Daemon {
    names: &[Amram],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: None,
};

const JOCHEBED: &Daemon<Source> = &Daemon {
    names: &[Jochebed],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: None,
};

const MOSES: &Daemon<Source> = &Daemon {
    names: &[name::Name::מֹשֶׁה, name::Name::Moses],
    words: &[],
    deeds: &[],

    father: Some(AMRAM),
    mother: Some(JOCHEBED),
    teacher: None,

    predecessor: None,
};

const AARON: &Daemon<Source> = &Daemon {
    names: &[Aaron],
    words: &[],
    deeds: &[],

    father: Some(AMRAM),
    mother: Some(JOCHEBED),
    teacher: None,

    predecessor: None,
};

const JUDAH_SON_OF_JACOB: &Daemon<Source> = &Daemon {
    names: &[Judah],
    words: &[],
    deeds: &[],

    father: Some(JACOB),
    mother: None,
    teacher: None,

    predecessor: None,
};

const PEREZ: &Daemon<Source> = &Daemon {
    names: &[Perez],
    words: &[],
    deeds: &[],

    father: Some(JUDAH_SON_OF_JACOB),
    mother: None,
    teacher: None,

    predecessor: None,
};

const HEZRON: &Daemon<Source> = &Daemon {
    names: &[Hezron],
    words: &[],
    deeds: &[],

    father: Some(PEREZ),
    mother: None,
    teacher: None,

    predecessor: None,
};

const RAM: &Daemon<Source> = &Daemon {
    names: &[Ram],
    words: &[],
    deeds: &[],

    father: Some(HEZRON),
    mother: None,
    teacher: None,

    predecessor: None,
};

const AMMINADAB: &Daemon<Source> = &Daemon {
    names: &[Amminadab],
    words: &[],
    deeds: &[],

    father: Some(RAM),
    mother: None,
    teacher: None,

    predecessor: None,
};

const NASHON: &Daemon<Source> = &Daemon {
    names: &[Nashon],
    words: &[],
    deeds: &[],

    father: Some(AMMINADAB),
    mother: None,
    teacher: None,

    predecessor: None,
};

const SALMON: &Daemon<Source> = &Daemon {
    names: &[Salmon],
    words: &[],
    deeds: &[],

    father: Some(NASHON),
    mother: None,
    teacher: None,

    predecessor: None,
};

const BOAZ: &Daemon<Source> = &Daemon {
    names: &[Boaz],
    words: &[],
    deeds: &[],

    father: Some(SALMON),
    mother: None,
    teacher: None,

    predecessor: None,
};

const OBED: &Daemon<Source> = &Daemon {
    names: &[Obed],
    words: &[],
    deeds: &[],

    father: Some(BOAZ),
    mother: None,
    teacher: None,

    predecessor: None,
};

const JESSE: &Daemon<Source> = &Daemon {
    names: &[Jesse],
    words: &[],
    deeds: &[],

    father: Some(OBED),
    mother: None,
    teacher: None,

    predecessor: None,
};

const DAVID: &Daemon<Source> = &Daemon {
    names: &[David],
    words: &[],
    deeds: &[],

    father: Some(JESSE),
    mother: None,
    teacher: None,

    predecessor: None,
};

const NATHAN: &Daemon<Source> = &Daemon {
    names: &[Nathan],
    words: &[],
    deeds: &[],

    father: Some(DAVID),
    mother: None,
    teacher: None,

    predecessor: None,
};

const MATTATHA: &Daemon<Source> = &Daemon {
    names: &[Mattatha],
    words: &[],
    deeds: &[],

    father: Some(NATHAN),
    mother: None,
    teacher: None,

    predecessor: None,
};

const MENNA: &Daemon<Source> = &Daemon {
    names: &[Menna],
    words: &[],
    deeds: &[],

    father: Some(MATTATHA),
    mother: None,
    teacher: None,

    predecessor: None,
};

const MELEA: &Daemon<Source> = &Daemon {
    names: &[Melea],
    words: &[],
    deeds: &[],

    father: Some(MENNA),
    mother: None,
    teacher: None,

    predecessor: None,
};

const ELIAKIM: &Daemon<Source> = &Daemon {
    names: &[Eliakim],
    words: &[],
    deeds: &[],

    father: Some(MELEA),
    mother: None,
    teacher: None,

    predecessor: None,
};

const JONAM: &Daemon<Source> = &Daemon {
    names: &[Jonam],
    words: &[],
    deeds: &[],

    father: Some(ELIAKIM),
    mother: None,
    teacher: None,

    predecessor: None,
};

const JOSEPH_SON_OF_JONAM: &Daemon<Source> = &Daemon {
    names: &[Joseph],
    words: &[],
    deeds: &[],

    father: Some(JONAM),
    mother: None,
    teacher: None,

    predecessor: None,
};

const JUDAH: &Daemon<Source> = &Daemon {
    names: &[Judah],
    words: &[],
    deeds: &[],

    father: Some(JOSEPH_SON_OF_JONAM),
    mother: None,
    teacher: None,

    predecessor: None,
};

const SIMEON: &Daemon<Source> = &Daemon {
    names: &[Simeon],
    words: &[],
    deeds: &[],

    father: Some(JUDAH),
    mother: None,
    teacher: None,

    predecessor: None,
};

const LEVI_SON_OF_SIMEON: &Daemon<Source> = &Daemon {
    names: &[Levi],
    words: &[],
    deeds: &[],

    father: Some(SIMEON),
    mother: None,
    teacher: None,

    predecessor: None,
};

const MATTHAT_SON_OF_LEVI: &Daemon<Source> = &Daemon {
    names: &[Matthat],
    words: &[],
    deeds: &[],

    father: Some(LEVI_SON_OF_SIMEON),
    mother: None,
    teacher: None,

    predecessor: None,
};

const JORIM: &Daemon<Source> = &Daemon {
    names: &[Jorim],
    words: &[],
    deeds: &[],

    father: Some(MATTHAT_SON_OF_LEVI),
    mother: None,
    teacher: None,

    predecessor: None,
};

const ELIEZER: &Daemon<Source> = &Daemon {
    names: &[Eliezer],
    words: &[],
    deeds: &[],

    father: Some(JORIM),
    mother: None,
    teacher: None,

    predecessor: None,
};

const JOSHUA: &Daemon<Source> = &Daemon {
    names: &[name::Name::Joshua],
    words: &[],
    deeds: &[],

    father: Some(ELIEZER),
    mother: None,
    teacher: None,

    predecessor: None,
};

const ER: &Daemon<Source> = &Daemon {
    names: &[Er],
    words: &[],
    deeds: &[],

    father: Some(JOSHUA),
    mother: None,
    teacher: None,

    predecessor: None,
};

const ELMADAM: &Daemon<Source> = &Daemon {
    names: &[Elmadam],
    words: &[],
    deeds: &[],

    father: Some(ER),
    mother: None,
    teacher: None,

    predecessor: None,
};

const COSAM: &Daemon<Source> = &Daemon {
    names: &[Cosam],
    words: &[],
    deeds: &[],

    father: Some(ELMADAM),
    mother: None,
    teacher: None,

    predecessor: None,
};

const ADDI: &Daemon<Source> = &Daemon {
    names: &[Addi],
    words: &[],
    deeds: &[],

    father: Some(COSAM),
    mother: None,
    teacher: None,

    predecessor: None,
};

const MELKI_SON_OF_ADDI: &Daemon<Source> = &Daemon {
    names: &[Melki],
    words: &[],
    deeds: &[],

    father: Some(ADDI),
    mother: None,
    teacher: None,

    predecessor: None,
};

const NERI: &Daemon<Source> = &Daemon {
    names: &[Neri],
    words: &[],
    deeds: &[],

    father: Some(MELKI_SON_OF_ADDI),
    mother: None,
    teacher: None,

    predecessor: None,
};

const SHEALTIEL: &Daemon<Source> = &Daemon {
    names: &[Shealtiel],
    words: &[],
    deeds: &[],

    father: Some(NERI),
    mother: None,
    teacher: None,

    predecessor: None,
};

const ZERUBBABEL: &Daemon<Source> = &Daemon {
    names: &[Zerubbabel],
    words: &[],
    deeds: &[],

    father: Some(SHEALTIEL),
    mother: None,
    teacher: None,

    predecessor: None,
};

const RHESA: &Daemon<Source> = &Daemon {
    names: &[Rhesa],
    words: &[],
    deeds: &[],

    father: Some(ZERUBBABEL),
    mother: None,
    teacher: None,

    predecessor: None,
};

const JOANAN: &Daemon<Source> = &Daemon {
    names: &[Joanan],
    words: &[],
    deeds: &[],

    father: Some(RHESA),
    mother: None,
    teacher: None,

    predecessor: None,
};

const JODA: &Daemon<Source> = &Daemon {
    names: &[Joda],
    words: &[],
    deeds: &[],

    father: Some(JOANAN),
    mother: None,
    teacher: None,

    predecessor: None,
};

const JOSEK: &Daemon<Source> = &Daemon {
    names: &[Josek],
    words: &[],
    deeds: &[],

    father: Some(JODA),
    mother: None,
    teacher: None,

    predecessor: None,
};

const SEMEIN: &Daemon<Source> = &Daemon {
    names: &[Semein],
    words: &[],
    deeds: &[],

    father: Some(JOSEK),
    mother: None,
    teacher: None,

    predecessor: None,
};

const MATTATHIAS_SON_OF_SEMEIN: &Daemon<Source> = &Daemon {
    names: &[Mattathias],
    words: &[],
    deeds: &[],

    father: Some(SEMEIN),
    mother: None,
    teacher: None,

    predecessor: None,
};

const MAATH: &Daemon<Source> = &Daemon {
    names: &[Maath],
    words: &[],
    deeds: &[],

    father: Some(MATTATHIAS_SON_OF_SEMEIN),
    mother: None,
    teacher: None,

    predecessor: None,
};

const NAGGAI: &Daemon<Source> = &Daemon {
    names: &[Naggai],
    words: &[],
    deeds: &[],

    father: Some(MAATH),
    mother: None,
    teacher: None,

    predecessor: None,
};

const ESLI: &Daemon<Source> = &Daemon {
    names: &[Esli],
    words: &[],
    deeds: &[],

    father: Some(NAGGAI),
    mother: None,
    teacher: None,

    predecessor: None,
};

const NAHUM: &Daemon<Source> = &Daemon {
    names: &[name::Name::Nahum],
    words: &[],
    deeds: &[],

    father: Some(ESLI),
    mother: None,
    teacher: None,

    predecessor: None,
};

const AMOS: &Daemon<Source> = &Daemon {
    names: &[name::Name::Amos],
    words: &[],
    deeds: &[],

    father: Some(NAHUM),
    mother: None,
    teacher: None,

    predecessor: None,
};

const MATTATHIAS: &Daemon<Source> = &Daemon {
    names: &[Mattathias],
    words: &[],
    deeds: &[],

    father: Some(AMOS),
    mother: None,
    teacher: None,

    predecessor: None,
};

const JOSEPH_SON_OF_MATTATHIAS: &Daemon<Source> = &Daemon {
    names: &[Joseph],
    words: &[],
    deeds: &[],

    father: Some(MATTATHIAS),
    mother: None,
    teacher: None,

    predecessor: None,
};

const JANNAI: &Daemon<Source> = &Daemon {
    names: &[Jannai],
    words: &[],
    deeds: &[],

    father: Some(JOSEPH_SON_OF_MATTATHIAS),
    mother: None,
    teacher: None,

    predecessor: None,
};

const MELKI: &Daemon<Source> = &Daemon {
    names: &[Melki],
    words: &[],
    deeds: &[],

    father: Some(JANNAI),
    mother: None,
    teacher: None,

    predecessor: None,
};

const LEVI: &Daemon<Source> = &Daemon {
    names: &[Levi],
    words: &[],
    deeds: &[],

    father: Some(MELKI),
    mother: None,
    teacher: None,

    predecessor: None,
};

const MATTHAT: &Daemon<Source> = &Daemon {
    names: &[Matthat],
    words: &[],
    deeds: &[],

    father: Some(LEVI),
    mother: None,
    teacher: None,

    predecessor: None,
};

const HELI: &Daemon<Source> = &Daemon {
    names: &[Heli],
    words: &[],
    deeds: &[],

    father: Some(MATTHAT),
    mother: None,
    teacher: None,

    predecessor: None,
};

const JOSEPH: &Daemon<Source> = &Daemon {
    names: &[Joseph],
    words: &[],
    deeds: &[Deed {
        desc: "decided to divorce Mary secretely to avoid her disgrace",
        srcs: &[Source {
            book: name::Name::Matthew,
            chapter: 1,
            verses: [19, 19],
        }],
    }],

    father: Some(HELI),
    mother: None,
    teacher: None,

    predecessor: None,
};

pub const JESUS: &Daemon<Source> = &Daemon {
    names: &[
        name::Name::Jesus,
        name::Name::Iesus,
        name::Name::Ἰησοῦς,
        name::Name::יֵשׁוּעַ,
        name::Name::יֵשׁוּ,
    ],

    words: &[
        Source {
            book: name::Name::Matthew,
            chapter: 5,
            verses: [13, 16],
        },
        Source {
            book: name::Name::Matthew,
            chapter: 7,
            verses: [6, 7],
        },
        Source {
            book: name::Name::Matthew,
            chapter: 22,
            verses: [21, 21],
        },
        Source {
            book: name::Name::Revelation,
            chapter: 22,
            verses: [15, 16],
        },
    ],
    deeds: &[Deed {
        desc: "walked on water",
        srcs: &[Source {
            book: name::Name::Matthew,
            chapter: 14,
            verses: [25, 25],
        }],
    }],

    father: Some(JOSEPH),
    mother: Some(MARY),
    teacher: None,

    predecessor: None,
};

// Ἐγὼ Ἰησοῦς ἔπεμψα τὸν ἄγγελόν μου μαρτυρῆσαι ὑμῖν ταῦτα ἐπὶ ταῖς ἐκκλησίαις.
// ἐγώ εἰμι ἡ ῥίζα καὶ τὸ γένος Δαυείδ, ὁ ἀστὴρ ὁ λαμπρός, ὁ πρωϊνός.
#[uniffi::export]
pub fn arc_morning_star() -> ArcDaemon {
    Arc::unwrap_or_clone(JESUS.new_arc().unwrap())
}

#[uniffi::export]
pub fn box_morning_star() -> BoxDaemon {
    *JESUS.new_box().unwrap()
}
