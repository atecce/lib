fn main() {
    for src in [
        bible::Source {
            book: name::Name::Genesis,
            chapter: 28,
            start: 10,
            end: Some(22),
        },
        bible::Source {
            book: name::Name::Exodus,
            chapter: 18,
            start: 14,
            end: Some(27),
        }
    ] {
        let verses = bible::kjv::word(src.book)[src.chapter as usize - 1];
        println!("{}", src);
        let text: &[&str];
        if let Some(end) = src.end {
            text = &verses[src.start as usize - 1..=end as usize - 1];
        } else {
            text = &verses[src.start as usize - 1..src.start as usize];
        }
        println!("{:#?}", text);
    }
}
