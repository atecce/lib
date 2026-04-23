uniffi::setup_scaffolding!();

use std::sync::Arc;

use deed::BoxDeed;
use deed::Deed;
use name::Name;
use source::Source;

#[uniffi::export]
pub trait Ancestry: Send + Sync {
    fn father(&self) -> Option<Arc<ArcDaemon>>;
}

#[uniffi::export]
pub fn genealogy(daemon: Arc<dyn Ancestry>) {
    let mut cur = daemon.father();
    while let Some(node) = cur {
        // TODO(atec): probably some index check
        println!("{:#?}", node.names[0]);
        cur = node.father();
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
    fn father(&self) -> Option<Arc<ArcDaemon>> {
        self.father.and_then(|f| f.new())
    }
}

impl Daemon<'_> {
    pub fn new(&self) -> Option<Arc<ArcDaemon>> {
        Some(Arc::new(ArcDaemon {
            names: self.names.to_vec(),
            words: self.words.to_vec(),
            deeds: self.deeds_to_vec(),
            father: self.father(),
            mother: self.mother(),
            teacher: self.teacher(),
            predecessor: self.predecessor(),
        }))
    }

    fn mother(self) -> Option<Arc<ArcDaemon>> {
        self.mother.and_then(|m| m.new())
    }

    fn teacher(self) -> Option<Arc<ArcDaemon>> {
        self.teacher.and_then(|t| t.new())
    }

    fn predecessor(self) -> Option<Arc<ArcDaemon>> {
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

// impl Ancestry for BoxDaemon {
//     fn father(&self) -> Option<Box<BoxDaemon>> {
//         self.father.clone()
//     }
// }

#[uniffi::export]
impl BoxDaemon {
    pub fn names(&self) -> Vec<Name> {
        self.names.clone()
    }
}

#[derive(Clone, Debug, uniffi::Object)]
pub struct ArcDaemon {
    pub names: Vec<Name>,
    pub words: Vec<Source>,
    pub deeds: Vec<BoxDeed>,

    pub father: Option<Arc<ArcDaemon>>,
    pub mother: Option<Arc<ArcDaemon>>,
    pub teacher: Option<Arc<ArcDaemon>>,

    pub predecessor: Option<Arc<ArcDaemon>>,
}

impl Ancestry for ArcDaemon {
    fn father(&self) -> Option<Arc<ArcDaemon>> {
        self.father.clone()
    }
}

#[uniffi::export]
impl ArcDaemon {
    pub fn names(&self) -> Vec<Name> {
        self.names.clone()
    }
}
