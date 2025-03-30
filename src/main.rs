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

use std::fs::File;
use std::io;
use std::io::{BufRead,BufReader};
use crate::bible::JESUS;
use crate::greece::APOLLO;
use crate::macedon::ALEXANDER;
use crate::rome::CICERO;
use crate::persia::CYRUS;

fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>());
}

fn main() {
    println!("{:?}", JESUS);
    println!("{:?}", APOLLO);
    println!("{:?}", ALEXANDER);
    println!("{:?}", CICERO);
    println!("{:?}", CYRUS);
    println!();

    let mut r = BufReader::new(File::open("./pg10.txt").expect("can't open file"));

    let mut b: Vec<u8> = Vec::new();
    let mut chapter: u8 = 0;
    let mut verse: u8 = 0;

    while r.read_until(b':', &mut b).is_ok() {

        let s = String::from_utf8_lossy(&b);

        if chapter > 0 {

            let head = &s[0..1];

            // TODO(atec): some recursive bullshit
            match head.parse::<u8>() {
                Ok(n) => {

                    verse = n;

                    let head = &s[0..2];
                    match head.parse::<u8>() {
                        Ok(n) => {

                            verse = n;

                            let head = &s[0..3];
                            match head.parse::<u8>() {
                                Ok(n) => verse = n,
                                Err(_) => _ = 0,
                            }
                        },
                        Err(_) => _ = 0,
                    }
                },
                Err(_) => _ = 0,
            }
        }

        let tail = &s[s.len()-2..s.len()-1];

        // TODO(atec): some recursive bullshit
        match tail.parse::<u8>() {
            Ok(n) => {

                chapter = n;

                let tail = &s[s.len()-3..s.len()-1];
                match tail.parse::<u8>() {
                    Ok(n) => {

                        chapter = n;

                        let tail = &s[s.len()-4..s.len()-1];
                        match tail.parse::<u8>() {
                            Ok(n) => chapter = n,
                            Err(_) => _ = 0,
                        }
                    },
                    Err(_) => _ = 0,
                }
            },
            Err(_) => _ = 0,
        }

        println!("chapter: {}", chapter);
        println!("verse: {}", verse);
        println!("s: {}\n", s);

        b.clear();

        let mut tmp = String::new();
        io::stdin().read_line(&mut tmp).ok().expect("failed to read line");
    }
}
