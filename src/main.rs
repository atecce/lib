mod name;
mod word;
mod book;
mod daemon;
mod bible;
mod greece;
mod macedon;
mod rome;
mod persia;

use crate::bible::JESUS;
use crate::greece::APOLLO;
use crate::macedon::ALEXANDER;
use crate::rome::CICERO;
use crate::persia::CYRUS;

fn main() {
    println!("{:?}", JESUS);
    println!("{:?}", APOLLO);
    println!("{:?}", ALEXANDER);
    println!("{:?}", CICERO);
    println!("{:?}", CYRUS);
}
