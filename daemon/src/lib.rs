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
        self.father.and_then(|f| f.new_arc())
    }
}

impl Daemon<'_> {
    pub fn new_arc(&self) -> Option<Arc<ArcDaemon>> {
        Some(Arc::new(ArcDaemon {
            names: self.names.to_vec(),
            words: self.words.to_vec(),
            deeds: self.deeds_to_vec(),
            father: self.father(),
            mother: self.arc_mother(),
            teacher: self.arc_teacher(),
            predecessor: self.arc_predecessor(),
        }))
    }

    pub fn new_box(&self) -> Option<Box<BoxDaemon>> {
        Some(Box::new(BoxDaemon {
            names: self.names.to_vec(),
            words: self.words.to_vec(),
            deeds: self.deeds_to_vec(),
            father: self.box_father(),
            mother: self.box_mother(),
            teacher: self.box_teacher(),
            predecessor: self.box_predecessor(),
        }))
    }

    fn arc_mother(self) -> Option<Arc<ArcDaemon>> {
        self.mother.and_then(|m| m.new_arc())
    }

    fn arc_teacher(self) -> Option<Arc<ArcDaemon>> {
        self.teacher.and_then(|t| t.new_arc())
    }

    fn arc_predecessor(self) -> Option<Arc<ArcDaemon>> {
        self.predecessor.and_then(|p| p.new_arc())
    }

    fn box_father(self) -> Option<Box<BoxDaemon>> {
        self.mother.and_then(|f| f.new_box())
    }

    fn box_mother(self) -> Option<Box<BoxDaemon>> {
        self.mother.and_then(|m| m.new_box())
    }

    fn box_teacher(self) -> Option<Box<BoxDaemon>> {
        self.teacher.and_then(|t| t.new_box())
    }

    fn box_predecessor(self) -> Option<Box<BoxDaemon>> {
        self.predecessor.and_then(|p| p.new_box())
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

#[uniffi::export]
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
