// Luke 3:23-38

#[derive(Debug)]
pub struct Daemon {
    names: &[&str],
    words: &[&str],
    deeds: &[&str],

    father: Option<Box<Daemon>>,
    mother: Option<Box<Daemon>>,
}

const almighty: Daemon = Daemon {
    names: &[
        "God",
    ],
    words: &[
        "text is the universal interface",
    ],
    deeds: &[
        "creation",
    ],

    father: None,
    mother: None,
};

const mary: Daemon = Daemon {
    names: &str[
        "Mary",
    ],
    words: &str[],
    deeds: &str[],

    father: None,
    mother: None,
};

const adam: Daemon = Daemon {
    names: &str[
        "Adam",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(almighty)),
    mother: None,
};

const seth: Daemon = Daemon {
    names: &str[
        "Seth",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(adam)),
    mother: None,
};

const enosh: Daemon = Daemon {
    names: &str[
        "Enosh",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(seth)),
    mother: None,
};

const kenan: Daemon = Daemon {
    names: &str[
        "Kenan",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(enosh)),
    mother: None,
};

const mahalalel: Daemon = Daemon {
    names: &str[
        "Mahalalel",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(kenan)),
    mother: None,
};

const jared: Daemon = Daemon {
    names: &str[
        "Jared",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(mahalalel)),
    mother: None,
};

const enoch: Daemon = Daemon {
    names: &str[
        "Enoch",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(jared)),
    mother: None,
};

const methuselah: Daemon = Daemon {
    names: &str[
        "Methuselah",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(enoch)),
    mother: None,
};

const lamech: Daemon = Daemon {
    names: &str[
        "Lamech",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(methuselah)),
    mother: None,
};

const noah: Daemon = Daemon {
    names: &str[
        "Noah",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(lamech)),
    mother: None,
};

const shem: Daemon = Daemon {
    names: &str[
        "Shem",
    ],
    words: [String;0];,
    deeds: [String;0];,

    father: Some(Box::new(noah)),
    mother: None,
};

const arphaxad: Daemon = Daemon {
    names: &str[
        String::from("Arphaxad"),
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(shem)),
    mother: None,
};

const cainan: Daemon = Daemon {
    names: &str[
        "Cainan",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(arphaxad)),
    mother: None,
};

const shelah: Daemon = Daemon {
    names: &str[
        "Eber",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(cainan)),
    mother: None,
};

const eber: Daemon = Daemon {
    names: &str[
        "Eber",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(shelah)),
    mother: None,
};

const peleg: Daemon = Daemon {
    names: &str[
        "Peleg",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(eber)),
    mother: None,
};

const reu: Daemon = Daemon {
    names: &str[
        "Reu",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(peleg)),
    mother: None,
};

const serug: Daemon = Daemon {
    names: &str[
        "Serug",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(reu)),
    mother: None,
};

const nahor: Daemon = Daemon {
    names: &str[
        "Nahor",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(serug)),
    mother: None,
};

const terrah: Daemon = Daemon {
    names: &str[
        "Terrah",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(nahor)),
    mother: None,
};

const abraham: Daemon = Daemon {
    names: &str[
        "Abraham",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(terrah)),
    mother: None,
};

const isaac: Daemon = Daemon {
    names: &str[
        "Isaac",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(abraham)),
    mother: None,
};

const jacob: Daemon = Daemon {
    names: &str[
        "Jacob",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(isaac)),
    mother: None,
};

const judah_son_of_jacob: Daemon = Daemon {
    names: &str[
        "Judah",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(jacob)),
    mother: None,
};

const perez: Daemon = Daemon {
    names: &str[
        "Perez",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(judah_son_of_jacob)),
    mother: None,
};

const hezron: Daemon = Daemon {
    names: &str[
        "Hezron",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(perez)),
    mother: None,
};

const ram: Daemon = Daemon {
    names: &str[
        "Ram",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(hezron)),
    mother: None,
};

const amminadab: Daemon = Daemon {
    names: &str[
        "Amminadab",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(ram)),
    mother: None,
};

const nashon: Daemon = Daemon {
    names: &str[
        "Nashon",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(amminadab)),
    mother: None,
};

const salmon: Daemon = Daemon {
    names: &str[
        "Salmon",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(nashon)),
    mother: None,
};

const boaz: Daemon = Daemon {
    names: &str[
        "Boaz",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(salmon)),
    mother: None,
};

const obed: Daemon = Daemon {
    names: &str[
        "Obed",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(boaz)),
    mother: None,
};

const jesse: Daemon = Daemon {
    names: &str[
        "Jesse",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(obed)),
    mother: None,
};

const david: Daemon = Daemon {
    names: &str[
        "David",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(jesse)),
    mother: None,
};

const nathan: Daemon = Daemon {
    names: &str[
        "Nathan",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(david)),
    mother: None,
};

const mattatha: Daemon = Daemon {
    names: &str[
        "Mattatha",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(nathan)),
    mother: None,
};

const menna: Daemon = Daemon {
    names: &str[
        "Menna",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(mattatha)),
    mother: None,
};

const melea: Daemon = Daemon {
    names: &str[
        "Melea",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(menna)),
    mother: None,
};

const eliakam: Daemon = Daemon {
    names: &str[
        "Eliakam",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(melea)),
    mother: None,
};

const jonam: Daemon = Daemon {
    names: &str[
        "Jonam",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(eliakam)),
    mother: None,
};

const joseph_son_of_jonam: Daemon = Daemon {
    names: &str[
        "Jonam",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(jonam)),
    mother: None,
};

const judah: Daemon = Daemon {
    names: &str[
        "Judah",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(joseph_son_of_jonam)),
    mother: None,
};

const simeon: Daemon = Daemon {
    names: &str[
        "Simeon",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(judah)),
    mother: None,
};

const levi_son_of_simeon: Daemon = Daemon {
    names: &str[
        "Levi",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(simeon)),
    mother: None,
};

const matthat_son_of_levi: Daemon = Daemon {
    names: &str[
        "Matthat",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(levi_son_of_simeon)),
    mother: None,
};

const jorim: Daemon = Daemon {
    names: &str[
        "Jorim",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(matthat_son_of_levi)),
    mother: None,
};

const eliezer: Daemon = Daemon {
    names: &str[
        "Eliezer",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(jorim)),
    mother: None,
};

const joshua: Daemon = Daemon {
    names: &str[
        "Joshua",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(eliezer)),
    mother: None,
};

const er: Daemon = Daemon {
    names: &str[
        "Er",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(joshua)),
    mother: None,
};

const elmadam: Daemon = Daemon {
    names: &str[
        "Elmadam",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(er)),
    mother: None,
};

const cosam: Daemon = Daemon {
    names: &str[
        "Cosam",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(elmadam)),
    mother: None,
};

const addi: Daemon = Daemon {
    names: &str[
        "Addi",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(cosam)),
    mother: None,
};

const melki_son_of_addi: Daemon = Daemon {
    names: &str[
        "Melki",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(addi)),
    mother: None,
};

const neri: Daemon = Daemon {
    names: &str[
        "Neri",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(melki_son_of_addi)),
    mother: None,
};

const shealtiel: Daemon = Daemon {
    names: &str[
        "Shealtiel",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(neri)),
    mother: None,
};

const zerubbabel: Daemon = Daemon {
    names: &str[
        "Zerubbabel",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(shealtiel)),
    mother: None,
};

const rhesa: Daemon = Daemon {
    names: &str[
        "Rhesa",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(zerubbabel)),
    mother: None,
};

const joanan: Daemon = Daemon {
    names: &str[
        "Joanan",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(rhesa)),
    mother: None,
};

const joda: Daemon = Daemon {
    names: &str[
        "Joda",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(joanan)),
    mother: None,
};

const josek: Daemon = Daemon {
    names: &str[
        "Josek",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(joda)),
    mother: None,
};

const semein: Daemon = Daemon {
    names: &str[
        "Semein",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(josek)),
    mother: None,
};

const mattathias_son_of_semein: Daemon = Daemon {
    names: &str[
        "Mattathias",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(semein)),
    mother: None,
};

const maath: Daemon = Daemon {
    names: &str[
        "maath",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(mattathias_son_of_semein)),
    mother: None,
};

const naggai: Daemon = Daemon {
    names: &str[
        "Naggai",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(maath)),
    mother: None,
};

const esli: Daemon = Daemon {
    names: &str[
        "Esli",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(naggai)),
    mother: None,
};

const nahum: Daemon = Daemon {
    names: &str[
        "Nahum",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(esli)),
    mother: None,
};

const amos: Daemon = Daemon {
    names: &str[
        "Amos",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(nahum)),
    mother: None,
};

const mattathias: Daemon = Daemon {
    names: &str[
        "Mattathias",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(amos)),
    mother: None,
};

const joseph_son_of_mattathias: Daemon = Daemon {
    names: &str[
        "Joseph",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(mattathias)),
    mother: None,
};

const jannai: Daemon = Daemon {
    names: &str[
        "Jannai",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(joseph_son_of_mattathias)),
    mother: None,
};

const melki: Daemon = Daemon {
    names: &str[
        "Melki",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(jannai)),
    mother: None,
};

const levi: Daemon = Daemon {
    names: &str[
        "Levi",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(melki)),
    mother: None,
};

const matthat: Daemon = Daemon {
    names: &str[
        "Matthat",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(levi)),
    mother: None,
};

const heli: Daemon = Daemon {
    names: &str[
        "Heli",
    ],
    words: &str[],
    deeds: &str[],

    father: Some(Box::new(matthat)),
    mother: None,
};

const joseph: Daemon = Daemon {
    names: &str[
        "Joseph",
    ],
    words: &str[],
    deeds: &str["divorcing Mary without disgrace"],

    father: Some(Box::new(heli)),
    mother: None,
};

const jesus: Daemon = Daemon {
    names: &str[
        "Jesus",
    ],
    words: &str[
        "render Caesar what is Caesar's",
    ],
    deeds: &str[
        "walked on water",
    ],

    father: Some(Box::new(joseph)),
    mother: Some(Box::new(mary)),
};
