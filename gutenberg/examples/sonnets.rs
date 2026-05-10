fn main() {
    println!("{:#?}", gutenberg::shakespeare::new_reader().read_until_sonnets());
}
