mod persia;
mod rome;

use crate::persia::CYRUS;
use crate::rome::CICERO;

use bible::JESUS;
use book::Book;
use daemon::genealogy;
use greece::APOLLO;
use greece::macedon::ALEXANDER;
use name::Name;
use source::Source;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use gliner::model::pipeline::token::TokenMode;
use gliner::model::{GLiNER, input::text::TextInput, params::Parameters};
use gliner::util::result::Result;
use orp::params::RuntimeParameters;

fn print_optimates() {
    println!("{:?}", genealogy(*JESUS));
    println!("{:?}", genealogy(*APOLLO));
    println!("{:?}", genealogy(*ALEXANDER));
    println!("{:?}", genealogy(*CICERO));
    println!("{:?}", genealogy(*CYRUS));
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
