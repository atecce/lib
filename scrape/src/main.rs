use futures::{StreamExt, stream};
use reqwest::Client;
use scraper::ElementRef;
use scraper::Html;
use scraper::Selector;

use gutenberg::DOMAIN;

#[tokio::main]
async fn main() {
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
