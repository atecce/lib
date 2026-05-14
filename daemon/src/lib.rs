uniffi::setup_scaffolding!();

use std::clone::Clone;
use std::sync::Arc;

use citation::Citation;
use deed::Deed;
use name::Name;

#[derive(Clone, Copy, Debug)]
pub struct Daemon<'a, C> {
    pub names: &'a [Name],
    pub words: &'a [C],
    pub deeds: &'a [Deed<'a, C>],

    pub father: Option<&'a Daemon<'a, C>>,
    pub mother: Option<&'a Daemon<'a, C>>,
    pub teacher: Option<&'a Daemon<'a, C>>,

    pub predecessor: Option<&'a Daemon<'a, C>>,
}

impl<C: Citation> Daemon<'_, C> {
    pub fn new_arc(&self) -> Option<Arc<ArcDaemon>> {
        Some(Arc::new(ArcDaemon {
            names: self.names.to_vec(),
            words: self.words.into_iter().map(|word| word.cite()).collect(),
//            deeds: self.deeds.to_vec(),
            father: self.arc_father(),
            mother: self.arc_mother(),
            teacher: self.arc_teacher(),
            predecessor: self.arc_predecessor(),
        }))
    }

    pub fn new_box(&self) -> Option<Box<BoxDaemon>> {
        Some(Box::new(BoxDaemon {
            names: self.names.to_vec(),
            words: self.words.into_iter().map(|word| word.cite()).collect(),
//            deeds: self.deeds.to_vec(),
            father: self.box_father(),
            mother: self.box_mother(),
            teacher: self.box_teacher(),
            predecessor: self.box_predecessor(),
        }))
    }

    fn arc_father(&self) -> Option<Arc<ArcDaemon>> {
        self.father.and_then(|f| f.new_arc())
    }

    fn arc_mother(&self) -> Option<Arc<ArcDaemon>> {
        self.mother.and_then(|m| m.new_arc())
    }

    fn arc_teacher(&self) -> Option<Arc<ArcDaemon>> {
        self.teacher.and_then(|t| t.new_arc())
    }

    fn arc_predecessor(&self) -> Option<Arc<ArcDaemon>> {
        self.predecessor.and_then(|p| p.new_arc())
    }

    fn box_father(&self) -> Option<Box<BoxDaemon>> {
        self.father.and_then(|f| f.new_box())
    }

    fn box_mother(&self) -> Option<Box<BoxDaemon>> {
        self.mother.and_then(|m| m.new_box())
    }

    fn box_teacher(&self) -> Option<Box<BoxDaemon>> {
        self.teacher.and_then(|t| t.new_box())
    }

    fn box_predecessor(&self) -> Option<Box<BoxDaemon>> {
        self.predecessor.and_then(|p| p.new_box())
    }
}

#[derive(Clone, Debug, uniffi::Object)]
pub struct ArcDaemon {
    pub names: Vec<Name>,
    pub words: Vec<String>,
//    pub deeds: Vec<Deed<C>>,

    pub father: Option<Arc<ArcDaemon>>,
    pub mother: Option<Arc<ArcDaemon>>,
    pub teacher: Option<Arc<ArcDaemon>>,

    pub predecessor: Option<Arc<ArcDaemon>>,
}

#[uniffi::export]
impl ArcDaemon {
    fn father(&self) -> Option<Arc<ArcDaemon>> {
        self.father.clone()
    }
    pub fn names(&self) -> Vec<Name> {
        self.names.clone()
    }
    pub fn words(&self) -> Vec<String> {
        self.words.clone()
    }
//    pub fn deeds(&self) -> Vec<Deed<C>> {
//        self.deeds.clone()
//    }
}

#[derive(Clone, Debug, uniffi::Object)]
pub struct BoxDaemon {
    pub names: Vec<Name>,
    pub words: Vec<String>,
//    pub deeds: Vec<Deed<'_, C>>,

    pub father: Option<Box<BoxDaemon>>,
    pub mother: Option<Box<BoxDaemon>>,
    pub teacher: Option<Box<BoxDaemon>>,

    pub predecessor: Option<Box<BoxDaemon>>,
}

#[uniffi::export]
impl BoxDaemon {
    pub fn names(&self) -> Vec<Name> {
        self.names.clone()
    }
    pub fn words(&self) -> Vec<String> {
        self.words.clone()
    }
//    pub fn deeds(&self) -> Vec<Deed<C>> {
//        self.deeds.clone()
//    }
}
