mod binary_tree;
mod hello;
mod merkle_tree;
mod my_hash;

pub use hello::hello;
pub use my_hash::simple_hash;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello, world!");
    }
}
