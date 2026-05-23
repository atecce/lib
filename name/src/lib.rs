uniffi::setup_scaffolding!();

pub mod bible;
pub mod shakespeare;

use std::fmt;
use std::str::FromStr;

use serde::Serialize;

macro_rules! names {
    (
        pub enum $name:ident {
            $(
                $variant:ident $(=> [$($alias:literal),+])?
            ),* $(,)?
        }
    ) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, uniffi::Enum)]
        pub enum $name {
            $($variant),*
        }

        impl FromStr for $name {
            type Err = NameError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let trimmed = s.trim();
                $(
                    $(
                        $(
                            if trimmed.eq_ignore_ascii_case($alias) {
                                return Ok($name::$variant);
                            }
                        )+
                    )?

                    if trimmed.eq_ignore_ascii_case(stringify!($variant)) {
                        return Ok($name::$variant);
                    }
                )*
                Err(NameError::NameNotFound)
            }
        }
    };
}

names! {
    pub enum Name {
        //
        God,

        // greece
        Κρόνος => ["Cronos", "Cronus", "Kronos"],

        Ζεύς => ["Zeus"],

        Hera,

        Λατώ => ["Leto"],

        Ἑρμῆς => ["Hermes"],

        Ἀπόλλων => ["Apollo"],

        Ἄρτεμις => ["Artemis"],

        Achilles,

        Σωκράτης => ["Socrates"],
        Πλάτων => ["Plato"],
        Ἀριστοτέλης => ["Aristotle"],

        Φίλιππος => ["Phillip"],
        Ἀλέξανδρος => ["Alexander", "Alessandro"],

        // persia
        Cyrus,

        // israel
 מֹשֶׁה => ["Moses"],
 יֵשׁוּ  => ["יֵשׁוּעַ" ,"Ἰησοῦς", "Iesus", "Jesus"],

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
        Jupiter,
        Juno,
        Mars,
        Mercury,
        Diana,
        Romulus,
        Remus,
        Caesar,
        Cicero,

        // shakespeare
        TheSonnets => ["THE SONNETS"],
        AllsWellThatEndsWell => ["ALL’S WELL THAT ENDS WELL"],
        TheTragedyOFAntonyAndCleopatra => ["THE TRAGEDY OF ANTONY AND CLEOPATRA"],
        AsYouLikeIt => ["AS YOU LIKE IT"],
        TheComedyOfErrors => ["THE COMEDY OF ERRORS"],
        TheTragedyOfCoiolanus => ["THE TRAGEDY OF CORIOLANUS"],
        Cymbeline => ["CYMBELINE"],
        TheTragedyOfHamletPrinceOfDenmark => ["THE TRAGEDY OF HAMLET, PRINCE OF DENMARK"],
        TheFirstPartOfKingHenryTheFourth => ["THE FIRST PART OF KING HENRY THE FOURTH"],
        TheSecondPartOfKingHenryTheFourth => ["THE SECOND PART OF KING HENRY THE FOURTH"],
        TheLifeOfKingHenryTheFifth => ["THE LIFE OF KING HENRY THE FIFTH"],
        TheFirstPartOfHenryTheSixth => ["THE FIRST PART OF HENRY THE SIXTH"],
        TheSecondPartOfKingHenryTheSixth => ["THE SECOND PART OF KING HENRY THE SIXTH"],
        TheThirdPartOfKingHenryTheSixth => ["THE THIRD PARTY OF KING HENRY THE SIXTH"],
        KingHenryTheEigth => ["KING HENRY THE EIGTH"],
        TheLifeAndDeathOfKingJohn => ["THE LIFE AND DEATH OF KING JOHN"],
        TheTragedyOfJuliusCaesar => ["THE TRAGEDY OF JULIUS CAESAR"],
        TheTragedyOfKingLear => ["THE TRAGEDY OF KING LEAR"],
        LovesLaboursLost => ["LOVE’S LABOUR’S LOST"],
        TheTragedyOfMacbeth => ["THE TRAGEDY OF MACBETH"],
        MeasureForMeasure => ["MEASURE FOR MEASURE"],
        TheMerchantOfVenice => ["THE MERCHANT OF VENICE"],
        TheMerryWivesOfWindsor => ["THE MERRY WIVES OF WINDSOR"],
        AMidsummerNightsDream => ["A MIDSUMMER NIGHT’S DREAM"],
        MuchAdoAboutNothing => ["MUCH ADO ABOUT NOTHING"],
        TheTragedyOfOthelloTheMoorOfVenice => ["THE TRAGEDY OF OTHELLO, THE MOOR OF VENICE"],
        PericlesPrinceOfTyre => ["PERICLES, PRINCE OF TYRE"],
        KingRichardTheSecond => ["KING RICHARD THE SECOND"],
        KingRichardTheThird => ["KING RICHARD THE THIRD"],
        TheTragedyOfRomeoAndJuliey => ["THE TRAGEDY OF ROMEO AND JULIET"],
        TheTamingOfTheShrew => ["THE TAMING OF THE SHREW"],
        Tempest => ["TEMPEST"],
        TheLifeOfTimonOfAthens => ["THE LIFE OF TIMON OF ATHENS"],
        TheTragedyOfTitusAndronicus => ["THE TRAGEDY OF TITUS ANDRONICUS"],
        TroilusAndCressida => ["TROILUS AND CRESSIDA"],
        TwelthNightOrWhatYouKill => ["TWELTH NIGHT; OR, WHAT YOU KILL"],
        TheTwoGentlemenOfVerona => ["THE TWO GENTLEMEN OF VERONA"],
        TheTwoNobleKinsmen => ["THE TWO NOBLE KINSMEN"],
        TheWintersTale => ["THE WINTER’S TALE"],
        ALoversComplaint => ["A LOVER’S COMPLAINT"],
        ThePassionatePilgrim => ["THE PASSIONATE PILGRIM"],
        ThePhoenixAndTheTurtle => ["THE PHOENIX AND THE TURTLE"],
        TheRapeOfLucrece => ["THE RAPE OF LUCRECE"],
        VenusAndAdonis => ["VENUS AND ADONIS"],
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq, uniffi::Error)]
pub enum NameError {
    NameNotFound,
}

impl fmt::Display for NameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[uniffi::export]
pub fn parse_name(string: String) -> Result<Name, NameError> {
    string.parse::<Name>()
}
