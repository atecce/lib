uniffi::setup_scaffolding!();

use deed::Deed;
use deed::SwiftDeed;
use name::Name;
use source::Source;

pub trait Ancestry {
    fn father(&self) -> Option<Box<SwiftDaemon>>;
    fn genealogy(&self) {
        let mut cur = self.father();
        while let Some(node) = cur {
            // TODO(atec): probably some index check
            println!("{:#?}", node.names[0]);
            cur = node.father();
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Daemon<'a> {
    pub names: &'a [Name],
    pub words: &'a [Source],
    pub deeds: &'a [Deed<'a>],

    pub father: Option<&'a Daemon<'a>>,
    pub mother: Option<&'a Daemon<'a>>,
    pub teacher: Option<&'a Daemon<'a>>,

    pub predecessor: Option<&'a Daemon<'a>>,
}

impl Ancestry for Daemon<'_> {
    fn father(&self) -> Option<Box<SwiftDaemon>> {
        self.father.and_then(|f| f.new())
    }
}

impl Daemon<'_> {
    pub fn new(&self) -> Option<Box<SwiftDaemon>> {
        Some(Box::new(SwiftDaemon {
            names: self.names.to_vec(),
            words: self.words.to_vec(),
            deeds: self.deeds_to_vec(),
            father: self.father(),
            mother: self.mother(),
            teacher: self.teacher(),
            predecessor: self.predecessor(),
        }))
    }

    fn mother(self) -> Option<Box<SwiftDaemon>> {
        if let Some(mother) = self.mother {
            let ret = *mother;
            ret.new()
        } else {
            None
        }
    }

    fn teacher(self) -> Option<Box<SwiftDaemon>> {
        if let Some(teacher) = self.teacher {
            let ret = *teacher;
            ret.new()
        } else {
            None
        }
    }

    fn predecessor(self) -> Option<Box<SwiftDaemon>> {
        if let Some(predecessor) = self.predecessor {
            let ret = *predecessor;
            ret.new()
        } else {
            None
        }
    }

    fn deeds_to_vec(self) -> Vec<SwiftDeed> {
        let mut swift_deeds = Vec::new();
        for deed in self.deeds {
            swift_deeds.push(Deed::new(*deed));
        }
        swift_deeds
    }
}

#[derive(Clone, Debug, uniffi::Object)]
pub struct SwiftDaemon {
    pub names: Vec<Name>,
    pub words: Vec<Source>,
    pub deeds: Vec<SwiftDeed>,

    pub father: Option<Box<SwiftDaemon>>,
    pub mother: Option<Box<SwiftDaemon>>,
    pub teacher: Option<Box<SwiftDaemon>>,

    pub predecessor: Option<Box<SwiftDaemon>>,
}

impl Ancestry for SwiftDaemon {
    fn father(&self) -> Option<Box<SwiftDaemon>> {
        self.father.clone()
    }
}
