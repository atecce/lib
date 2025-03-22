#[derive(Debug)]
pub struct Daemon<'a> {
    words: Vec<&'a str>,
    deeds: Vec<&'a str>,
}

fn main() {

    let god = Daemon {
        words: vec![
            "text is the universal interface",
        ],
        deeds: vec![
            "creation",
        ],
    };

    println!("{:?}", god);
}
