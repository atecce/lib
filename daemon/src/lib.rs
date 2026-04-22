uniffi::setup_scaffolding!();

use deed::Deed;
use deed::BoxDeed;
use name::Name;
use source::Source;

pub trait Ancestry {
    fn father(&self) -> Option<Box<BoxDaemon>>;
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
    fn father(&self) -> Option<Box<BoxDaemon>> {
        self.father.and_then(|f| f.new())
    }
}

impl Daemon<'_> {
    pub fn new(&self) -> Option<Box<BoxDaemon>> {
        Some(Box::new(BoxDaemon {
            names: self.names.to_vec(),
            words: self.words.to_vec(),
            deeds: self.deeds_to_vec(),
            father: self.father(),
            mother: self.mother(),
            teacher: self.teacher(),
            predecessor: self.predecessor(),
        }))
    }

    fn mother(self) -> Option<Box<BoxDaemon>> {
	self.mother.and_then(|m| m.new())
    }

    fn teacher(self) -> Option<Box<BoxDaemon>> {
	self.teacher.and_then(|t| t.new())
    }

    fn predecessor(self) -> Option<Box<BoxDaemon>> {
	self.predecessor.and_then(|p| p.new())
    }

    fn deeds_to_vec(self) -> Vec<BoxDeed> {
        let mut swift_deeds = Vec::new();
        for deed in self.deeds {
            swift_deeds.push(Deed::new(*deed));
        }
        swift_deeds
    }
}

#[derive(Clone, Debug, uniffi::Object)]
pub struct BoxDaemon {
    pub names: Vec<Name>,
    pub words: Vec<Source>,
    pub deeds: Vec<BoxDeed>,

    pub father: Option<Box<BoxDaemon>>,
    pub mother: Option<Box<BoxDaemon>>,
    pub teacher: Option<Box<BoxDaemon>>,

    pub predecessor: Option<Box<BoxDaemon>>,
}

impl Ancestry for BoxDaemon {
    fn father(&self) -> Option<Box<BoxDaemon>> {
        self.father.clone()
    }
}
