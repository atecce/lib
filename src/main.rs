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

fn extract_verse<R>(r: &mut BufReader<R>, s: &mut String, b: &mut Vec<u8>, verse: &mut u8) where R: std::io::Read {

    match &s[0..1].parse::<u8>() {
        Ok(n) => {

            *verse = *n;
            b.clear();

            match &s[0..2].parse::<u8>() {
                Ok(n) => {

                    *verse = *n;

                    match &s[0..3].parse::<u8>() {
                        Ok(n) => *verse = *n,
                        Err(_) => _ = 0,
                    }
                },
                Err(_) => _ = 0,
            }
        },
        Err(e) => {

            println!("failed conversion {}", e);

            r.read_until(b':', b);
            *s = String::from_utf8_lossy(&b).to_string();
            println!("debug: {}", s);

            let mut tmp = String::new();
            io::stdin().read_line(&mut tmp).ok().expect("failed to read line");
            extract_verse(r, s, b, verse);
        }
    }
}

fn main() {
    println!("{:?}", JESUS);
    println!("{:?}", APOLLO);
    println!("{:?}", ALEXANDER);
    println!("{:?}", CICERO);
    println!("{:?}", CYRUS);
    println!();

    let mut r = BufReader::new(File::open("./pg10.txt").expect("can't open file"));

    let mut chapter: u8 = 0;
    let mut verse: u8 = 0;

    let mut b: Vec<u8> = Vec::new();
    let mut started = false;
    let mut tmp = String::new();

    while r.read_until(b':', &mut b).is_ok() {

        let mut s = String::from_utf8_lossy(&b).to_string();

        if chapter > 0 {
            extract_verse(&mut r, &mut s, &mut b, &mut verse);
        }

        // TODO(atec): some recursive bullshit
        match &s[s.len()-2..s.len()-1].parse::<u8>() {
            Ok(n) => {

                chapter = *n;

                match &s[s.len()-3..s.len()-1].parse::<u8>() {
                    Ok(n) => {

                        chapter = *n;

                        match &s[s.len()-4..s.len()-1].parse::<u8>() {
                            Ok(n) => chapter = *n,
                            Err(_) => _ = 0,
                        }
                    },
                    Err(_) => _ = 0,
                }

                // TODO(atec): hack to avoid index error on s[0..1]
                if !started {
                    started = true;
                    b.clear();
                }
            },
            Err(_) => _ = 0,
        }

        println!("chapter: {}", chapter);
        println!("verse: {}", verse);
        println!("s: {}", s);

        io::stdin().read_line(&mut tmp).ok().expect("failed to read line");
    }
}
