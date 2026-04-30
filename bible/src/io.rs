uniffi::setup_scaffolding!();

use gutenberg::new_reader;
use name::BIBLE;
use name::Name;
use std::collections::HashMap;
use std::io::BufReader;

#[uniffi::export]
pub fn read_all() -> HashMap<Name, Vec<Vec<String>>> {
    let mut word = HashMap::<Name, Vec<Vec<String>>>::new();
    for book in BIBLE {
        match book {
            name::Name::Genesis => word.insert(
                book,
                vec![
                    vec!["".to_string(); 31],
                    vec!["".to_string(); 25],
                    vec!["".to_string(); 24],
                    vec!["".to_string(); 26],
                    vec!["".to_string(); 32],
                    vec!["".to_string(); 22],
                    vec!["".to_string(); 24],
                    vec!["".to_string(); 22],
                    vec!["".to_string(); 29],
                    vec!["".to_string(); 32],
                    vec!["".to_string(); 32],
                    vec!["".to_string(); 20],
                    vec!["".to_string(); 18],
                    vec!["".to_string(); 24],
                    vec!["".to_string(); 21],
                    vec!["".to_string(); 16],
                    vec!["".to_string(); 27],
                    vec!["".to_string(); 33],
                    vec!["".to_string(); 38],
                    vec!["".to_string(); 18],
                    vec!["".to_string(); 34],
                    vec!["".to_string(); 24],
                    vec!["".to_string(); 20],
                    vec!["".to_string(); 67],
                    vec!["".to_string(); 34],
                    vec!["".to_string(); 35],
                    vec!["".to_string(); 46],
                    vec!["".to_string(); 22],
                    vec!["".to_string(); 35],
                    vec!["".to_string(); 43],
                    vec!["".to_string(); 55],
                    vec!["".to_string(); 32],
                    vec!["".to_string(); 20],
                    vec!["".to_string(); 31],
                    vec!["".to_string(); 29],
                    vec!["".to_string(); 43],
                    vec!["".to_string(); 36],
                    vec!["".to_string(); 30],
                    vec!["".to_string(); 23],
                    vec!["".to_string(); 23],
                    vec!["".to_string(); 57],
                    vec!["".to_string(); 38],
                    vec!["".to_string(); 34],
                    vec!["".to_string(); 34],
                    vec!["".to_string(); 28],
                    vec!["".to_string(); 34],
                    vec!["".to_string(); 31],
                    vec!["".to_string(); 22],
                    vec!["".to_string(); 33],
                    vec!["".to_string(); 26],
                ],
            ),
            name::Name::Exodus => word.insert(book, vec![Vec::new(); 40]),
            name::Name::Leviticus => word.insert(book, vec![Vec::new(); 27]),
            name::Name::Numbers => word.insert(book, vec![Vec::new(); 36]),
            name::Name::Deuteronomy => word.insert(book, vec![Vec::new(); 34]),
            name::Name::Joshua => word.insert(book, vec![Vec::new(); 24]),
            name::Name::Judges => word.insert(book, vec![Vec::new(); 21]),
            name::Name::Ruth => word.insert(book, vec![Vec::new(); 4]),
            name::Name::SamuelI => word.insert(book, vec![Vec::new(); 31]),
            name::Name::SamuelII => word.insert(book, vec![Vec::new(); 24]),
            name::Name::KingsI => word.insert(book, vec![Vec::new(); 22]),
            name::Name::KingsII => word.insert(book, vec![Vec::new(); 25]),
            name::Name::ChroniclesI => word.insert(book, vec![Vec::new(); 29]),
            name::Name::ChroniclesII => word.insert(book, vec![Vec::new(); 36]),
            name::Name::Ezra => word.insert(book, vec![Vec::new(); 10]),
            name::Name::Nehemiah => word.insert(book, vec![Vec::new(); 13]),
            name::Name::Esther => word.insert(book, vec![Vec::new(); 10]),
            name::Name::Job => word.insert(book, vec![Vec::new(); 42]),
            name::Name::Psalms => word.insert(book, vec![Vec::new(); 150]),
            name::Name::Proverbs => word.insert(book, vec![Vec::new(); 31]),
            name::Name::Ecclesiastes => word.insert(book, vec![Vec::new(); 12]),
            name::Name::SongOfSolomon => word.insert(book, vec![Vec::new(); 8]),
            name::Name::Isaiah => word.insert(book, vec![Vec::new(); 66]),
            name::Name::Jeremiah => word.insert(book, vec![Vec::new(); 52]),
            // TODO(atec): Lamentations
            name::Name::Lamentations => word.insert(book, vec![Vec::new(); 5]),
            name::Name::Ezekiel => word.insert(book, vec![Vec::new(); 48]),
            name::Name::Daniel => word.insert(book, vec![Vec::new(); 12]),
            name::Name::Hosea => word.insert(book, vec![Vec::new(); 14]),
            name::Name::Joel => word.insert(book, vec![Vec::new(); 3]),
            name::Name::Amos => word.insert(book, vec![Vec::new(); 9]),
            name::Name::Obadiah => word.insert(book, vec![Vec::new(); 1]),
            name::Name::Jonah => word.insert(book, vec![Vec::new(); 4]),
            name::Name::Micah => word.insert(book, vec![Vec::new(); 7]),
            name::Name::Nahum => word.insert(book, vec![Vec::new(); 3]),
            name::Name::Habakkuk => word.insert(book, vec![Vec::new(); 3]),
            name::Name::Zephaniah => word.insert(book, vec![Vec::new(); 3]),
            name::Name::Haggai => word.insert(book, vec![Vec::new(); 2]),
            name::Name::Zechariah => word.insert(book, vec![Vec::new(); 14]),
            name::Name::Malachi => word.insert(book, vec![Vec::new(); 4]),
            name::Name::Matthew => word.insert(book, vec![Vec::new(); 28]),
            name::Name::Mark => word.insert(book, vec![Vec::new(); 16]),
            name::Name::Luke => word.insert(book, vec![Vec::new(); 24]),
            name::Name::John => word.insert(book, vec![Vec::new(); 21]),
            name::Name::Acts => word.insert(book, vec![Vec::new(); 28]),
            name::Name::Romans => word.insert(book, vec![Vec::new(); 16]),
            name::Name::CorinthiansI => word.insert(book, vec![Vec::new(); 16]),
            name::Name::CorinthiansII => word.insert(book, vec![Vec::new(); 13]),
            name::Name::Galatians => word.insert(book, vec![Vec::new(); 6]),
            name::Name::Ephesians => word.insert(book, vec![Vec::new(); 6]),
            name::Name::Philippians => word.insert(book, vec![Vec::new(); 4]),
            name::Name::Colossians => word.insert(book, vec![Vec::new(); 4]),
            name::Name::ThessaloniansI => word.insert(book, vec![Vec::new(); 5]),
            name::Name::ThessaloniansII => word.insert(book, vec![Vec::new(); 3]),
            name::Name::TimothyI => word.insert(book, vec![Vec::new(); 6]),
            name::Name::TimothyII => word.insert(book, vec![Vec::new(); 4]),
            name::Name::Titus => word.insert(book, vec![Vec::new(); 3]),
            name::Name::Philemon => word.insert(book, vec![Vec::new(); 1]),
            name::Name::Hebrews => word.insert(book, vec![Vec::new(); 13]),
            name::Name::James => word.insert(book, vec![Vec::new(); 5]),
            name::Name::PeterI => word.insert(book, vec![Vec::new(); 5]),
            name::Name::PeterII => word.insert(book, vec![Vec::new(); 3]),
            name::Name::JohnI => word.insert(book, vec![Vec::new(); 5]),
            name::Name::JohnII => word.insert(book, vec![Vec::new(); 1]),
            name::Name::JohnIII => word.insert(book, vec![Vec::new(); 1]),
            name::Name::Jude => word.insert(book, vec![Vec::new(); 1]),
            name::Name::Revelation => word.insert(book, vec![Vec::new(); 22]),
            // TODO(atec): this should throw some kind of error
            _ => word.insert(book, Vec::new()),
        };
    }

    for (book, chapter, verse, text) in new_reader() {
        if let Some(chapter_and_verse) = word.get_mut(&book) {
            if chapter_and_verse[chapter - 1].get(verse - 1).is_none() {
                chapter_and_verse[chapter - 1].push(text);
            } else {
                chapter_and_verse[chapter - 1][verse - 1] = text;
            }
        }
    }

    word
}
