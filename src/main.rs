// Luke 3:23-38

#[derive(Debug)]
pub struct Daemon {
    names: Vec<String>,
    words: Vec<String>,
    deeds: Vec<String>,

    father: Option<Box<Daemon>>,
    mother: Option<Box<Daemon>>,
}

fn main() {

    let almighty = Daemon {
        names: vec![
            String::from("God"),
        ],
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
        names: vec![
            String::from("Mary"),
        ],
        words: vec![],
        deeds: vec![],

        father: None,
        mother: None,
    };

    let adam = Daemon {
        names: vec![
            String::from("Adam"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(almighty)),
        mother: None,
    };

    let seth = Daemon {
        names: vec![
            String::from("Seth"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(adam)),
        mother: None,
    };

    let enosh = Daemon {
        names: vec![
            String::from("Enosh"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(seth)),
        mother: None,
    };

    let kenan = Daemon {
        names: vec![
            String::from("Kenan"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(enosh)),
        mother: None,
    };

    let mahalalel = Daemon {
        names: vec![
            String::from("Mahalalel"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(kenan)),
        mother: None,
    };

    let jared = Daemon {
        names: vec![
            String::from("Jared"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(mahalalel)),
        mother: None,
    };

    let enoch = Daemon {
        names: vec![
            String::from("Enoch"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(jared)),
        mother: None,
    };

    let methuselah = Daemon {
        names: vec![
            String::from("Methuselah"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(enoch)),
        mother: None,
    };

    let lamech = Daemon {
        names: vec![
            String::from("Lamech"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(methuselah)),
        mother: None,
    };
 
     let noah = Daemon {
        names: vec![
            String::from("Noah"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(lamech)),
        mother: None,
    };

    let shem = Daemon {
        names: vec![
            String::from("Shem"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(noah)),
        mother: None,
    };

    let arphaxad = Daemon {
        names: vec![
            String::from("Arphaxad"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(shem)),
        mother: None,
    };

    let cainan = Daemon {
        names: vec![
            String::from("Cainan"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(arphaxad)),
        mother: None,
    };

    let shelah = Daemon {
        names: vec![
            String::from("Eber"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(cainan)),
        mother: None,
    };

    let eber = Daemon {
        names: vec![
            String::from("Eber"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(shelah)),
        mother: None,
    };

    let peleg = Daemon {
        names: vec![
            String::from("Peleg"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(eber)),
        mother: None,
    };

    let reu = Daemon {
        names: vec![
            String::from("Reu"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(peleg)),
        mother: None,
    };

    let serug = Daemon {
        names: vec![
            String::from("Serug"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(reu)),
        mother: None,
    };

    let nahor = Daemon {
        names: vec![
            String::from("Nahor"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(serug)),
        mother: None,
    };

    let terrah = Daemon {
        names: vec![
            String::from("Terrah"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(nahor)),
        mother: None,
    };

    let abraham = Daemon {
        names: vec![
            String::from("Abraham"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(terrah)),
        mother: None,
    };

    let isaac = Daemon {
        names: vec![
            String::from("Isaac"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(abraham)),
        mother: None,
    };

    let jacob = Daemon {
        names: vec![
            String::from("Jacob"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(isaac)),
        mother: None,
    };

    let judah_son_of_jacob = Daemon {
        names: vec![
            String::from("Judah"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(jacob)),
        mother: None,
    };

    let perez = Daemon {
        names: vec![
            String::from("Perez"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(judah_son_of_jacob)),
        mother: None,
    };

    let hezron = Daemon {
        names: vec![
            String::from("Hezron"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(perez)),
        mother: None,
    };

    let ram = Daemon {
        names: vec![
            String::from("Ram"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(hezron)),
        mother: None,
    };

    let amminadab = Daemon {
        names: vec![
            String::from("Amminadab"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(ram)),
        mother: None,
    };

    let nashon = Daemon {
        names: vec![
            String::from("Nashon"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(amminadab)),
        mother: None,
    };

    let salmon = Daemon {
        names: vec![
            String::from("Salmon"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(nashon)),
        mother: None,
    };

    let boaz = Daemon {
        names: vec![
            String::from("Boaz"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(salmon)),
        mother: None,
    };

    let obed = Daemon {
        names: vec![
            String::from("Obed"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(boaz)),
        mother: None,
    };

    let jesse = Daemon {
        names: vec![
            String::from("Jesse"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(obed)),
        mother: None,
    };

    let david = Daemon {
        names: vec![
            String::from("David"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(jesse)),
        mother: None,
    };

    let nathan = Daemon {
        names: vec![
            String::from("Nathan"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(david)),
        mother: None,
    };

    let mattatha = Daemon {
        names: vec![
            String::from("Mattatha"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(nathan)),
        mother: None,
    };

    let menna = Daemon {
        names: vec![
            String::from("Menna"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(mattatha)),
        mother: None,
    };

    let melea = Daemon {
        names: vec![
            String::from("Melea"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(menna)),
        mother: None,
    };

    let eliakam = Daemon {
        names: vec![
            String::from("Eliakam"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(melea)),
        mother: None,
    };

    let jonam = Daemon {
        names: vec![
            String::from("Jonam"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(eliakam)),
        mother: None,
    };

    let joseph_son_of_jonam = Daemon {
        names: vec![
            String::from("Jonam"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(jonam)),
        mother: None,
    };

    let judah = Daemon {
        names: vec![
            String::from("Judah"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(joseph_son_of_jonam)),
        mother: None,
    };

    let simeon = Daemon {
        names: vec![
            String::from("Simeon"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(judah)),
        mother: None,
    };

    let levi_son_of_simeon = Daemon {
        names: vec![
            String::from("Levi"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(simeon)),
        mother: None,
    };

    let matthat_son_of_levi = Daemon {
        names: vec![
            String::from("Matthat"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(levi_son_of_simeon)),
        mother: None,
    };

    let jorim = Daemon {
        names: vec![
            String::from("Jorim"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(matthat_son_of_levi)),
        mother: None,
    };

    let eliezer = Daemon {
        names: vec![
            String::from("Eliezer"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(jorim)),
        mother: None,
    };

    let joshua = Daemon {
        names: vec![
            String::from("Joshua"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(eliezer)),
        mother: None,
    };

    let er = Daemon {
        names: vec![
            String::from("Er"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(joshua)),
        mother: None,
    };

    let elmadam = Daemon {
        names: vec![
            String::from("Elmadam"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(er)),
        mother: None,
    };

    let cosam = Daemon {
        names: vec![
            String::from("Cosam"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(elmadam)),
        mother: None,
    };

    let addi = Daemon {
        names: vec![
            String::from("Addi"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(cosam)),
        mother: None,
    };

    let melki_son_of_addi = Daemon {
        names: vec![
            String::from("Melki"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(addi)),
        mother: None,
    };

    let neri = Daemon {
        names: vec![
            String::from("Neri"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(melki_son_of_addi)),
        mother: None,
    };

    let shealtiel = Daemon {
        names: vec![
            String::from("Shealtiel"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(neri)),
        mother: None,
    };

    let zerubbabel = Daemon {
        names: vec![
            String::from("Zerubbabel"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(shealtiel)),
        mother: None,
    };

    let rhesa = Daemon {
        names: vec![
            String::from("Rhesa"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(zerubbabel)),
        mother: None,
    };

    let joanan = Daemon {
        names: vec![
            String::from("Joanan"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(rhesa)),
        mother: None,
    };

    let joda = Daemon {
        names: vec![
            String::from("Joda"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(joanan)),
        mother: None,
    };

    let josek = Daemon {
        names: vec![
            String::from("Josek"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(joda)),
        mother: None,
    };

    let semein = Daemon {
        names: vec![
            String::from("Semein"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(josek)),
        mother: None,
    };

    let mattathias_son_of_semein = Daemon {
        names: vec![
            String::from("Mattathias"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(semein)),
        mother: None,
    };

    let maath = Daemon {
        names: vec![
            String::from("maath"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(mattathias_son_of_semein)),
        mother: None,
    };

    let naggai = Daemon {
        names: vec![
            String::from("Naggai"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(maath)),
        mother: None,
    };

    let esli = Daemon {
        names: vec![
            String::from("Esli"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(naggai)),
        mother: None,
    };

    let nahum = Daemon {
        names: vec![
            String::from("Nahum"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(esli)),
        mother: None,
    };

    let amos = Daemon {
        names: vec![
            String::from("Amos"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(nahum)),
        mother: None,
    };

    let mattathias = Daemon {
        names: vec![
            String::from("Mattathias"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(amos)),
        mother: None,
    };

    let joseph_son_of_mattathias = Daemon {
        names: vec![
            String::from("Joseph"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(mattathias)),
        mother: None,
    };

    let jannai = Daemon {
        names: vec![
            String::from("Jannai"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(joseph_son_of_mattathias)),
        mother: None,
    };

    let melki = Daemon {
        names: vec![
            String::from("Melki"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(jannai)),
        mother: None,
    };

    let levi = Daemon {
        names: vec![
            String::from("Levi"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(melki)),
        mother: None,
    };

    let matthat = Daemon {
        names: vec![
            String::from("Matthat"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(levi)),
        mother: None,
    };

    let heli = Daemon {
        names: vec![
            String::from("Heli"),
        ],
        words: vec![],
        deeds: vec![],

        father: Some(Box::new(matthat)),
        mother: None,
    };

    let joseph = Daemon {
        names: vec![
            String::from("Joseph"),
        ],
        words: vec![],
        deeds: vec![String::from("divorcing Mary without disgrace")],

        father: Some(Box::new(heli)),
        mother: None,
    };

    let jesus = Daemon {
        names: vec![
            String::from("Jesus"),
        ],
        words: vec![
            String::from("render Caesar what is Caesar's"),
        ],
        deeds: vec![
            String::from("walked on water"),
        ],

        father: Some(Box::new(joseph)),
        mother: Some(Box::new(mary)),
    };

    println!("{:?}", jesus);
}
