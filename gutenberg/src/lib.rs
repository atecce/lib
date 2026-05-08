use name::BIBLE;
use name::Name;
use name::Name::JohnII;
use name::Name::JohnIII;
use name::Name::Jude;
use name::Name::Obadiah;
use name::Name::Philemon;
use name::Name::Revelation;

use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;

pub const DOMAIN: &str = "https://gutenberg.org";

pub struct Reader<R> {
    r: BufReader<R>,
    b: Vec<u8>,
    book: Name,
    i: usize,
    new_book: bool,
    chapter: usize,
    last_chapter: usize,
    new_chapter: bool,
    started: bool,
    revelation: bool,
    amen: bool,
}

pub fn new_reader() -> Reader<&'static [u8]> {
    Reader {
        r: BufReader::new(&include_bytes!("../../gutenberg/cache/epub/10/pg10.txt")[..]),
        b: [0u8; 4096].to_vec(),
        book: BIBLE[0],
        i: 0,
        new_book: false,
        chapter: 1,
        last_chapter: 1,
        new_chapter: false,
        started: false,
        revelation: false,
        amen: false,
    }
}

impl<R: std::io::Read> Reader<R> {
    fn cur_str(&self) -> String {
        String::from_utf8_lossy(&self.b).to_string()
    }
    fn next_str(&mut self) -> Option<String> {
        match self.r.read_until(b':', &mut self.b) {
            Ok(0) => None,
            Ok(_) => Some(self.cur_str()),
            // TODO(atec): log or even panic on error
            Err(_) => None,
        }
    }
    fn clear(&mut self) {
        self.b.clear()
    }
    fn read_until_started(&mut self) {
        while let Some(s) = self.next_str() {
            if is_delimited_string(s) {

                self.started = true;
                self.clear();

                // TODO(atec): s unused here. maybe we can just do this with cur_str
                _ = self.next_str().unwrap();
                return
            }

            self.clear();
        }
    }
    fn read_until_next_verse(&mut self) -> String {
        let mut s = self.cur_str();

        while !is_delimited_string(s.clone()) {
            s = self.next_str().unwrap();
        }

        self.clear();

        return s;
    }
}

impl<R: std::io::Read> Iterator for Reader<R> {
    type Item = (Name, usize, usize, String);

    fn next(&mut self) -> Option<Self::Item> {
        // special case for final verse
        if self.amen {
            return None;
        }
        if self.revelation {
            self.amen = true;
            let text = "21 The grace of our Lord Jesus Christ be with you all. Amen.";
            return Some((Revelation, 22, 21, text.to_string()));
        }

        // TODO(atec): flushes the boilerplate at the start of pg10
        //             maybe some kind of "parent" struct or trait which
        //             handles this for gutenberg's entire corpora
        if !self.started {
            self.read_until_started();
            let (verse, text) = extract_verse(self.read_until_next_verse());
            return Some((self.book, self.chapter, verse, text));
        }

        while let Some(s) = self.next_str() {
            match extract_chapter(s) {
                Ok(n) => {
                    if self.new_book {
                        self.i += 1;
                        self.book = BIBLE[self.i];
                        self.new_book = false;
                    }
                    if self.new_chapter {
                        self.chapter = n;
                        self.new_chapter = false;
                    }
                    if self.last_chapter != n {
                        if self.last_chapter > n {
                            self.new_book = true;
                        }
                        self.new_chapter = true;
                        self.last_chapter = n;
                    }

                    let (verse, text) = extract_verse(self.read_until_next_verse());

                    // special case for Philemon, JohnII, JohnIII, and Jude
                    // these books only have one chapter so we will just
                    // hard code the final verse
                    if match self.book {
                        Obadiah => verse == 21,
                        Philemon => verse == 25,
                        JohnII => verse == 13,
                        JohnIII => verse == 14,
                        Jude => verse == 25,
                        _ => false,
                    } {
                        self.new_book = true;
                        self.new_chapter = true;
                    }

                    // special case for penultimate verse
                    if self.book == Revelation && self.chapter == 22 && verse == 20 {
                        self.revelation = true;
                    }

                    return Some((self.book, self.chapter, verse, text));
                }
                Err(_) => {
                    // TODO(atec): possibly accumulate warnings
                    continue;
                }
            }
        }
        None
    }
}

fn extract_verse(s: String) -> (usize, String) {
    return (
        s.chars()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<usize>()
            .unwrap(),
        s.replace("\r\n", " ")
            .trim_start_matches(|c: char| c.is_ascii_digit())
            .trim_start()
            .trim_end_matches(':')
            .trim_end_matches(|c: char| c.is_ascii_digit())
            .trim_end()
            .to_string(),
    );
}

fn extract_chapter(s: String) -> Result<usize, ParseIntError> {
    s.trim_end_matches(':')
        .chars()
        .rev()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<usize>()
}

fn is_delimited_string(s: String) -> bool {
    s.trim_end_matches(':').ends_with(|c: char| c.is_ascii_digit())
}

pub fn read_all() -> HashMap<Name, Vec<Vec<String>>> {
    let mut word = HashMap::<Name, Vec<Vec<String>>>::new();
    for book in name::BIBLE {
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
