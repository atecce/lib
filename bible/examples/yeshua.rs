fn main() {
    for src in bible::JESUS.words {
        println!("{}", src);
        let text = &bible::kjv::word(src.book)[src.chapter as usize - 1]
            [src.verses[0] as usize - 1..=src.verses[1] as usize - 1];
        println!("{:?}", text);
    }

    for deed in bible::JESUS.deeds {
        println!("desc: {}", deed.desc);
        for src in deed.srcs {
            println!("{}", src);
            let text = &bible::kjv::word(src.book)[src.chapter as usize - 1]
                [src.verses[0] as usize - 1..=src.verses[1] as usize - 1];
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
                            verses: [(j + 1).try_into().unwrap(), (j + 1).try_into().unwrap()],
                        }
                    );
                    println!("{}", verse);
                }
            }
        }
    }
}
