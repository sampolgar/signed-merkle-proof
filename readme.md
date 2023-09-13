This is a project to implement signed merkle tree proofs in rust according to [https://www.ietf.org/archive/id/draft-steele-cose-merkle-tree-proofs-01.html#name-proof-types-registry](IETF draft here)
Which uses merkle tree primitives from [https://datatracker.ietf.org/doc/html/rfc9162#name-data-structures](RFC 9162) and [here](https://github.com/transmute-industries/rfc9162/)

### First, developing intuition

#### Step 1: Add Personal Values to Merkle Tree

1. **Hashing**: Hash the user's personal values (e.g., age, name, membership status) using a secure hash function.
2. **Merkle Tree Update**: Add this hash as a leaf node to the Merkle Tree.
3. **Root Update**: Update the Merkle Tree and generate a new Merkle Root.

#### Step 2: User Given Merkle Tree Details

1. **Merkle Proof**: Provide the user with a Merkle Proof, which is the minimal set of nodes needed to compute the Merkle Root from their leaf node.
2. **Root Hash**: Optionally, give the user the current Merkle Root for verification purposes.

#### Step 3: Verifier Asks User to Prove Themselves

1. **Request for Proof**: The verifier asks the user to prove that they are indeed the owner of a leaf node in the Merkle Tree.

#### Step 4: User Proves Their Credentials

1. **Merkle Proof**: The user provides the Merkle Proof to show that their data is part of the Merkle Tree. This proves that they are part of the dataset without revealing which exact data is theirs.

By removing the commitment step, you simplify the process but also lose the benefits of commitment, such as the ability to hide the actual values and later reveal them in a verifiable manner. However, this could be a good starting point for you to get the basic functionality working before adding more advanced features like commitments and zero-knowledge proofs.
`
//https://medium.com/crypto-0-nite/merkle-proofs-explained-6dd429623dc5
//https://computersciencewiki.org/index.php/Merkle_proof
https://github.com/qbzzt/merkle-proofs-for-offline-data-integrity/blob/main/scripts/index.js#L65

// use hex;
// use sha2::{Digest, Sha256, Sha512};

// struct LeafNode {
// commitment: [u8; 32],
// }

// struct InternalNode {
// left: Box<TreeNode>,
// right: Box<TreeNode>,
// hash: [u8; 32],
// }

// enum TreeNode {
// Leaf(LeafNode),
// Internal(InternalNode),
// }

// pub struct MerkleTree {
// root: TreeNode,
// height: u8,
// leaf_count: u8,
// }

// impl MerkleTree {
// fn new() -> MerkleTree {
// MerkleTree {
// root: TreeNode::Leaf(LeafNode {
// commitment: [0; 32],
// }),
// height: 0,
// leaf_count: 0,
// }
// }

// // fn commit(&mut self, data: String) -> Vec<u8> {
// // let mut hasher = Sha256::new();
// // hasher.update(data.as_bytes());
// // let result = hasher.finalize();
// // self.add(result.clone());
// // result
// // }

// fn commit(&mut self, data: String) -> Vec<u8> {
// let mut hasher = Sha256::new();
// hasher.update(data.as_bytes());
// let result = hasher.finalize();
// let hash_string = hex::encode(result.clone());
// self.add(hash_string);
// result
// }

// fn add(&mut self, hash: String) {
// if let TreeNode::Leaf(ref mut leaf) = self.root {
// let mut node = leaf;
// loop {
// if let TreeNode::Leaf(ref mut right) = node.right {
// node = right;
// } else {
// break;
// }
// }
// node.right = Some(Box::new(Leaf {
// hash: hash.clone(),
// left: None,
// right: None,
// }));
// } else {
// self.root = Some(Box::new(Leaf {
// hash: hash.clone(),
// left: None,
// right: None,
// }));
// }
// }
// }
