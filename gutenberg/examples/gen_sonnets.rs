fn main() {
    let sonnets = gutenberg::shakespeare::new_reader().read_until_sonnets();

    println!("pub const SONNETS: &'static [&'static [&'static str]] = &[");
    for sonnet in sonnets {
        println!("    &[");
        for line in sonnet {
            println!("        \"{}\",", line);
        }
        println!("    ],");
    }
    println!("];");
}
