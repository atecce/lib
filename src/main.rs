#[derive(Debug)]
pub struct Daemon {
    words: Vec<String>,
    deeds: Vec<String>,

    father: Option<Box<Daemon>>,
    mother: Option<Box<Daemon>>,
}

fn main() {

    let almighty = Daemon {
        words: vec![
            String::from("text is the universal interface"),
        ],
        deeds: vec![
            String::from("creation"),
        ],

        father: None,
        mother: None,
    };

    let mary = Daemon {
        words: vec![],
        deeds: vec![],

        father: None,
        mother: None,
    };

    let jesus = Daemon {
        words: vec![
            String::from("render Caesar what is Caesar's"),
        ],
        deeds: vec![
            String::from("walked on water"),
        ],

        father: Some(Box::new(almighty)),
        mother: Some(Box::new(mary)),
    };

    println!("{:?}", jesus);
}
