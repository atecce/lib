uniffi::setup_scaffolding!();
use std::sync::Arc;

use deed::Deed;
use deed::SwiftDeed;
use name::Name;
use source::Source;

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

impl Daemon<'_> {
    fn new(&self) -> Option<Arc<ArcDaemon>> {
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

    fn father(self) -> Option<Arc<ArcDaemon>> {
        if let Some(father) = self.father {
            let ret = *father;
            ret.new()
        } else {
            None
        }
    }

    fn mother(self) -> Option<Arc<ArcDaemon>> {
        if let Some(mother) = self.mother {
            let ret = *mother;
            ret.new()
        } else {
            None
        }
    }

    fn teacher(self) -> Option<Arc<ArcDaemon>> {
        if let Some(teacher) = self.teacher {
            let ret = *teacher;
            ret.new()
        } else {
            None
        }
    }

    fn predecessor(self) -> Option<Arc<ArcDaemon>> {
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

    pub fn genealogy(self) {
        let mut cur = self.father;
        while let Some(node) = cur {
            // TODO(atec): probably some index check
            println!("{:#?}", node.names[0]);
            cur = node.father;
        }
    }
}

#[derive(uniffi::Object)]
pub struct ArcDaemon {
    pub names: Vec<Name>,
    pub words: Vec<Source>,
    pub deeds: Vec<SwiftDeed>,

    pub father: Option<Arc<ArcDaemon>>,
    pub mother: Option<Arc<ArcDaemon>>,
    pub teacher: Option<Arc<ArcDaemon>>,

    pub predecessor: Option<Arc<ArcDaemon>>,
}
