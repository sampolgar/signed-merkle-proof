mod binary_tree;
mod hello;
mod merkle_tree;

pub use hello::hello;
// pub use prebuilt_merkletree::CBMT;
// pub use merkle_tree::MerkleTree;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello, world!");
    }

    // #[test]
    // fn test_merkle_tree() {
    //     let mut tree = MerkleTree::new();
    //     let hash = tree.commit("Hello, world!".to_string());
    //     assert_eq!(
    //         hash,
    //         "2c74fd17edafd80e8447b0d46741ee243b7eb74dd2149a0ab1b9246fb30382f6"
    //     );
    // }
}
