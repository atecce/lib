mod persia;
mod rome;

use crate::persia::CYRUS;
use crate::rome::CICERO;

use bible::JESUS;
use book::Book;
use greece::macedon::ALEXANDER;
use greece::APOLLO;
use name::Name;
use source::Source;

use std::collections::HashMap;
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

fn main() {
    print_optimates();
}

const DOMAIN: &str = "https://gutenberg.org";

#[tokio::test]
async fn scrape() {
    let client = Client::new();

    let bodies = stream::iter(('a'..='z').map(|c| format!("{}/browse/authors/{}", DOMAIN, c)))
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
                    for element in
                        Html::parse_document(&String::from_utf8(b.to_vec()).expect("invalid utf8"))
                            .select(&Selector::parse("h2").unwrap())
                    {
                        println!("{:#?}", element.inner_html());
                        if let Some(next_next_sibling) = element
                            .next_sibling()
                            .expect("failed to get next sibling")
                            .next_sibling()
                        {
                            if let Some(next_next_element_ref) = ElementRef::wrap(next_next_sibling)
                            {
                                println!(
                                    "next next sibling: {:?}",
                                    next_next_element_ref.inner_html()
                                );

                                let mut urls = Vec::new();

                                for li in
                                    next_next_element_ref.select(&Selector::parse("li").unwrap())
                                {
                                    println!("\t{}", li.inner_html());
                                    for a in li.select(&Selector::parse("a").unwrap()) {
                                        println!("\t\t{:?}", a);
                                        if let Some(href) = a.value().attr("href") {
                                            println!("\t\t{}", href);
                                            println!("\t\t{}{}", DOMAIN, href);

                                            let url = format!("{}{}.txt.utf-8", DOMAIN, href);
                                            println!("\t\t{}", url);
                                            urls.push(url);
                                        } else {
                                            println!("\t\tfailed to find href");
                                        }
                                    }
                                }

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
                                            Ok(b) => println!("{:?}", b),
                                            Err(e) => eprintln!("e: {}", e),
                                        }
                                    })
                                    .await;
                            } else {
                                println!("failed to wrap node with element");
                                println!("{:#?}", next_next_sibling.value());
                            }
                        } else {
                            println!("failed to find next next sibling");
                        }
                    }
                }
                Err(e) => eprintln!("e: {}", e),
            }
        })
        .await;
}

#[test]
fn things_that_can_be_named() -> Result<()> {
    let word = bible::io::read_all();

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

    println!("{:#?}", serde_json::to_string(&index).unwrap());

    Ok(())
}
