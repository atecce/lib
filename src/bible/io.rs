use std::collections::HashMap;
use std::io::{BufRead,BufReader};
use std::num::ParseIntError;
use crate::name::Name;
use crate::bible::main::BOOKS;

pub struct Reader<R> {
    r: BufReader<R>,
    b: Vec<u8>,
    book: Name,
    i: usize,
    chapter: usize,
    last: usize,
    verse: usize,
    started: bool,
    word: HashMap::<Name, Vec<Vec<String>>>,
}

pub fn new_reader<R: std::io::Read>(r: BufReader<R>) -> Reader<R> {

    let mut word = HashMap::<Name, Vec<Vec<String>>>::new();
    for i in 0..65 {
        word.insert(BOOKS[i], Vec::new());
    }

    Reader{
        r: r,
        b: Vec::new(),
        book: BOOKS[0],
        i: 0,
        chapter: 0,
        verse: 0,
        last: 0,
        started: false,
        word: word,
    }
}

impl<R: std::io::Read> Iterator for Reader<R> {
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

                    self.verse = extract_verse(&mut self.r, &mut s, &mut self.b);
                    if let Some(chapter_and_verse) = self.word.get_mut(&self.book) {
                        chapter_and_verse[self.chapter-1].push(s.replace("\r\n", " "));
                    }

                    return Some((self.book, self.chapter, self.verse, s));
                }

                self.b.clear();
            }
        }

        // TODO(atec); perhaps use returned byte number
        while self.r.read_until(b':', &mut self.b).is_ok() {

            let mut s = String::from_utf8_lossy(&self.b).to_string();

            match extract_chapter(&mut s) {
                Ok(n) => {

                    println!("extracting chapter");
                    self.last = self.chapter;
                    self.chapter = n;

                    if self.last > self.chapter {
                        self.i += 1;
                        self.book = BOOKS[self.i];
                    }

                    if let Some(chapter_and_verse) = self.word.get_mut(&self.book) {
                        chapter_and_verse.push(Vec::new());
                    }
                    println!("done extracting chapter");

                    println!("extracting verse...");
                    self.verse = extract_verse(&mut self.r, &mut s, &mut self.b);
                    if let Some(chapter_and_verse) = self.word.get_mut(&self.book) {
                        chapter_and_verse[self.chapter-1].push(s.replace("\r\n", " "));
                    }
                    println!("done extracting verse");

                    return Some((self.book, self.chapter, self.verse, s.replace("\r\n", " ")));
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

fn extract_chapter(s: &mut String) -> Result<usize, ParseIntError> {
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

fn extract_verse<R>(r: &mut BufReader<R>, s: &mut String, b: &mut Vec<u8>) -> usize
    where R: std::io::Read {

    let mut verse = 0;

    // TODO(atec): hack at the end
    if s.len() == 0 {
        return verse;
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

        *s = String::from_utf8_lossy(&b).to_string();

        println!("s: {}", s);

        next_verse = s.is_char_boundary(s.len()-1) &&
            s.is_char_boundary(s.len()-2) &&
            s[s.len()-2..s.len()-1].parse::<usize>().is_ok()
    }

    b.clear();

    return verse;
}
