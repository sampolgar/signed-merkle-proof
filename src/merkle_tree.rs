impl MerkleTree {
    //
    pub fn commit(&mut self, data: String) -> String {
        let hash = hash::hash(data);
        self.add(hash.clone());
        hash
    }

    // traverse tree to find where to add new leaf
    // move from left to right until we find a node with no right child
    // add new leaf as right child of that node
    fn add(&mut self, hash: String) {
        if let Some(ref mut root) = self.root {
            let mut node = root;
            loop {
                if let Some(ref mut right) = node.right {
                    node = right;
                } else {
                    break;
                }
            }
            node.right = Some(Box::new(Leaf {
                hash: hash.clone(),
                left: None,
                right: None,
            }));
        } else {
            self.root = Some(Box::new(Leaf {
                hash: hash.clone(),
                left: None,
                right: None,
            }));
        }
    }
}
