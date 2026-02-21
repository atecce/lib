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
use std::collections::HashMap;
use std::io::BufReader;
use crate::name::Name;
use crate::bible::main::BOOKS;
use crate::bible::main::JESUS;
use crate::greece::main::APOLLO;
use crate::greece::macedon::ALEXANDER;
use crate::rome::CICERO;
use crate::persia::CYRUS;

use orp::params::RuntimeParameters;
use gliner::util::result::Result;
use gliner::model::{GLiNER, input::text::TextInput, params::Parameters};
use gliner::model::pipeline::span::SpanMode;

fn print_optimates() {
    println!("{:?}", JESUS);
    println!("{:?}", APOLLO);
    println!("{:?}", ALEXANDER);
    println!("{:?}", CICERO);
    println!("{:?}", CYRUS);
    println!();
}

fn read_bible() -> HashMap::<Name, Vec<Vec<String>>> {

    let mut word = HashMap::<Name, Vec<Vec<String>>>::new();
    for i in 0..65 {
        word.insert(BOOKS[i], Vec::new());
    }

    {
        let r = bible::io::new_reader(BufReader::new(File::open("./pg10.txt").expect("can't open file")), &mut word);
        for _ in r {}
    }
    
    return word;
}

// type Record = (String, f32, f32, f32, f32);
// 
// fn read_csv() -> Result<Vec<Record>, Box<dyn Error>> {
// 
//     let mut r = csv::Reader::from_reader(File::open("./MacroTrends_Data_Download_BRK.A.trimmed.csv").expect("can't open file"));
//     let mut records = Vec::new();
// 
//     for res in r.deserialize() {
//         let record: Record = res?;
//         records.push(record);
//     }
// 
//     Ok(records)
// }

fn main() -> Result<()> {
    let word = read_bible();

    for src in JESUS.words {
        println!("book: {}", src.book.name);
        println!("chapter: {}", src.chapter);
        println!("verses: {:?}", src.verses);
        let text = &word[&src.book.name][src.chapter-1][src.verses[0]-1..=src.verses[1]-1];
        println!("{:?}", text);
    }

    for deed in JESUS.deeds {
        println!("desc: {}", deed.desc);
        for src in deed.srcs {
            println!("book: {}", src.book.name);
            println!("chapter: {}", src.chapter);
            println!("verses: {:?}", src.verses);
            let text = &word[&src.book.name][src.chapter-1][src.verses[0]-1..=src.verses[1]-1];
            println!("{:?}", text);
        }
    }

    let model = GLiNER::<SpanMode>::new(
        Parameters::default(),
        RuntimeParameters::default(),
        "tokenizer.json",
        "pytorch_model.bin",
    )?;

    let mut verses = Vec::new();
    for (book, chapter_and_verse) in word {
        for (i, chapter) in chapter_and_verse.iter().enumerate() {
            for (j, verse) in chapter.iter().enumerate() {
                verses.push(verse.clone());
            }
        }
    }

    let tmp: Vec<&str> = verses
        .iter()
        .map(|s| s.as_str())
        .collect();

    let input = TextInput::from_str(
        &tmp,
        &[],
    )?;

    println!("{}", model.inference(input)?);

    Ok(())
}
