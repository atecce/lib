use crate::BOOKS;

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

pub fn read_all() -> HashMap<Name, Vec<Vec<String>>> {
    let mut word = HashMap::<Name, Vec<Vec<String>>>::new();
    for book in BOOKS {
        match book {
            name::Name::Genesis => word.insert(book, vec![Vec::new(); 50]),
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
            name::Name::Lamentations => word.insert(book, vec![Vec::new(); 48]),
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

    let r = new_reader(
        BufReader::new(&include_bytes!("../../gutenberg.org/cache/epub/10/pg10.txt")[..]),
        &mut word,
    );

    for (_, _, _, _) in r {}

    word
}

struct Reader<'a, R> {
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
    word: &'a mut HashMap<Name, Vec<Vec<String>>>,
}

fn new_reader<R: std::io::Read>(
    r: BufReader<R>,
    word: &mut HashMap<Name, Vec<Vec<String>>>,
) -> Reader<'_, R> {
    Reader {
        r: r,
        b: Vec::new(),
        book: BOOKS[0],
        i: 0,
        new_book: false,
        chapter: 1,
        last_chapter: 1,
        new_chapter: false,
        started: false,
        revelation: false,
        amen: false,
        word: word,
    }
}

impl<R: std::io::Read> Iterator for Reader<'_, R> {
    type Item = (Name, usize, usize, String);

    fn next(&mut self) -> Option<Self::Item> {
        // special case for final verse
        if self.amen {
            return None;
        }
        if self.revelation {
            self.amen = true;
            let text = "21 The grace of our Lord Jesus Christ be with you all. Amen.";
            if let Some(chapter_and_verse) = self.word.get_mut(&self.book) {
                chapter_and_verse[self.chapter - 1].push(text.to_string());
            }
            return Some((Revelation, 22, 21, text.to_string()));
        }

        // TODO(atec): hack to avoid index error on s[0..1] at beginning
        if !self.started {
            while self.r.read_until(b':', &mut self.b).is_ok() {
                let mut s = String::from_utf8_lossy(&self.b).to_string();

                if s.is_char_boundary(s.len() - 1)
                    && s.is_char_boundary(s.len() - 2)
                    && s[s.len() - 2..s.len() - 1].parse::<usize>().is_ok()
                {
                    self.started = true;
                    self.b.clear();
                    let _ = self.r.read_until(b':', &mut self.b);
                    s = String::from_utf8_lossy(&self.b).to_string();

                    let (verse, text) = self.extract_verse();
                    if let Some(chapter_and_verse) = self.word.get_mut(&self.book) {
                        chapter_and_verse[self.chapter - 1].push(s.clone());
                    }

                    return Some((self.book, self.chapter, verse, text));
                }

                self.b.clear();
            }
        }

        // TODO(atec); perhaps use returned byte number
        while self.r.read_until(b':', &mut self.b).is_ok() {
            match self.extract_chapter() {
                Ok(n) => {
                    if self.new_book {
                        self.i += 1;
                        self.book = BOOKS[self.i];
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

                    let (verse, text) = self.extract_verse();
                    if let Some(chapter_and_verse) = self.word.get_mut(&self.book) {
                        chapter_and_verse[self.chapter - 1].push(text.clone());
                    }

                    // special case for Philemon, JohnII, JohnIII, and Jude
                    // these books only have one chapter so we will just
                    // hard code the final verse
                    if (self.book == Obadiah && verse == 21)
                        || (self.book == Philemon && verse == 25)
                        || (self.book == JohnII && verse == 13)
                        || (self.book == JohnIII && verse == 14)
                        || (self.book == Jude && verse == 25)
                    {
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

impl<R> Reader<'_, R> {
    fn extract_chapter(&self) -> Result<usize, ParseIntError> {
        let s = String::from_utf8_lossy(&self.b).to_string();
        let mut chapter = 0;

        if s.is_char_boundary(s.len() - 1) && s.is_char_boundary(s.len() - 2) {
            for i in 0..3 {
                match &s[s.len() - (2 + i)..s.len() - 1].parse::<usize>() {
                    Ok(n) => {
                        chapter = *n;
                    }
                    Err(e) => {
                        if i == 0 {
                            return Err(e.clone());
                        }
                        // TODO(atec): warn about nested fallbacks
                    }
                }
            }
        } else {
            // TODO(atec): perhaps create own err type
            return "is not character boundary".parse::<usize>();
        }

        return Ok(chapter);
    }

    fn extract_verse(&mut self) -> (usize, String)
    where
        R: std::io::Read,
    {
        let mut s = String::from_utf8_lossy(&self.b).to_string();
        let mut verse = 0;

        // TODO(atec): hack at the end
        if s.len() == 0 {
            return (verse, "".to_string());
        }

        for i in 0..3 {
            match &s[0..(1 + i)].parse::<usize>() {
                Ok(n) => {
                    verse = *n;
                }
                Err(e) => {
                    if i == 0 {
                        // TODO(atec): perhaps panic
                        println!("failed conversion extracting verse: {}", e);
                        println!(
                            "should not happen because we read to the end of the verse as soon as we match"
                        );
                    } else {
                        // TODO(atec): warn about fallbacks
                    }
                }
            }
        }

        let mut next_verse = s.is_char_boundary(s.len() - 1)
            && s.is_char_boundary(s.len() - 2)
            && s[s.len() - 2..s.len() - 1].parse::<usize>().is_ok();

        while !next_verse {
            // TODO(atec); perhaps use returned byte number
            let _ = self.r.read_until(b':', &mut self.b);
            s = String::from_utf8_lossy(&self.b).to_string();

            next_verse = s.is_char_boundary(s.len() - 1)
                && s.is_char_boundary(s.len() - 2)
                && s[s.len() - 2..s.len() - 1].parse::<usize>().is_ok()
        }

        self.b.clear();

        return (verse, s.replace("\r\n", " "));
    }
}
