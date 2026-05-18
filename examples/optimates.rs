use lib::persia::CYRUS;
use lib::rome::CICERO;

fn main() {
    println!("{:?}", std::iter::successors(Some(bible::genealogy::יֵשׁוּ), |d| d.father.map(|boxed| boxed)).map(|f| f.names).collect::<Vec<_>>());
    println!("{:?}", std::iter::successors(Some(greece::ΑΠΟΛΛΩΝ), |d| d.father.map(|boxed| boxed)).map(|f| f.names).collect::<Vec<_>>());
    println!("{:?}", std::iter::successors(Some(greece::macedon::ΑΛΕΞΑΝΔΡΟΣ), |d| d.father.map(|boxed| boxed)).map(|f| f.names).collect::<Vec<_>>());
    println!("{:?}", std::iter::successors(Some(CICERO), |d| d.father.map(|boxed| boxed)).map(|f| f.names).collect::<Vec<_>>());
    println!("{:?}", std::iter::successors(Some(CYRUS), |d| d.father.map(|boxed| boxed)).map(|f| f.names).collect::<Vec<_>>());
}
