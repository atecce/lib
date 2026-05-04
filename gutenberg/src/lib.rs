use name::BIBLE;
use name::Name;
use name::Name::JohnII;
use name::Name::JohnIII;
use name::Name::Jude;
use name::Name::Obadiah;
use name::Name::Philemon;
use name::Name::Revelation;

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
        b: Vec::new(),
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

impl<R> Reader<R> {
    fn clear(&mut self) {
        self.b.clear()
    }
    fn cur_str(&self) -> String {
        String::from_utf8_lossy(&self.b).to_string()
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
            while self.r.read_until(b':', &mut self.b).is_ok() {
                let mut s = self.cur_str();
                if s.trim_end_matches(':')
                    .ends_with(|c: char| c.is_ascii_digit())
                {
                    self.started = true;
                    self.clear();
                    let _ = self.r.read_until(b':', &mut self.b);
                    s = self.cur_str();

                    let (verse, text) = self.extract_verse();
                    return Some((self.book, self.chapter, verse, text));
                }

                self.clear();
            }
        }

        // TODO(atec); perhaps use returned byte number
        while self.r.read_until(b':', &mut self.b).is_ok() {
            match extract_chapter(self.cur_str()) {
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

                    let (verse, text) = self.extract_verse();

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

impl<R> Reader<R> {
    fn extract_verse(&mut self) -> (usize, String)
    where
        R: std::io::Read,
    {
        let mut s = self.cur_str();
        let mut verse = 0;

        // TODO(atec): hack at the end
        if s.len() == 0 {
            return (verse, "".to_string());
        }

        let verse = s
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<usize>()
            .unwrap_or(0);

        while !s
            .trim_end_matches(':')
            .ends_with(|c: char| c.is_ascii_digit())
        {
            // TODO(atec); perhaps use returned byte number
            let _ = self.r.read_until(b':', &mut self.b);
            s = self.cur_str();
        }

        self.clear();

        return (
            verse,
            s.replace("\r\n", " ")
                .trim_start_matches(|c: char| c.is_ascii_digit())
                .trim_start()
                .trim_end_matches(':')
                .trim_end_matches(|c: char| c.is_ascii_digit())
                .trim_end()
                .to_string(),
        );
    }
}

fn extract_chapter(str: String) -> Result<usize, ParseIntError> {
    str.trim_end_matches(':')
        .chars()
        .rev()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<usize>()
}
