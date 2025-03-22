#[derive(Debug)]
pub struct Daemon {
    words: Vec<String>,
    deeds: Vec<String>,

    father: Option<Box<Daemon>>,
    mother: Option<Box<Daemon>>,
}

fn main() {

    let god = Daemon {
        words: vec![
            String::from("text is the universal interface"),
        ],
        deeds: vec![
            String::from("creation"),
        ],

        father: None,
        mother: None,
    };

    println!("{:?}", god);
}
