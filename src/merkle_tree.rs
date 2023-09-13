


// use hex;
// use sha2::{Digest, Sha256, Sha512};

// struct LeafNode {
//     commitment: [u8; 32],
// }

// struct InternalNode {
//     left: Box<TreeNode>,
//     right: Box<TreeNode>,
//     hash: [u8; 32],
// }

// enum TreeNode {
//     Leaf(LeafNode),
//     Internal(InternalNode),
// }

// pub struct MerkleTree {
//     root: TreeNode,
//     height: u8,
//     leaf_count: u8,
// }

// impl MerkleTree {
//     fn new() -> MerkleTree {
//         MerkleTree {
//             root: TreeNode::Leaf(LeafNode {
//                 commitment: [0; 32],
//             }),
//             height: 0,
//             leaf_count: 0,
//         }
//     }

//     // fn commit(&mut self, data: String) -> Vec<u8> {
//     //     let mut hasher = Sha256::new();
//     //     hasher.update(data.as_bytes());
//     //     let result = hasher.finalize();
//     //     self.add(result.clone());
//     //     result
//     // }

//     fn commit(&mut self, data: String) -> Vec<u8> {
//         let mut hasher = Sha256::new();
//         hasher.update(data.as_bytes());
//         let result = hasher.finalize();
//         let hash_string = hex::encode(result.clone());
//         self.add(hash_string);
//         result
//     }

//     fn add(&mut self, hash: String) {
//         if let TreeNode::Leaf(ref mut leaf) = self.root {
//             let mut node = leaf;
//             loop {
//                 if let TreeNode::Leaf(ref mut right) = node.right {
//                     node = right;
//                 } else {
//                     break;
//                 }
//             }
//             node.right = Some(Box::new(Leaf {
//                 hash: hash.clone(),
//                 left: None,
//                 right: None,
//             }));
//         } else {
//             self.root = Some(Box::new(Leaf {
//                 hash: hash.clone(),
//                 left: None,
//                 right: None,
//             }));
//         }
//     }
// }
