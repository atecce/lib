use lib::persia::CYRUS;
use lib::rome::CICERO;

use bible::genealogy::JESUS;
use greece::APOLLO;
use greece::macedon::ALEXANDER;

fn main() {
    println!("{:?}", std::iter::successors(Some(JESUS), |d| d.father.map(|boxed| boxed)).map(|f| f.names).collect::<Vec<_>>());
    println!("{:?}", std::iter::successors(Some(APOLLO), |d| d.father.map(|boxed| boxed)).map(|f| f.names).collect::<Vec<_>>());
    println!("{:?}", std::iter::successors(Some(ALEXANDER), |d| d.father.map(|boxed| boxed)).map(|f| f.names).collect::<Vec<_>>());
    println!("{:?}", std::iter::successors(Some(CICERO), |d| d.father.map(|boxed| boxed)).map(|f| f.names).collect::<Vec<_>>());
    println!("{:?}", std::iter::successors(Some(CYRUS), |d| d.father.map(|boxed| boxed)).map(|f| f.names).collect::<Vec<_>>());
}
