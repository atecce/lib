mod persia;
mod rome;

use crate::persia::CYRUS;
use crate::rome::CICERO;

use bible::JESUS;
use daemon::genealogy;
use greece::APOLLO;
use greece::macedon::ALEXANDER;

fn print_optimates() {
    println!("{:?}", genealogy(*JESUS));
    println!("{:?}", genealogy(*APOLLO));
    println!("{:?}", genealogy(*ALEXANDER));
    println!("{:?}", genealogy(*CICERO));
    println!("{:?}", genealogy(*CYRUS));
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

fn main() {
    print_optimates();
}
