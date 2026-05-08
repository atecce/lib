fn main() {
    let word = gutenberg::read_all();
    println!("pub const fn word(book: Name) -> &'static [&'static [&'static str]] {{");
    println!("    match book {{");

    for book in name::BIBLE {
        let chapters = &word[&book];

        println!("        name::Name::{} => &[", book);
        for chapter in chapters {
            println!("            &[");

            for verse in chapter {
                let escaped_verse = verse.replace('"', "\\\"");
                println!("                \"{}\",", escaped_verse);
            }
            println!("            ],");
        }
        println!("        ],");
    }
    println!("        _ => panic!(\"unknown book\"),");
    println!("    }}");
    println!("}}");
}
