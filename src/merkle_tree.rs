struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

struct BinaryTree {
    root: Option<Box<Node>>,
}

impl BinaryTree {
    //create new binary tree
    fn new() -> BinaryTree {
        BinaryTree { root: None }
    }
    //insert value into binary tree
    //create a new node struct with a value, add it to the tree
    fn insert(&mut self, value: i32) {
        let new_node = Node {
            value,
            left: None,
            right: None,
        };
        match self.root {
            None => self.root = Some(Box::new(new_node)),
            Some(ref mut node) => {
                if value < node.value {
                    node.left = Some(Box::new(new_node));
                } else {
                    node.right = Some(Box::new(new_node));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3_insert() {
        let mut tree = BinaryTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        assert_eq!(tree.root.as_ref().unwrap().value, 5);
        assert_eq!(tree.root.as_ref().unwrap().left.as_ref().unwrap().value, 3);
        assert_eq!(tree.root.as_ref().unwrap().right.as_ref().unwrap().value, 7);
    }
}
