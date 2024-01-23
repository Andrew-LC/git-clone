
// 100644 blob 83baae61804e65cc73a7201a7252750c76066a30	test.txt
use std::fmt::{self, Display, Formatter};

pub struct Tree {
    pub mode: String,
    pub name: String,
    pub hash: String,
}

impl Tree {
    pub fn new(mode: String, name: String, hash: String) -> Self {
        Self { mode, name, hash }
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {}\t{}", self.mode, "blob", self.hash, self.name)
    }

    pub fn to_tree_object_string(&self) -> String {
        format!("{} {}\0", self.to_string(), self.hash)
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}\t{}", self.mode, "blob", self.hash, self.name)
    }
}
