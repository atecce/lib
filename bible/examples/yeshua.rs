fn main() {
    for src in bible::genealogy::JESUS.words {
        let verses = bible::kjv::word(src.book)[src.chapter as usize - 1];
        println!("{}", src);
        let text: &[&str];
        if let Some(end) = src.end {
            text = &verses[src.start as usize - 1..=end as usize - 1];
        } else {
            text = &verses[src.start as usize - 1..src.start as usize];
        }
        println!("{:?}", text);
    }

    for deed in bible::genealogy::JESUS.deeds {
        println!("desc: {}", deed.desc);
        for src in deed.srcs {
            let verses = bible::kjv::word(src.book)[src.chapter as usize - 1];
            println!("{}", src);
            let text: &[&str];
            if let Some(end) = src.end {
                text = &verses[src.start as usize - 1..=end as usize - 1];
            } else {
                text = &verses[src.start as usize - 1..src.start as usize];
            }
            println!("{:?}", text);
        }
    }

    for book in name::BIBLE {
        for (i, chapter) in bible::kjv::word(book).iter().enumerate() {
            for (j, verse) in chapter.iter().enumerate() {
                if verse.contains("Joshua") {
                    println!(
                        "{}",
                        bible::Source {
                            book: book,
                            chapter: (i + 1).try_into().unwrap(),
                            start: (j + 1).try_into().unwrap(),
                            end: Some((j + 1).try_into().unwrap()),
                        }
                    );
                    println!("{}", verse);
                }
            }
        }
    }
}
