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
        TheSonnets,
        AllsWellThatEndsWell,
        TheTragedyOFAntonyAndCleopatra,
        AsYouLikeIt,
        TheComedyOfErrors,
        TheTragedyOfCoiolanus,
        Cymbeline,
        TheTragedyOfHamletPrinceOfDenmark,
        TheFirstPartOfKingHenryTheFourth,
        TheSecondPartOfKingHenryTheFourth,
        TheLifeOfKingHenryTheFifth,
        TheFirstPartOfHenryTheSixth,
        TheSecondPartOfKingHenryTheSixth,
        TheThirdPartOfKingHenryTheSixth,
        KingHenryTheEigth,
        TheLifeAndDeathOfKingJohn,
        TheTragedyOfJuliusCaesar,
        TheTragedyOfKingLear,
        LovesLaboursLost,
        TheTragedyOfMacbeth,
        MeasureForMeasure,
        TheMerchantOfVenice,
        TheMerryWivesOfWindsor,
        AMidsummerNightsDream,
        MuchAdoAboutNothing,
        TheTragedyOfOthelloTheMoorOfVenice,
        PericlesPrinceOfTyre,
        KingRichardTheSecond,
        KingRichardTheThird,
        TheTragedyOfRomeoAndJuliey,
        TheTamingOfTheShrew,
        Tempest,
        TheLifeOfTimonOfAthens,
        TheTragedyOfTitusAndronicus,
        TroilusAndCressida,
        TwelthNightOrWhatYouKill,
        TheTwoGentlemenOfVerona,
        TheTwoNobleKinsmen,
        TheWintersTale,
        ALoversComplaint,
        ThePassionatePilgrim,
        ThePhoenixAndTheTurtle,
        TheRapeOfLucrece,
        VenusAndAdonis,
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
