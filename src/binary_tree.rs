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
        let new_node = Box::new(Node {
            value,
            left: None,
            right: None,
        });
        if self.root.is_none() {
            self.root = Some(new_node);
            return;
        }
        let mut current_node = &mut self.root;
        loop {
            let node = current_node.as_mut().unwrap();
            if value < node.value {
                if node.left.is_none() {
                    node.left = Some(new_node);
                    return;
                } else {
                    current_node = &mut node.left;
                }
            } else {
                if node.right.is_none() {
                    node.right = Some(new_node);
                    return;
                } else {
                    current_node = &mut node.right;
                }
            }
        }
    }

    //breadth first search
    //tree format is
    //     5
    //    / \
    //   3   7
    //  / \   \
    // 2   4  10
    fn bfs(&self) -> Option<Vec<i32>> {
        let mut queue = Vec::new();
        let mut ordered_result = Vec::new();
        match &self.root {
            None => return None,
            Some(node) => queue.push(node),
        }
        while !queue.is_empty() {
            let node = queue.remove(0);
            ordered_result.push(node.value);
            if let Some(left) = &node.left {
                queue.push(left);
            }
            if let Some(right) = &node.right {
                queue.push(right);
            }
        }
        Some(ordered_result)
    }

    fn dfs(&self) -> Option<Vec<i32>> {
        let mut stack = Vec::new();
        let mut ordered_result = Vec::new();
        match &self.root {
            None => return None,
            Some(node) => stack.push(node),
        }
        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            ordered_result.push(node.value);
            if let Some(right) = &node.right {
                stack.push(right);
            }
            if let Some(left) = &node.left {
                stack.push(left);
            }
        }
        Some(ordered_result)
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

    #[test]
    fn test_bfs() {
        let mut tree = BinaryTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(2);
        tree.insert(10);
        tree.insert(4);
        let bfs_result = tree.bfs();
        assert_eq!(bfs_result, Some(vec![5, 3, 7, 2, 4, 10]));
    }

    #[test]
    fn test_dfs() {
        let mut tree = BinaryTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(2);
        tree.insert(10);
        tree.insert(4);
        let dfs_result = tree.dfs();
        assert_eq!(dfs_result, Some(vec![5, 3, 2, 4, 7, 10]));
    }
}
