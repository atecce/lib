use name::BIBLE;
use name::Name;
use name::Name::JohnII;
use name::Name::JohnIII;
use name::Name::Jude;
use name::Name::Obadiah;
use name::Name::Philemon;
use name::Name::Revelation;

use std::io::{BufRead, BufReader};
use std::num::ParseIntError;

use futures::{StreamExt, stream};

use reqwest::Client;

use scraper::ElementRef;
use scraper::Html;
use scraper::Selector;

const DOMAIN: &str = "https://gutenberg.org";

pub struct Reader<R> {
    r: BufReader<R>,
    b: Vec<u8>,
    book: Name,
    i: usize,
    new_book: bool,
    chapter: usize,
    last_chapter: usize,
    new_chapter: bool,
    started: bool,
    revelation: bool,
    amen: bool,
}

pub fn new_reader<R: std::io::Read>(
    r: BufReader<R>,
) -> Reader<R> {
    Reader {
        r: r,
        b: Vec::new(),
        book: BIBLE[0],
        i: 0,
        new_book: false,
        chapter: 1,
        last_chapter: 1,
        new_chapter: false,
        started: false,
        revelation: false,
        amen: false,
    }
}

impl<R: std::io::Read> Iterator for Reader<R> {
    type Item = (Name, usize, usize, String);

    fn next(&mut self) -> Option<Self::Item> {
        // special case for final verse
        if self.amen {
            return None;
        }
        if self.revelation {
            self.amen = true;
            let text = "21 The grace of our Lord Jesus Christ be with you all. Amen.";
            return Some((Revelation, 22, 21, text.to_string()));
        }

        // TODO(atec): hack to avoid index error on s[0..1] at beginning
        if !self.started {
            while self.r.read_until(b':', &mut self.b).is_ok() {
                let mut s = String::from_utf8_lossy(&self.b).to_string();

                if s.is_char_boundary(s.len() - 1)
                    && s.is_char_boundary(s.len() - 2)
                    && s[s.len() - 2..s.len() - 1].parse::<usize>().is_ok()
                {
                    self.started = true;
                    self.b.clear();
                    let _ = self.r.read_until(b':', &mut self.b);
                    s = String::from_utf8_lossy(&self.b).to_string();

                    let (verse, text) = self.extract_verse();
                    return Some((self.book, self.chapter, verse, text));
                }

                self.b.clear();
            }
        }

        // TODO(atec); perhaps use returned byte number
        while self.r.read_until(b':', &mut self.b).is_ok() {
            match self.extract_chapter() {
                Ok(n) => {
                    if self.new_book {
                        self.i += 1;
                        self.book = BIBLE[self.i];
                        self.new_book = false;
                    }
                    if self.new_chapter {
                        self.chapter = n;
                        self.new_chapter = false;
                    }
                    if self.last_chapter != n {
                        if self.last_chapter > n {
                            self.new_book = true;
                        }
                        self.new_chapter = true;
                        self.last_chapter = n;
                    }

                    let (verse, text) = self.extract_verse();

                    // special case for Philemon, JohnII, JohnIII, and Jude
                    // these books only have one chapter so we will just
                    // hard code the final verse
                    if (self.book == Obadiah && verse == 21)
                        || (self.book == Philemon && verse == 25)
                        || (self.book == JohnII && verse == 13)
                        || (self.book == JohnIII && verse == 14)
                        || (self.book == Jude && verse == 25)
                    {
                        self.new_book = true;
                        self.new_chapter = true;
                    }

                    // special case for penultimate verse
                    if self.book == Revelation && self.chapter == 22 && verse == 20 {
                        self.revelation = true;
                    }

                    return Some((self.book, self.chapter, verse, text));
                }
                Err(_) => {
                    // TODO(atec): possibly accumulate warnings
                    continue;
                }
            }
        }
        None
    }
}

impl<R> Reader<R> {
    fn extract_chapter(&self) -> Result<usize, ParseIntError> {
        let s = String::from_utf8_lossy(&self.b).to_string();
        let mut chapter = 0;

        if s.is_char_boundary(s.len() - 1) && s.is_char_boundary(s.len() - 2) {
            for i in 0..3 {
                match &s[s.len() - (2 + i)..s.len() - 1].parse::<usize>() {
                    Ok(n) => {
                        chapter = *n;
                    }
                    Err(e) => {
                        if i == 0 {
                            return Err(e.clone());
                        }
                        // TODO(atec): warn about nested fallbacks
                    }
                }
            }
        } else {
            // TODO(atec): perhaps create own err type
            return "is not character boundary".parse::<usize>();
        }

        return Ok(chapter);
    }

    fn extract_verse(&mut self) -> (usize, String)
    where
        R: std::io::Read,
    {
        let mut s = String::from_utf8_lossy(&self.b).to_string();
        let mut verse = 0;

        // TODO(atec): hack at the end
        if s.len() == 0 {
            return (verse, "".to_string());
        }

        for i in 0..3 {
            match &s[0..(1 + i)].parse::<usize>() {
                Ok(n) => {
                    verse = *n;
                }
                Err(e) => {
                    if i == 0 {
                        // TODO(atec): perhaps panic
                        println!("failed conversion extracting verse: {}", e);
                        println!(
                            "should not happen because we read to the end of the verse as soon as we match"
                        );
                    } else {
                        // TODO(atec): warn about fallbacks
                    }
                }
            }
        }

        let mut next_verse = s.is_char_boundary(s.len() - 1)
            && s.is_char_boundary(s.len() - 2)
            && s[s.len() - 2..s.len() - 1].parse::<usize>().is_ok();

        while !next_verse {
            // TODO(atec); perhaps use returned byte number
            let _ = self.r.read_until(b':', &mut self.b);
            s = String::from_utf8_lossy(&self.b).to_string();

            next_verse = s.is_char_boundary(s.len() - 1)
                && s.is_char_boundary(s.len() - 2)
                && s[s.len() - 2..s.len() - 1].parse::<usize>().is_ok()
        }

        self.b.clear();

        return (verse, s.replace("\r\n", " "));
    }
}

#[tokio::test]
async fn scrape() {
    let client = Client::new();

    let bodies = stream::iter(('a'..='z').map(|c| format!("{}/browse/authors/{}", DOMAIN, c)))
        .map(|url| {
            let client = &client;
            async move {
                let res = client.get(url).send().await?;
                res.bytes().await
            }
        })
        .buffer_unordered(10);

    bodies
        .for_each(|b| async {
            match b {
                Ok(b) => {
                    for element in
                        Html::parse_document(&String::from_utf8(b.to_vec()).expect("invalid utf8"))
                            .select(&Selector::parse("h2").unwrap())
                    {
                        println!("{:#?}", element.inner_html());
                        if let Some(next_next_sibling) = element
                            .next_sibling()
                            .expect("failed to get next sibling")
                            .next_sibling()
                        {
                            if let Some(next_next_element_ref) = ElementRef::wrap(next_next_sibling)
                            {
                                println!(
                                    "next next sibling: {:?}",
                                    next_next_element_ref.inner_html()
                                );

                                let mut urls = Vec::new();

                                for li in
                                    next_next_element_ref.select(&Selector::parse("li").unwrap())
                                {
                                    println!("\t{}", li.inner_html());
                                    for a in li.select(&Selector::parse("a").unwrap()) {
                                        println!("\t\t{:?}", a);
                                        if let Some(href) = a.value().attr("href") {
                                            println!("\t\t{}", href);
                                            println!("\t\t{}{}", DOMAIN, href);

                                            let url = format!("{}{}.txt.utf-8", DOMAIN, href);
                                            println!("\t\t{}", url);
                                            urls.push(url);
                                        } else {
                                            println!("\t\tfailed to find href");
                                        }
                                    }
                                }

                                let bodies = stream::iter(urls)
                                    .map(|url| {
                                        let client = &client;
                                        async move {
                                            let res = client.get(url).send().await?;
                                            res.bytes().await
                                        }
                                    })
                                    .buffer_unordered(10);

                                bodies
                                    .for_each(|b| async {
                                        match b {
                                            Ok(b) => println!("{:?}", b),
                                            Err(e) => eprintln!("e: {}", e),
                                        }
                                    })
                                    .await;
                            } else {
                                println!("failed to wrap node with element");
                                println!("{:#?}", next_next_sibling.value());
                            }
                        } else {
                            println!("failed to find next next sibling");
                        }
                    }
                }
                Err(e) => eprintln!("e: {}", e),
            }
        })
        .await;
}
