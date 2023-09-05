mod hash;
mod hello;
mod merkle_tree;
mod node;
mod proof;


pub struct Leaf {
    hash: String,
    left: Option<Box<Leaf>>,
    right: Option<Box<Leaf>>,
}

pub struct MerkleTree {
    root: Option<Leaf>,
}

impl MerkleTree {
    pub fn new() -> MerkleTree {
        MerkleTree { root: None }
    }

    // Add more methods here...
}
