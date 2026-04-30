mod persia;
mod rome;

use crate::persia::CYRUS;
use crate::rome::CICERO;

use std::error::Error;
use std::fs::File;
use std::io::BufWriter;

use bible::JESUS;
use bible::io::read_all;
use bible::kjv::word;
use bible::kjv::get_word;

use daemon::genealogy;
use greece::APOLLO;
use greece::macedon::ALEXANDER;

fn print_optimates() {
    println!("{:?}", genealogy(*JESUS).into_iter().map(|f| f.names).collect::<Vec<_>>());
    println!("{:?}", genealogy(*APOLLO).into_iter().map(|f| f.names).collect::<Vec<_>>());
    println!("{:?}", genealogy(*ALEXANDER).into_iter().map(|f| f.names).collect::<Vec<_>>());
    println!("{:?}", genealogy(*CICERO).into_iter().map(|f| f.names).collect::<Vec<_>>());
    println!("{:?}", genealogy(*CYRUS).into_iter().map(|f| f.names).collect::<Vec<_>>());
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

fn main() -> Result<(), Box<dyn Error>> {

    let word = bible::io::read_all();
    println!("pub const fn word(book: Name) -> &'static [&'static [&'static str]] {{");
    println!("    match book {{");

    for book in name::BIBLE {
        let chapters = &word[&book];

        println!("        name::Name::{} => &[", book);
        for chapter in chapters {
            println!("            &[");

            for verse in chapter {
                let escaped_verse = verse.replace('"', "\\\"");
                println!("                \"{}\",", escaped_verse);
            }
            println!("            ],");
        }
        println!("        ],");
    }
    println!("        _ => panic!(\"unknown book\"),");
    println!("    }}");
    println!("}}");

    Ok(())
}
