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
use crate::book::Book;
use crate::greece::macedon::ALEXANDER;
use crate::greece::main::APOLLO;
use crate::name::Name;
use crate::persia::CYRUS;
use crate::rome::CICERO;
use crate::src::Source;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use gliner::model::pipeline::token::TokenMode;
use gliner::model::{GLiNER, input::text::TextInput, params::Parameters};
use gliner::util::result::Result;
use orp::params::RuntimeParameters;

use futures::{StreamExt, stream};

use reqwest::Client;

use scraper::ElementRef;
use scraper::Html;
use scraper::Selector;

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

fn in_the_beginning_was_the() -> HashMap<Name, Vec<Vec<String>>> {
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

    word
}

#[tokio::main]
async fn main() {
    let client = Client::new();

    let urls = ('a'..='z').map(|c| format!("https://gutenberg.org/browse/authors/{}", c));

    let bodies = stream::iter(urls)
        .map(|url| {
            let client = &client;
            async move {
                let res = client.get(url).send().await?;
                res.bytes().await
            }
        })
        .buffer_unordered(10);

    bodies
        .for_each(|b| async {
            match b {
                Ok(b) => {
                    for url in
                        ('a'..='z').map(|c| format!("https://gutenberg.org/browse/authors/{}", c))
                    {
                        let s = String::from_utf8(b.to_vec()).expect("invalid utf8");
                        let doc = Html::parse_document(&s);
                        let selector = Selector::parse("h2").unwrap();
                        for element in doc.select(&selector) {
                            println!("{:#?}", element.inner_html());
                            if let Some(next_sibling) = element
                                .next_sibling()
                                .expect("failed to get next sibling")
                                .next_sibling()
                            {
                                if let Some(next_element_ref) = ElementRef::wrap(next_sibling) {
                                    println!("{:?}", next_element_ref.inner_html());
                                } else {
                                    println!("failed to wrap node with element");
                                    println!("{:#?}", next_sibling.value());
                                }
                            } else {
                                println!("failed to find next sibling");
                            }
                        }
                    }
                }
                Err(e) => eprintln!("e: {}", e),
            }
        })
        .await;
}

#[test]
fn yeshua() {
    let word = in_the_beginning_was_the();

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

    for (book, chapter_and_verse) in &word {
        for (i, chapter) in chapter_and_verse.iter().enumerate() {
            for (j, verse) in chapter.iter().enumerate() {
                if verse.contains("Joshua") {
                    println!("{} {}:{}", book, i + 1, j + 1);
                    println!("{}", verse);
                }
            }
        }
    }
}

#[test]
fn things_that_can_be_named() -> Result<()> {
    let word = in_the_beginning_was_the();

    println!("initiating model...");
    let model = GLiNER::<TokenMode>::new(
        Parameters::default(),
        RuntimeParameters::default(),
        "./tokenizer.json",
        "./model.onnx",
    )?;

    let mut index = HashMap::<String, Vec<Source>>::new();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("setting ctrlc handler");

    for (book, chapter_and_verse) in &word {
        for (i, chapter) in chapter_and_verse.iter().enumerate() {
            if !running.load(Ordering::SeqCst) {
                break;
            }

            println!("initiating input for {} {}...", book, i + 1);
            let input = TextInput::new(chapter.to_vec(), vec![String::from("person")])?;

            println!("inferring...");
            let output = model.inference(input)?;
            for spans in output.spans {
                for span in spans {
                    println!(
                        "{:3} | {:16} | {:10} | {:.1}%",
                        span.sequence() + 1,
                        span.text(),
                        span.class(),
                        span.probability() * 100.0,
                    );

                    let src = Source {
                        book: Book { name: *book },
                        chapter: i + 1,
                        verses: [span.sequence() + 1, span.sequence() + 1],
                    };

                    if let Some(srcs) = index.get_mut(span.text()) {
                        srcs.push(src);
                    } else {
                        index.insert(span.text().to_string(), vec![src]);
                    }
                }
            }
        }
    }

    println!("{:#?}", index);

    Ok(())
}
