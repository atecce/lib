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
use std::io::{BufRead,BufReader,BufWriter};
use std::io::prelude::*;
use std::num::ParseIntError;
use crate::name::Name;
use crate::bible::BOOKS;
use crate::bible::JESUS;
use crate::greece::main::APOLLO;
use crate::greece::macedon::ALEXANDER;
use crate::rome::CICERO;
use crate::persia::CYRUS;

fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>());
}

fn push_chapter(word: &mut HashMap<&Name, Vec<Vec<String>>>,
    book: &Name, chapter: &mut usize, last: &mut usize) {

    if let Some(chapter_and_verse) = word.get_mut(book) {
        if chapter != last {
            chapter_and_verse.push(Vec::new());
        }
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

fn extract_verse(word: &mut HashMap<&Name, Vec<Vec<String>>>, s: &mut String,
    b: &mut Vec<u8>, book: &Name, chapter: &mut usize, verse: &mut usize, text: &mut String) {

    // TODO(atec): hack at the end
    if s.len() == 0 {
        return
    }

    match &s[0..1].parse::<usize>() {
        Ok(n) => {

            *text = s.clone();

            if let Some(chapter_and_verse) = word.get_mut(book) {
                chapter_and_verse[*chapter-1].push(text.replace("\r\n", " "));
            }

            *verse = *n;

            match &s[0..2].parse::<usize>() {
                Ok(n) => {
                    *verse = *n;
                    match &s[0..3].parse::<usize>() {
                        Ok(n) => *verse = *n,
                        Err(_) => _ = 0,
                    }
                },
                Err(_) => _ = 0,
            }
        },
        Err(e) => {

            println!("failed conversion extracting verse: {}", e);
            println!("appending text (probably a colon inside the verse)");

            *s = String::from_utf8_lossy(&b).to_string();
            *text = text.to_owned() + s;

            if let Some(chapter_and_verse) = word.get_mut(book) {
                chapter_and_verse[*chapter-1][*verse-1] = text.replace("\r\n", " ");
            }
        }
    }

    b.clear();
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

    let mut r = BufReader::new(File::open("./pg10.txt").expect("can't open file"));

    let mut i: usize = 0;
    let mut book = BOOKS[i];

    let mut chapter: usize = 0;
    let mut verse: usize = 0;
    let mut last: usize = 0;

    let mut target_book = BOOKS[i].to_string();
    let mut target_chapter: usize = 0;
    let mut target_verse: usize = 0;
    let mut next: bool = false;

    let mut b: Vec<u8> = Vec::new();
    let mut started = false;
    let mut text = String::new();

    let mut word = HashMap::<&Name, Vec<Vec<String>>>::new();
    for i in 1..65 {
        word.insert(BOOKS[i], Vec::new());
    }

    while r.read_until(b':', &mut b).is_ok() {

        let mut line = String::new();

        if (book.to_string() == target_book && chapter == target_chapter && verse == target_verse) || next {

            println!("book: {}", book);
            println!("chapter: {}", chapter);
            println!("verse: {}", verse);

            if chapter != 0 && verse != 0 {
                println!("{}", word[book][chapter-1][verse-1]);
            }

            let stdin = io::stdin();
            let mut stdout = io::stdout();

            write!(stdout, "reading bible; set chapter and verse: ").unwrap();
            stdout.flush().unwrap();

            let mut lines = stdin.lock().lines();
            line = lines.next().unwrap().unwrap();

            let tmp1 = line.split_whitespace().collect::<Vec<_>>();
            println!("{:?}", tmp1);

            if tmp1[0] == "next" {
                next = true;
            } else {
                next = false;

                target_book = tmp1[0].to_string();
                println!("target book: {}", target_book);

                let tmp2 = tmp1[1].split(":").collect::<Vec<_>>();
                println!("{:?}", tmp2);

                target_chapter = tmp2[0].parse::<usize>().unwrap();
                println!("target chapter: {:?}", target_chapter);

                target_verse = tmp2[1].parse::<usize>().unwrap();
                println!("target verse: {:?}", target_verse);
            }
        }

        let mut s = String::from_utf8_lossy(&b).to_string();

        if chapter > 0 {
            extract_verse(&mut word, &mut s, &mut b, book, &mut chapter, &mut verse, &mut text);
        }

        // TODO(atec): hack at the end
        if s.len() == 0 {
            let mut w = BufWriter::new(File::create("word.json")?);
            serde_json::to_writer(&mut w, &word)?;
            println!("{:?}", w.flush());
            return Ok(());
        }

        match extract_chapter(&mut s) {
            Ok(n) => {
                last = chapter;
                chapter = n;

                if last > chapter {
                    i += 1;
                    book = BOOKS[i];
                }

                push_chapter(&mut word, book, &mut chapter, &mut last);

                // TODO(atec): hack to avoid index error on s[0..1]
                if !started {
                    started = true;
                    b.clear();
                }
            },
            Err(e) => {
                println!("failed to push chapter: {}", e);
            },
        }
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
