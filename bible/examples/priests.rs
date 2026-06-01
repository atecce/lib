fn main() {
    println!("{:?}", std::iter::successors(Some(bible::genealogy::AARON), |d| d.father.map(|boxed| boxed)).map(|f| f.names).collect::<Vec<_>>());
}
