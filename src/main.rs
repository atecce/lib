mod name;
mod deed;
mod src;
mod book;
mod daemon;
mod bible;
mod greece;
mod rome;
mod persia;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead,BufReader};
use std::io::prelude::*;
use std::num::ParseIntError;
use crate::name::Name;
use crate::bible::BOOKS;
use crate::bible::JESUS;
use crate::greece::main::APOLLO;
use crate::greece::macedon::ALEXANDER;
use crate::rome::CICERO;
use crate::persia::CYRUS;

struct BibleReader<R> {
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

impl<R: std::io::Read> Iterator for BibleReader<R> {
    type Item = (Name, usize, usize, String);

    fn next(&mut self) -> Option<Self::Item> {

        // TODO(atec): hack to avoid index error on s[0..1] at beginning
        if !self.started {
            let _ = self.r.read_until(b':', &mut self.b);
            let _ = self.r.read_until(b':', &mut self.b);
            let _ = self.r.read_until(b':', &mut self.b);
            let _ = self.r.read_until(b':', &mut self.b);
            let _ = self.r.read_until(b':', &mut self.b);
            let _ = self.r.read_until(b':', &mut self.b);
            let _ = self.r.read_until(b':', &mut self.b);
            let _ = self.r.read_until(b':', &mut self.b);
            let _ = self.r.read_until(b':', &mut self.b);
            self.started = true;
            self.b.clear();
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

                    println!("s: {}", s);
                    println!("book: {}", self.book);
                    println!("chapter: {}", self.chapter);
                    println!("verse: {}", self.verse);

                    println!("extracting verse...");
                    self.verse = extract_verse(&mut self.r, &mut s, &mut self.b);
                    if let Some(chapter_and_verse) = self.word.get_mut(&self.book) {
                        chapter_and_verse[self.chapter-1].push(s.replace("\r\n", " "));
                    }
                    println!("done extracting verse");

                    println!("{:?}", self.word);
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

fn print_optimates() {
    println!("{:?}", JESUS);
    println!("{:?}", APOLLO);
    println!("{:?}", ALEXANDER);
    println!("{:?}", CICERO);
    println!("{:?}", CYRUS);
    println!();
}

fn read_bible() -> std::io::Result<()> {

    let mut word = HashMap::<Name, Vec<Vec<String>>>::new();
    for i in 0..65 {
        word.insert(BOOKS[i], Vec::new());
    }

    let br = BibleReader{
        r: BufReader::new(File::open("./pg10.txt").expect("can't open file")),
        b: Vec::new(),
        book: BOOKS[0],
        i: 0,
        chapter: 0,
        verse: 0,
        last: 0,
        started: false,
        word: word,
    };

    let mut target_book = BOOKS[0].to_string();
    let mut target_chapter: usize = 1;
    let mut target_verse: usize = 1;

    let mut line = String::new();

    for (book, chapter, verse, text) in br {

        let mut stdin = io::stdin();
        let mut stdout = io::stdout();

        write!(stdout, "continue...").unwrap();
        stdout.flush().unwrap();

        let _ = stdin.read(&mut [0u8]).unwrap();

//        if book.to_string() == target_book && chapter == target_chapter && verse == target_verse {
//
//            println!("book: {}", book);
//            println!("chapter: {}", chapter);
//            println!("verse: {}", verse);
//
//            let stdin = io::stdin();
//            let mut stdout = io::stdout();
//
//            write!(stdout, "reading bible; set chapter and verse: ").unwrap();
//            stdout.flush().unwrap();
//
//            let mut lines = stdin.lock().lines();
//            line = lines.next().unwrap().unwrap();
//
//            let tmp1 = line.split_whitespace().collect::<Vec<_>>();
//            println!("{:?}", tmp1);
//
//            target_book = tmp1[0].to_string();
//            println!("target book: {}", target_book);
//
//            let tmp2 = tmp1[1].split(":").collect::<Vec<_>>();
//            println!("{:?}", tmp2);
//
//            target_chapter = tmp2[0].parse::<usize>().unwrap();
//            println!("target chapter: {:?}", target_chapter);
//
//            target_verse = tmp2[1].parse::<usize>().unwrap();
//            println!("target verse: {:?}", target_verse);
//        }
    }
    Ok(())
}

type Record = (String, f32, f32, f32, f32);

fn read_csv() -> Result<Vec<Record>, Box<dyn Error>> {

    let mut r = csv::Reader::from_reader(File::open("./MacroTrends_Data_Download_BRK.A.trimmed.csv").expect("can't open file"));
    let mut records = Vec::new();

    for res in r.deserialize() {
        let record: Record = res?;
        records.push(record);
    }

    Ok(records)
}

fn main() {
    println!("{:?}", read_bible());
}
