mod name;
mod deed;
mod src;
mod book;
mod daemon;
mod bible;
mod greece;
mod rome;
mod persia;

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use crate::name::Name;
use crate::bible::main::BOOKS;
use crate::bible::main::JESUS;
use crate::greece::main::APOLLO;
use crate::greece::macedon::ALEXANDER;
use crate::rome::CICERO;
use crate::persia::CYRUS;

fn print_optimates() {
    println!("{:?}", JESUS);
    println!("{:?}", APOLLO);
    println!("{:?}", ALEXANDER);
    println!("{:?}", CICERO);
    println!("{:?}", CYRUS);
    println!();
}

fn read_bible() -> std::io::Result<()> {

    let r = bible::io::new_reader(BufReader::new(File::open("./pg10.txt").expect("can't open file")));

    let mut target_book = BOOKS[0].to_string();
    let mut target_chapter: usize = 1;
    let mut target_verse: usize = 1;

    let mut line = String::new();

    for (book, chapter, verse, text) in r {

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
