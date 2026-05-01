mod persia;
mod rome;

use crate::persia::CYRUS;
use crate::rome::CICERO;

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};

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

//                   Contents
//
//    THE SONNETS
//    ALL’S WELL THAT ENDS WELL
//    THE TRAGEDY OF ANTONY AND CLEOPATRA
//    AS YOU LIKE IT
//    THE COMEDY OF ERRORS
//    THE TRAGEDY OF CORIOLANUS
//    CYMBELINE
//    THE TRAGEDY OF HAMLET, PRINCE OF DENMARK
//    THE FIRST PART OF KING HENRY THE FOURTH
//    THE SECOND PART OF KING HENRY THE FOURTH
//    THE LIFE OF KING HENRY THE FIFTH
//    THE FIRST PART OF HENRY THE SIXTH
//    THE SECOND PART OF KING HENRY THE SIXTH
//    THE THIRD PART OF KING HENRY THE SIXTH
//    KING HENRY THE EIGHTH
//    THE LIFE AND DEATH OF KING JOHN
//    THE TRAGEDY OF JULIUS CAESAR
//    THE TRAGEDY OF KING LEAR
//    LOVE’S LABOUR’S LOST
//    THE TRAGEDY OF MACBETH
//    MEASURE FOR MEASURE
//    THE MERCHANT OF VENICE
//    THE MERRY WIVES OF WINDSOR
//    A MIDSUMMER NIGHT’S DREAM
//    MUCH ADO ABOUT NOTHING
//    THE TRAGEDY OF OTHELLO, THE MOOR OF VENICE
//    PERICLES, PRINCE OF TYRE
//    KING RICHARD THE SECOND
//    KING RICHARD THE THIRD
//    THE TRAGEDY OF ROMEO AND JULIET
//    THE TAMING OF THE SHREW
//    THE TEMPEST
//    THE LIFE OF TIMON OF ATHENS
//    THE TRAGEDY OF TITUS ANDRONICUS
//    TROILUS AND CRESSIDA
//    TWELFTH NIGHT; OR, WHAT YOU WILL
//    THE TWO GENTLEMEN OF VERONA
//    THE TWO NOBLE KINSMEN
//    THE WINTER’S TALE
//    A LOVER’S COMPLAINT
//    THE PASSIONATE PILGRIM
//    THE PHOENIX AND THE TURTLE
//    THE RAPE OF LUCRECE
//    VENUS AND ADONIS


fn main() -> Result<(), Box<dyn Error>> {

    let r = BufReader::new(&include_bytes!("../gutenberg/cache/epub/100/pg100.txt")[..]);
    let mut found = false;

    for line in r.lines() {
        let line = line?;
        if line.contains("THE SONNETS") {
            println!("{:?}", line);
        }
    }

    Ok(())
}

fn generate_word() {

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
}
