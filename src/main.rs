mod daemon;
mod bible;
mod homer;
mod rome;

use crate::bible::JESUS;
use crate::homer::APOLLO;
use crate::rome::CICERO;

fn main() {
    println!("{:?}", JESUS);
    println!("{:?}", APOLLO);
    println!("{:?}", CICERO);
}
