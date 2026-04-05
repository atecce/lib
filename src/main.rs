mod bible;
mod book;
mod daemon;
mod deed;
mod greece;
mod name;
mod persia;
mod rome;
mod src;

use crate::bible::main::BOOKS;
use crate::bible::main::JESUS;
use crate::greece::macedon::ALEXANDER;
use crate::greece::main::APOLLO;
use crate::name::Name;
use crate::persia::CYRUS;
use crate::rome::CICERO;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use gliner::model::pipeline::token::TokenMode;
use gliner::model::{GLiNER, input::text::TextInput, params::Parameters};
use gliner::util::result::Result;
use orp::params::RuntimeParameters;

fn print_optimates() {
    println!("{:?}", JESUS);
    println!("{:?}", APOLLO);
    println!("{:?}", ALEXANDER);
    println!("{:?}", CICERO);
    println!("{:?}", CYRUS);
    println!();
}

// type Record = (String, f32, f32, f32, f32);
//
// fn read_csv() -> Result<Vec<Record>, Box<dyn Error>> {
//     let mut r = csv::Reader::from_reader(
//         File::open("./MacroTrends_Data_Download_BRK.A.trimmed.csv").expect("can't open file"),
//     );
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
    println!("initiating word...");
    let mut word = HashMap::<Name, Vec<Vec<String>>>::new();
    for i in 0..65 {
        word.insert(BOOKS[i], Vec::new());
    }

    println!("populating verses...");
    let mut verses = Vec::new();
    {
        let r = bible::io::new_reader(
            BufReader::new(File::open("./pg10.txt").expect("can't open file")),
            &mut word,
        );

        for (_, _, _, verse) in r {
            verses.push(verse);
        }
    }

    println!("initiating model...");
    let model = GLiNER::<TokenMode>::new(
        Parameters::default(),
        RuntimeParameters::default(),
        "./tokenizer.json",
        "./model.onnx",
    )?;

    println!("initiating input...");
    let input = TextInput::new(
        verses.get(..100).unwrap().to_vec(),
        vec![String::from("person")],
    )?;

    println!("inferring...");
    let output = model.inference(input)?;
    for spans in output.spans {
        for span in spans {
            println!(
                "{:3} | {:16} | {:10} | {:.1}%",
                span.sequence(),
                span.text(),
                span.class(),
                span.probability() * 100.0
            );
        }
    }

    for src in JESUS.words {
        println!("book: {}", src.book.name);
        println!("chapter: {}", src.chapter);
        println!("verses: {:?}", src.verses);
        let text = &word[&src.book.name][src.chapter - 1][src.verses[0] - 1..=src.verses[1] - 1];
        println!("{:?}", text);
    }

    for deed in JESUS.deeds {
        println!("desc: {}", deed.desc);
        for src in deed.srcs {
            println!("book: {}", src.book.name);
            println!("chapter: {}", src.chapter);
            println!("verses: {:?}", src.verses);
            let text =
                &word[&src.book.name][src.chapter - 1][src.verses[0] - 1..=src.verses[1] - 1];
            println!("{:?}", text);
        }
    }

    for (book, chapter_and_verse) in word {
        for (i, chapter) in chapter_and_verse.iter().enumerate() {
            for (j, verse) in chapter.iter().enumerate() {
                if verse.contains("Joshua") {
                    println!("{} {}:{}", book, i + 1, j + 1);
                    println!("{}", verse);
                }
            }
        }
    }

    Ok(())
}
