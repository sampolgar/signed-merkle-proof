// create merkle tree
//tree format is
//           R
//        /      \
//      P1        P2
//    /   \      /   \
//   Q1   Q2    Q3    Q4
//  /\    / \   / \   / \
// D1 D2 D3 D4 D5 D6 D7 D8
// Where D is a hash of a data block
use crate::my_hash::simple_2to1;
use crate::my_hash::simple_hash;

#[derive(Clone)]
struct Node {
    hash: [u16; 16],
}

struct MerkleTree {
    root: Option<Box<Node>>,
    leaves: Vec<Node>,
}

impl MerkleTree {
    //create new binary tree
    pub fn new() -> Self {
        MerkleTree {
            root: None,
            leaves: vec![],
        }
    }

    //create a new node struct with a value, add it to the tree
    fn insert(&mut self, hash: [u16; 16]) {
        let new_node = Node { hash };
        self.leaves.push(new_node);
        self.update_root();
    }

    fn update_root(&mut self) {
        if self.leaves.is_empty() {
            self.root = None;
            return;
        }

        // Start with leaves
        let mut current_level = self.leaves.clone();

        //keep hashing pairs of nodes until the root remains
        while current_level.len() > 1 {
            let mut next_level = Vec::new();

            //take pairs of nodes
            for pair in current_level.chunks(2) {
                //if there is only one node left, duplicate it
                let node1 = &pair[0];
                let node2 = if pair.len() > 1 { &pair[1] } else { &pair[0] };

                //hash the pair of nodes
                let hash_pair = simple_2to1(&node1.hash, &node2.hash);

                //create a new node with the hash of the pair of nodes
                let new_node = Node { hash: hash_pair };

                //add the new node to the next level
                next_level.push(new_node);
            }

            //set the next level as the current level
            current_level = next_level.split_off(0);
        }
        self.root = Some(Box::new(current_level[0].clone()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4_insert() {
        let mut tree = MerkleTree::new();
        tree.insert(simple_hash("Data1"));
        tree.insert(simple_hash("Data2"));
        tree.insert(simple_hash("Data3"));
        tree.insert(simple_hash("Data4"));
        tree.insert(simple_hash("Data5"));
        tree.insert(simple_hash("Data6"));
        tree.insert(simple_hash("Data7"));
        tree.insert(simple_hash("Data8"));
        assert_eq!(
            tree.root.as_ref().unwrap().hash,
            simple_2to1(
                &simple_2to1(
                    &simple_2to1(&simple_hash("Data1"), &simple_hash("Data2")),
                    &simple_2to1(&simple_hash("Data3"), &simple_hash("Data4"))
                ),
                &simple_2to1(
                    &simple_2to1(&simple_hash("Data5"), &simple_hash("Data6")),
                    &simple_2to1(&simple_hash("Data7"), &simple_hash("Data8"))
                )
            )
        );
    }
}

//     fn test_3_insert() {
//         let mut tree = BinaryTree::new();
//         tree.insert(5);
//         tree.insert(3);
//         tree.insert(7);
//         assert_eq!(tree.root.as_ref().unwrap().value, 5);
//         assert_eq!(tree.root.as_ref().unwrap().left.as_ref().unwrap().value, 3);
//         assert_eq!(tree.root.as_ref().unwrap().right.as_ref().unwrap().value, 7);
//     }
// }
