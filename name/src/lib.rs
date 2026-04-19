// israel
// // books
// // // old
use crate::Name::Genesis;
use crate::Name::Exodus;
use crate::Name::Leviticus;
use crate::Name::Numbers;
use crate::Name::Deuteronomy;
use crate::Name::Joshua;
use crate::Name::Judges;
use crate::Name::Ruth;
use crate::Name::SamuelI;
use crate::Name::SamuelII;
use crate::Name::KingsI;
use crate::Name::KingsII;
use crate::Name::ChroniclesI;
use crate::Name::ChroniclesII;
use crate::Name::Ezra;
use crate::Name::Nehemiah;
use crate::Name::Esther;
use crate::Name::Job;
use crate::Name::Psalms;
use crate::Name::Proverbs;
use crate::Name::Ecclesiastes;
use crate::Name::SongOfSolomon;
use crate::Name::Isaiah;
use crate::Name::Jeremiah;
use crate::Name::Lamentations;
use crate::Name::Ezekiel;
use crate::Name::Daniel;
use crate::Name::Hosea;
use crate::Name::Joel;
use crate::Name::Amos;
use crate::Name::Obadiah;
use crate::Name::Jonah;
use crate::Name::Micah;
use crate::Name::Nahum;
use crate::Name::Habakkuk;
use crate::Name::Zephaniah;
use crate::Name::Haggai;
use crate::Name::Zechariah;
use crate::Name::Malachi;

// // // new
use crate::Name::Matthew;
use crate::Name::Mark;
use crate::Name::Luke;
use crate::Name::John;
use crate::Name::Acts;
use crate::Name::Romans;
use crate::Name::CorinthiansI;
use crate::Name::CorinthiansII;
use crate::Name::Galatians;
use crate::Name::Ephesians;
use crate::Name::Philippians;
use crate::Name::Colossians;
use crate::Name::ThessaloniansI;
use crate::Name::ThessaloniansII;
use crate::Name::TimothyI;
use crate::Name::TimothyII;
use crate::Name::Titus;
use crate::Name::Philemon;
use crate::Name::Hebrews;
use crate::Name::James;
use crate::Name::PeterI;
use crate::Name::PeterII;
use crate::Name::JohnI;
use crate::Name::JohnII;
use crate::Name::JohnIII;
use crate::Name::Jude;
use crate::Name::Revelation;

use serde::Serialize;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize)]
pub enum Name {
    //
    God,

    // greece
    Cronos,
    Cronus,
    Kronos,
    Zeus,
    Hera,
    Leto,
    Hermes,
    Apollo,
    Artemis,
    Achilles,
    Philip,

    // persia
    Cyrus,

    // israel
    // // books
    // // // old
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
    Lamentations,
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

    // // // new
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

    // // characters
    Yahweh,
    Mary,
    Adam,
    Eve,
    Cain,
    Abel,
    Seth,
    Enosh,
    Kenan,
    Mahalalel,
    Jared,
    Enoch,
    Methuselah,
    Lamech,
    Noah,
    Shem,
    Arphaxad,
    Cainan,
    Shelah,
    Eber,
    Peleg,
    Reu,
    Serug,
    Nahor,
    Terrah,
    Abraham,
    Isaac,
    Jacob,
    Israel,
    Reuben,
    Amram,
    Jochebed,
    Moses,
    Aaron,
    Perez,
    Hezron,
    Ram,
    Amminadab,
    Nashon,
    Salmon,
    Boaz,
    Obed,
    Jesse,
    David,
    Nathan,
    Mattatha,
    Menna,
    Melea,
    Eliakim,
    Jonam,
    Judah,
    Simeon,
    Jorim,
    Eliezer,
    Er,
    Elmadam,
    Cosam,
    Addi,
    Neri,
    Shealtiel,
    Zerubbabel,
    Rhesa,
    Joanan,
    Joda,
    Josek,
    Semein,
    Maath,
    Naggai,
    Esli,
    Mattathias,
    Jannai,
    Melki,
    Levi,
    Matthat,
    Heli,
    Joseph,

    // rome
    Mercury,
    Diana,
    Romulus,
    Remus,
    Socrates,
    Plato,
    Aristotle,
    Alexander,
    Caesar,
    Cicero,
    Jesus,
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub const BIBLE: [Name; 66] = [
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
    Lamentations,
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
