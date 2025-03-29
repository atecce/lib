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

//use std::fs::File;
//use std::io::{BufRead, BufReader};
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

//    let mut r = BufReader::new(File::open("./pg10.txt").expect("can't open file"));
//
//    for line in r.lines() {
//        let line = line.expect("can't read line");
//        let s = line.as_str();
//        let sIsEmpty = s.is_empty();
//        if !sIsEmpty {
//            println!("{:}", s.split(':').collect::<Vec<_>>());
//        }
//    }
}
