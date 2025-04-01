mod name;
mod deed;
mod src;
mod book;
mod daemon;
mod bible;
mod greece;
mod macedon;
mod rome;
mod persia;

use std::io;
use std::fs::File;
use std::io::{BufRead,BufReader};
use indexmap::IndexMap;
use crate::bible::JESUS;
use crate::greece::APOLLO;
use crate::macedon::ALEXANDER;
use crate::rome::CICERO;
use crate::persia::CYRUS;
use crate::name::Name;
use crate::name::Name::Genesis;
use crate::name::Name::Exodus;
use crate::name::Name::Leviticus;
use crate::name::Name::Numbers;
use crate::name::Name::Deuteronomy;
use crate::name::Name::Joshua;
use crate::name::Name::Judges;
use crate::name::Name::Ruth;
use crate::name::Name::Samuel;
use crate::name::Name::Kings;
use crate::name::Name::Chronicles;
use crate::name::Name::Ezra;
use crate::name::Name::Nehemiah;
use crate::name::Name::Esther;
use crate::name::Name::Job;
use crate::name::Name::Psalms;
use crate::name::Name::Proverbs;
use crate::name::Name::Ecclesiastes; 
use crate::name::Name::SongOfSolomon;
use crate::name::Name::Isaiah;
use crate::name::Name::Jeremiah;
use crate::name::Name::Ezekiel;
use crate::name::Name::Daniel;
use crate::name::Name::Hosea;
use crate::name::Name::Joel; 
use crate::name::Name::Amos;
use crate::name::Name::Obadiah;
use crate::name::Name::Jonah;
use crate::name::Name::Micah;
use crate::name::Name::Nahum;
use crate::name::Name::Habakkuk;
use crate::name::Name::Zephaniah;
use crate::name::Name::Haggai;
use crate::name::Name::Zechariah;
use crate::name::Name::Malachi;
use crate::name::Name::Matthew;
use crate::name::Name::Mark;
use crate::name::Name::Luke;
use crate::name::Name::John;
use crate::name::Name::Acts;
use crate::name::Name::Romans;
use crate::name::Name::Corinthians;
use crate::name::Name::Galatians;
use crate::name::Name::Ephesians;
use crate::name::Name::Philippians;
use crate::name::Name::Colossians;
use crate::name::Name::Thessalonians;
use crate::name::Name::Timothy;
use crate::name::Name::Titus;
use crate::name::Name::Philemon;
use crate::name::Name::Hebrews;
use crate::name::Name::James;
use crate::name::Name::Peter;
use crate::name::Name::Jude;
use crate::name::Name::Revelation;

fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>());
}

fn extract_verse(s: &mut String, b: &mut Vec<u8>, verse: &mut usize, text: &mut String) {

    if s.len() == 0 {
        return
    }
    match &s[0..1].parse::<usize>() {
        Ok(n) => {

            *text = s.clone();

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

            println!("failed conversion {}", e);

            *s = String::from_utf8_lossy(&b).to_string();
            *text = text.to_owned() + s;
        }
    }

    b.clear();
}

fn main() {
    println!("{:?}", JESUS);
    println!("{:?}", APOLLO);
    println!("{:?}", ALEXANDER);
    println!("{:?}", CICERO);
    println!("{:?}", CYRUS);
    println!();

    let mut r = BufReader::new(File::open("./pg10.txt").expect("can't open file"));

    let mut chapter: usize = 0;
    let mut verse: usize = 0;

    let mut b: Vec<u8> = Vec::new();
    let mut started = false;
    let mut text = String::new();
    
    let mut word = IndexMap::<Name, Vec<Vec<String>>>::from([
        (Genesis, Vec::new()),
        (Exodus, Vec::new()),
        (Leviticus, Vec::new()),
        (Numbers, Vec::new()),
        (Deuteronomy, Vec::new()),
        (Joshua, Vec::new()),
        (Judges, Vec::new()),
        (Ruth, Vec::new()),
        (Samuel, Vec::new()),
        (Kings, Vec::new()),
        (Chronicles, Vec::new()),
        (Ezra, Vec::new()),
        (Nehemiah, Vec::new()),
        (Esther, Vec::new()),
        (Job, Vec::new()),
        (Psalms, Vec::new()),
        (Proverbs, Vec::new()),
        (Ecclesiastes, Vec::new()),
        (SongOfSolomon, Vec::new()),
        (Isaiah, Vec::new()),
        (Jeremiah, Vec::new()),
        (Ezekiel, Vec::new()),
        (Daniel, Vec::new()),
        (Hosea, Vec::new()),
        (Joel, Vec::new()),
        (Amos, Vec::new()),
        (Obadiah, Vec::new()),
        (Jonah, Vec::new()),
        (Micah, Vec::new()),
        (Nahum, Vec::new()),
        (Habakkuk, Vec::new()),
        (Zephaniah, Vec::new()),
        (Haggai, Vec::new()),
        (Zechariah, Vec::new()),
        (Malachi, Vec::new()),

        (Matthew, Vec::new()),
        (Mark, Vec::new()),
        (Luke, Vec::new()),
        (John, Vec::new()),
        (Acts, Vec::new()),
        (Romans, Vec::new()),
        (Corinthians, Vec::new()),
        (Galatians, Vec::new()),
        (Ephesians, Vec::new()),
        (Philippians, Vec::new()),
        (Colossians, Vec::new()),
        (Thessalonians, Vec::new()),
        (Timothy, Vec::new()),
        (Titus, Vec::new()),
        (Philemon, Vec::new()),
        (Hebrews, Vec::new()),
        (James, Vec::new()),
        (Peter, Vec::new()),
        (Jude, Vec::new()),
        (Revelation, Vec::new()),
    ]);

    while r.read_until(b':', &mut b).is_ok() {

        let mut s = String::from_utf8_lossy(&b).to_string();

        if chapter > 0 {
            extract_verse(&mut s, &mut b, &mut verse, &mut text);
        }

        if s.len() == 0 {
            println!("{:?}", word);
            return
        }
        if s.is_char_boundary(s.len()-1) && s.is_char_boundary(s.len()-2) {
            // TODO(atec): some recursive bullshit
            match &s[s.len()-2..s.len()-1].parse::<usize>() {
                Ok(n) => {

                    chapter = *n;
                    word[&Genesis].push(Vec::new());

                    match &s[s.len()-3..s.len()-1].parse::<usize>() {
                        Ok(n) => {

                            chapter = *n;

                            match &s[s.len()-4..s.len()-1].parse::<usize>() {
                                Ok(n) => chapter = *n,
                                Err(_) => _ = 0,
                            }
                        },
                        Err(_) => _ = 0,
                    }

                    // TODO(atec): hack to avoid index error on s[0..1]
                    if !started {
                        started = true;
                        b.clear();
                    }
                },
                Err(_) => _ = 0,
            }
        }

        println!("chapter: {}", chapter);
        println!("verse: {}", verse);
        println!("text: {}", text);
        if chapter > 0 {
            word[&Genesis][chapter-1].push(text.clone());
        }

        println!("{:?}", word);

        let mut tmp = String::new();
        io::stdin().read_line(&mut tmp).ok().expect("failed to read line");
    }
}
