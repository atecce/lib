mod daemon;
mod bible;
mod greece;
mod macedon;
mod rome;

use crate::bible::JESUS;
use crate::greece::APOLLO;
use crate::macedon::ALEXANDER;
use crate::rome::CICERO;

fn main() {
    println!("{:?}", JESUS);
    println!("{:?}", APOLLO);
    println!("{:?}", ALEXANDER);
    println!("{:?}", CICERO);
}
