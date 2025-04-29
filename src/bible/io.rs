use std::collections::HashMap;
use std::io::{BufRead,BufReader};
use std::num::ParseIntError;
use crate::name::Name;
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
        word: word,
    }
}

impl<R: std::io::Read> Iterator for Reader<'_, R> {
    type Item = (Name, usize, usize, String);

    fn next(&mut self) -> Option<Self::Item> {

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

                    let (verse, text) = extract_verse(&mut self.r, &mut self.b);
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

            match extract_chapter(&self.b) {
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

                    let (verse, text) = extract_verse(&mut self.r, &mut self.b);
                    if let Some(chapter_and_verse) = self.word.get_mut(&self.book) {
                        chapter_and_verse[self.chapter-1].push(text.clone());
                    }

                    return Some((self.book, self.chapter, verse, text));
                },
                Err(e) => {
                    println!("failed to extract chapter: {}", e);
                    continue;
                },
            }
        }
        None
    }
}

fn extract_chapter(b: &Vec<u8>) -> Result<usize, ParseIntError> {

    let s = String::from_utf8_lossy(&b).to_string();

    if s.is_char_boundary(s.len()-1) && s.is_char_boundary(s.len()-2) {
        // TODO(atec): some recursive bullshit
        match &s[s.len()-2..s.len()-1].parse::<usize>() {
            Ok(n1) => {
                match &s[s.len()-3..s.len()-1].parse::<usize>() {
                    Ok(n2) => {
                        match &s[s.len()-4..s.len()-1].parse::<usize>() {
                            Ok(n3) => {
                                return Ok(*n3);
                            },
                            Err(e) => {
                                println!("failed conversion parsing three digit chapter: {}", e);
                                println!("falling back to two digits");
                                return Ok(*n2);
                            },
                        }
                    },
                    Err(e) => {
                        println!("failed conversion parsing two digit chapter: {}", e);
                        println!("falling back to one digit");
                        return Ok(*n1);
                    },
                }
            },
            Err(e) => return Err(e.clone()),
        }
    } else {
        return "is not character boundary".parse::<usize>();
    }
}

fn extract_verse<R>(r: &mut BufReader<R>, b: &mut Vec<u8>) -> (usize, String)
    where R: std::io::Read {

    let mut s = String::from_utf8_lossy(&b).to_string();
    let mut verse = 0;

    // TODO(atec): hack at the end
    if s.len() == 0 {
        return (verse, "".to_string());
    }

    match &s[0..1].parse::<usize>() {
        Ok(n) => {

            println!("matched verse");

            verse = *n;

            match &s[0..2].parse::<usize>() {
                Ok(n) => {
                    verse = *n;
                    match &s[0..3].parse::<usize>() {
                        Ok(n) => verse = *n,
                        Err(_) => _ = 0,
                    }
                },
                Err(_) => _ = 0,
            }
        },
        Err(e) => {
            println!("failed conversion extracting verse: {}", e);
            println!("should not happen because we read to the end of the verse as soon as we match");
        }
    }

    let mut next_verse = s.is_char_boundary(s.len()-1) &&
        s.is_char_boundary(s.len()-2) &&
        s[s.len()-2..s.len()-1].parse::<usize>().is_ok();

    println!("reading until end of verse");
    while !next_verse {

        // TODO(atec); perhaps used returned byte number
        let _ = r.read_until(b':', b);

        s = String::from_utf8_lossy(&b).to_string();

        println!("s: {}", s);

        next_verse = s.is_char_boundary(s.len()-1) &&
            s.is_char_boundary(s.len()-2) &&
            s[s.len()-2..s.len()-1].parse::<usize>().is_ok()
    }

    b.clear();

    return (verse, s.replace("\r\n", " "));
}
