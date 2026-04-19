use deed::Deed;
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
	pub fn genealogy(self) {
	    let mut cur = self.father;
	    while let Some(node) = cur {
		// TODO(atec): probably some index check
		println!("{:#?}", node.names[0]);
		cur = node.father;
	    }
	}
}
