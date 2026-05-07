use lib::persia::CYRUS;
use lib::rome::CICERO;

use daemon::genealogy;

use bible::JESUS;
use greece::APOLLO;
use greece::macedon::ALEXANDER;

fn main() {
    println!("{:?}", genealogy(*JESUS).into_iter().map(|f| f.names).collect::<Vec<_>>());
    println!("{:?}", genealogy(*APOLLO).into_iter().map(|f| f.names).collect::<Vec<_>>());
    println!("{:?}", genealogy(*ALEXANDER).into_iter().map(|f| f.names).collect::<Vec<_>>());
    println!("{:?}", genealogy(*CICERO).into_iter().map(|f| f.names).collect::<Vec<_>>());
    println!("{:?}", genealogy(*CYRUS).into_iter().map(|f| f.names).collect::<Vec<_>>());
    println!();
}
