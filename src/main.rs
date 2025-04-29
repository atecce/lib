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
use std::io::prelude::*;
use std::io::{BufReader,BufWriter};
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

    let mut word = HashMap::<Name, Vec<Vec<String>>>::new();
    for i in 0..65 {
        word.insert(BOOKS[i], Vec::new());
    }

    {
        let r = bible::io::new_reader(BufReader::new(File::open("./pg10.txt").expect("can't open file")), &mut word);
        for _ in r {}
    }

    let mut w = BufWriter::new(File::create("word.json")?);
    serde_json::to_writer(&mut w, &word)?;
    println!("{:?}", w.flush());

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
