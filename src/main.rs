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

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead,BufReader};
use crate::name::Name;
use crate::name::Name::Genesis;
use crate::name::Name::Exodus;
use crate::name::Name::Leviticus;
use crate::name::Name::Numbers;
use crate::name::Name::Deuteronomy;
use crate::name::Name::Joshua;
use crate::name::Name::Judges;
use crate::name::Name::Ruth;
use crate::name::Name::SamuelI;
use crate::name::Name::SamuelII;
use crate::name::Name::KingsI;
use crate::name::Name::KingsII;
use crate::name::Name::ChroniclesI;
use crate::name::Name::ChroniclesII;
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
use crate::name::Name::CorinthiansI;
use crate::name::Name::CorinthiansII;
use crate::name::Name::Galatians;
use crate::name::Name::Ephesians;
use crate::name::Name::Philippians;
use crate::name::Name::Colossians;
use crate::name::Name::ThessaloniansI;
use crate::name::Name::ThessaloniansII;
use crate::name::Name::TimothyI;
use crate::name::Name::TimothyII;
use crate::name::Name::Titus;
use crate::name::Name::Philemon;
use crate::name::Name::Hebrews;
use crate::name::Name::James;
use crate::name::Name::PeterI;
use crate::name::Name::PeterII;
use crate::name::Name::JohnI;
use crate::name::Name::JohnII;
use crate::name::Name::JohnIII;
use crate::name::Name::Jude;
use crate::name::Name::Revelation;
use crate::bible::JESUS;
use crate::greece::APOLLO;
use crate::macedon::ALEXANDER;
use crate::rome::CICERO;
use crate::persia::CYRUS;

fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>());
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
                chapter_and_verse[*chapter-1].push(text.clone());
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
                chapter_and_verse[*chapter-1][*verse-1] = text.clone();
            }
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

    let books = [
        &Genesis,
        &Exodus,
        &Leviticus,
        &Numbers,
        &Deuteronomy,
        &Joshua,
        &Judges,
        &Ruth,
        &SamuelI,
        &SamuelII,
        &KingsI,
        &KingsII,
        &ChroniclesI,
        &ChroniclesII,
        &Ezra,
        &Nehemiah,
        &Esther,
        &Job,
        &Psalms,
        &Proverbs,
        &Ecclesiastes,
        &SongOfSolomon,
        &Isaiah,
        &Jeremiah,
        &Ezekiel,
        &Daniel,
        &Hosea,
        &Joel,
        &Amos,
        &Obadiah,
        &Jonah,
        &Micah,
        &Nahum,
        &Habakkuk,
        &Zephaniah,
        &Haggai,
        &Zechariah,
        &Malachi,

        &Matthew,
        &Mark,
        &Luke,
        &John,
        &Acts,
        &Romans,
        &CorinthiansI,
        &CorinthiansII,
        &Galatians,
        &Ephesians,
        &Philippians,
        &Colossians,
        &ThessaloniansI,
        &ThessaloniansII,
        &TimothyI,
        &TimothyII,
        &Titus,
        &Philemon,
        &Hebrews,
        &James,
        &PeterI,
        &PeterII,
        &JohnI,
        &JohnII,
        &JohnIII,
        &Jude,
        &Revelation,
    ];

    let mut i: usize = 0;
    let mut book = books[i];

    let mut chapter: usize = 0;
    let mut verse: usize = 0;
    let mut last: usize = 0;

    let mut b: Vec<u8> = Vec::new();
    let mut started = false;
    let mut text = String::new();
    
    let mut word = HashMap::<&Name, Vec<Vec<String>>>::from([
        (books[0], Vec::new()),
        (books[1], Vec::new()),
        (books[2], Vec::new()),
        (books[3], Vec::new()),
        (books[4], Vec::new()),
        (books[5], Vec::new()),
        (books[6], Vec::new()),
        (books[7], Vec::new()),
        (books[8], Vec::new()),
        (books[9], Vec::new()),
        (books[10], Vec::new()),
        (books[11], Vec::new()),
        (books[12], Vec::new()),
        (books[13], Vec::new()),
        (books[14], Vec::new()),
        (books[15], Vec::new()),
        (books[16], Vec::new()),
        (books[17], Vec::new()),
        (books[18], Vec::new()),
        (books[19], Vec::new()),
        (books[20], Vec::new()),
        (books[21], Vec::new()),
        (books[22], Vec::new()),
        (books[23], Vec::new()),
        (books[24], Vec::new()),
        (books[25], Vec::new()),
        (books[26], Vec::new()),
        (books[27], Vec::new()),
        (books[28], Vec::new()),
        (books[29], Vec::new()),
        (books[30], Vec::new()),
        (books[31], Vec::new()),
        (books[32], Vec::new()),
        (books[33], Vec::new()),
        (books[34], Vec::new()),
        (books[35], Vec::new()),
        (books[36], Vec::new()),
        (books[37], Vec::new()),

        (books[38], Vec::new()),
        (books[39], Vec::new()),
        (books[40], Vec::new()),
        (books[41], Vec::new()),
        (books[42], Vec::new()),
        (books[43], Vec::new()),
        (books[44], Vec::new()),
        (books[45], Vec::new()),
        (books[46], Vec::new()),
        (books[47], Vec::new()),
        (books[48], Vec::new()),
        (books[49], Vec::new()),
        (books[50], Vec::new()),
        (books[51], Vec::new()),
        (books[52], Vec::new()),
        (books[53], Vec::new()),
        (books[54], Vec::new()),
        (books[55], Vec::new()),
        (books[56], Vec::new()),
        (books[57], Vec::new()),
        (books[58], Vec::new()),
        (books[59], Vec::new()),
        (books[60], Vec::new()),
        (books[61], Vec::new()),
        (books[62], Vec::new()),
        (books[63], Vec::new()),
        (books[64], Vec::new()),
    ]);

    while r.read_until(b':', &mut b).is_ok() {

        let mut s = String::from_utf8_lossy(&b).to_string();

        if chapter > 0 {
            extract_verse(&mut word, &mut s, &mut b, book, &mut chapter, &mut verse, &mut text);
        }

        // TODO(atec): hack at the end
        if s.len() == 0 {
            println!("{:?}", word);
            return
        }

        if s.is_char_boundary(s.len()-1) && s.is_char_boundary(s.len()-2) {

            // TODO(atec): some recursive bullshit
            match &s[s.len()-2..s.len()-1].parse::<usize>() {
                Ok(n1) => {

                    last = chapter;

                    match &s[s.len()-3..s.len()-1].parse::<usize>() {
                        Ok(n2) => {

                            match &s[s.len()-4..s.len()-1].parse::<usize>() {
                                Ok(n3) => {

                                    chapter = *n3;

                                    if let Some(chapter_and_verse) = word.get_mut(book) {
                                        if chapter != last {
                                            chapter_and_verse.push(Vec::new());
                                        }
                                    }
                                },
                                Err(e) => {
                                    println!("failed conversion parsing three digit chapter: {}", e);
                                    println!("falling back to two digits");
                                    chapter = *n2;

                                    if let Some(chapter_and_verse) = word.get_mut(book) {
                                        if chapter != last {
                                            chapter_and_verse.push(Vec::new());
                                        }
                                    }
                                },
                            }
                        },
                        Err(e) => {
                            println!("failed conversion parsing two digit chapter: {}", e);
                            println!("falling back to one digit");
                            chapter = *n1;

                            if let Some(chapter_and_verse) = word.get_mut(book) {
                                if chapter != last {
                                    chapter_and_verse.push(Vec::new());
                                }
                            }
                        },
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

        if last > chapter {
            i += 1;
            book = books[i];
            if let Some(chapter_and_verse) = word.get_mut(book) {
                if chapter != last {
                    chapter_and_verse.push(Vec::new());
                }
            }
        }
    }
}
