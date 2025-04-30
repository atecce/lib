use std::collections::HashMap;
use std::io::{BufRead,BufReader};
use std::num::ParseIntError;
use crate::name::Name;
use crate::name::Name::Philemon;
use crate::name::Name::JohnII;
use crate::name::Name::JohnIII;
use crate::name::Name::Jude;
use crate::name::Name::Revelation;
use crate::bible::main::BOOKS;

pub struct Reader<'a, R> {
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
    word: &'a mut HashMap::<Name, Vec<Vec<String>>>,
}

pub fn new_reader<R: std::io::Read>(r: BufReader<R>,
    word: &mut HashMap::<Name, Vec<Vec<String>>>) -> Reader<R> {

    Reader{
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
                chapter_and_verse[self.chapter-1].push(text.to_string());
            }
            return Some((Revelation, 22, 21, text.to_string()));
        }

        // TODO(atec): hack to avoid index error on s[0..1] at beginning
        if !self.started {
            while self.r.read_until(b':', &mut self.b).is_ok() {

                let mut s = String::from_utf8_lossy(&self.b).to_string();

                if s.is_char_boundary(s.len()-1) && s.is_char_boundary(s.len()-2)
                    && s[s.len()-2..s.len()-1].parse::<usize>().is_ok() {

                    self.started = true;
                    self.b.clear();
                    let _ = self.r.read_until(b':', &mut self.b);
                    s = String::from_utf8_lossy(&self.b).to_string();

                    self.chapter = 1;
                    if let Some(chapter_and_verse) = self.word.get_mut(&self.book) {
                        chapter_and_verse.push(Vec::new());
                    }

                    let (verse, text) = self.extract_verse();
                    if let Some(chapter_and_verse) = self.word.get_mut(&self.book) {
                        chapter_and_verse[self.chapter-1].push(s.clone());
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
                        if let Some(chapter_and_verse) = self.word.get_mut(&self.book) {
                            chapter_and_verse.push(Vec::new());
                        }
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
                        chapter_and_verse[self.chapter-1].push(text.clone());
                    }

                    // special case for Philemon, JohnII, JohnIII, and Jude
                    // these books only have one chapter so we will just
                    // hard code the final verse
                    if (self.book == Philemon && verse == 25) || (self.book == JohnII && verse == 13) ||
                        (self.book == JohnIII && verse == 14) || (self.book == Jude && verse == 25) {

                        self.new_book = true;
                        self.new_chapter = true;
                    }

                    // special case for penultimate verse
                    if self.book == Revelation && self.chapter == 22 && verse == 20 {
                        self.revelation = true;
                    }

                    return Some((self.book, self.chapter, verse, text));
                },
                Err(_) => {
                    // TODO(atec): possibly accumulate warnings
                    continue;
                },
            }
        }
        None
    }
}

impl<R> Reader<'_, R> {
    fn extract_chapter(&self) -> Result<usize, ParseIntError> {

        let s = String::from_utf8_lossy(&self.b).to_string();
        let mut chapter = 0;

        if s.is_char_boundary(s.len()-1) && s.is_char_boundary(s.len()-2) {
            for i in 0..3 {
                match &s[s.len()-(2+i)..s.len()-1].parse::<usize>() {
                    Ok(n) => {
                        chapter = *n;
                    },
                    Err(e) => {
                        if i == 0{
                            return Err(e.clone());
                        }
                        // TODO(atec): warn about nested fallbacks
                    },
                }
            }
        } else {
            // TODO(atec): perhaps create own err type
            return "is not character boundary".parse::<usize>();
        }

        return Ok(chapter);
    }

    fn extract_verse(&mut self) -> (usize, String)
        where R: std::io::Read {

        let mut s = String::from_utf8_lossy(&self.b).to_string();
        let mut verse = 0;

        // TODO(atec): hack at the end
        if s.len() == 0 {
            return (verse, "".to_string());
        }

        for i in 0..3 {
            match &s[0..(1+i)].parse::<usize>() {
                Ok(n) => {
                    verse = *n;
                },
                Err(e) => {
                    if i == 0 {
                        // TODO(atec): perhaps panic
                        println!("failed conversion extracting verse: {}", e);
                        println!("should not happen because we read to the end of the verse as soon as we match");
                    } else {
                        // TODO(atec): warn about fallbacks
                    }
                }
            }
        }

        let mut next_verse = s.is_char_boundary(s.len()-1) &&
            s.is_char_boundary(s.len()-2) &&
            s[s.len()-2..s.len()-1].parse::<usize>().is_ok();

        while !next_verse {

            // TODO(atec); perhaps use returned byte number
            let _ = self.r.read_until(b':', &mut self.b);
            s = String::from_utf8_lossy(&self.b).to_string();

            next_verse = s.is_char_boundary(s.len()-1) &&
                s.is_char_boundary(s.len()-2) &&
                s[s.len()-2..s.len()-1].parse::<usize>().is_ok()
        }

        self.b.clear();

        return (verse, s.replace("\r\n", " "));
    }
}
