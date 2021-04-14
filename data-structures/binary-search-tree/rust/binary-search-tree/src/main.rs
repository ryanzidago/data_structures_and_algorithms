fn main() {
    println!("Hello, world!");
}

pub mod binary_search_tree {
    use std::cell::RefCell;
    use std::cmp::Ordering;
    use std::collections::VecDeque;
    use std::rc::Rc;

    #[derive(Debug, PartialEq, Clone)]
    pub struct TreeNode {
        pub value: i32,
        pub left_child: Option<Rc<RefCell<TreeNode>>>,
        pub right_child: Option<Rc<RefCell<TreeNode>>>,
    }

    impl TreeNode {
        pub fn new(value: i32) -> TreeNode {
            TreeNode {
                value,
                left_child: None,
                right_child: None,
            }
        }

        pub fn contains(&self, searched_value: i32) -> bool {
            match self.value.cmp(&searched_value) {
                Ordering::Equal => true,
                Ordering::Greater => match &self.left_child {
                    Some(left_child) => left_child.borrow().contains(searched_value),
                    None => false,
                },
                Ordering::Less => match &self.right_child {
                    Some(right_child) => right_child.borrow().contains(searched_value),
                    None => false,
                },
            }
        }

        pub fn insert(&mut self, value_to_be_inserted: i32) {
            match value_to_be_inserted.cmp(&self.value) {
                Ordering::Less => match &mut self.left_child {
                    Some(left_child) => left_child.borrow_mut().insert(value_to_be_inserted),
                    None => {
                        self.left_child =
                            Some(Rc::new(RefCell::new(TreeNode::new(value_to_be_inserted))))
                    }
                },
                Ordering::Greater => match &mut self.right_child {
                    Some(right_child) => right_child.borrow_mut().insert(value_to_be_inserted),
                    None => {
                        self.right_child =
                            Some(Rc::new(RefCell::new(TreeNode::new(value_to_be_inserted))))
                    }
                },
                Ordering::Equal => (),
            }
        }
    }

    pub fn invert(root: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
        _invert(Some(Rc::new(RefCell::new(root))))
    }

    fn _invert(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let left_child = node.borrow().left_child.clone();
            let right_child = node.borrow().right_child.clone();

            node.borrow_mut().left_child = _invert(right_child);
            node.borrow_mut().right_child = _invert(left_child);
            return Some(node);
        }
        None
    }

    pub fn pre_order_traversal(root: TreeNode) -> Vec<i32> {
        _pre_order_traversal(Some(Rc::new(RefCell::new(root))), &mut Vec::new())
    }

    fn _pre_order_traversal(
        root: Option<Rc<RefCell<TreeNode>>>,
        result: &mut Vec<i32>,
    ) -> Vec<i32> {
        if let Some(node) = root {
            result.push(node.borrow().value);
            _pre_order_traversal(node.borrow().left_child.clone(), result);
            _pre_order_traversal(node.borrow().right_child.clone(), result);
        }

        result.clone()
    }

    pub fn in_order_traversal(root: TreeNode) -> Vec<i32> {
        _in_order_traversal(Some(Rc::new(RefCell::new(root))), &mut Vec::new())
    }

    fn _in_order_traversal(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) -> Vec<i32> {
        if let Some(node) = root {
            _in_order_traversal(node.borrow().left_child.clone(), result);
            result.push(node.borrow().value);
            _in_order_traversal(node.borrow().right_child.clone(), result);
        }

        result.clone()
    }

    pub fn post_order_traversal(root: TreeNode) -> Vec<i32> {
        _post_order_traversal(Some(Rc::new(RefCell::new(root))), &mut Vec::new())
    }

    fn _post_order_traversal(
        root: Option<Rc<RefCell<TreeNode>>>,
        result: &mut Vec<i32>,
    ) -> Vec<i32> {
        if let Some(node) = root {
            _post_order_traversal(node.borrow().left_child.clone(), result);
            _post_order_traversal(node.borrow().right_child.clone(), result);
            result.push(node.borrow().value)
        }

        result.clone()
    }

    pub fn level_order_traversal(root: TreeNode) -> Vec<Vec<i32>> {
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        let mut result: Vec<Vec<i32>> = Vec::new();

        queue.push_back(Rc::new(RefCell::new(root)));

        while !queue.is_empty() {
            let mut level: Vec<i32> = Vec::new();

            for _ in 0..queue.len() {
                if let Some(node) = queue.pop_front() {
                    level.push(node.borrow().value);

                    if let Some(left_child) = node.borrow().left_child.clone() {
                        queue.push_back(Rc::clone(&left_child));
                    }

                    if let Some(right_child) = node.borrow().right_child.clone() {
                        queue.push_back(Rc::clone(&right_child));
                    }
                }
            }

            result.push(level);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use crate::binary_search_tree::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn new_returns_a_tree_node_with_the_created_value_wrapped_in_the_option_enum() {
        let bst = TreeNode::new(100);
        let expected = TreeNode {
            value: 100,
            left_child: None,
            right_child: None,
        };

        assert_eq!(bst, expected);
    }

    #[test]
    fn contains_return_true_if_binary_search_tree_contains_the_searched_value() {
        let bst = TreeNode::new(100);
        assert!(bst.contains(100));
    }

    #[test]
    fn insert_inserts_a_value_into_a_binary_search_tree() {
        let mut bst = TreeNode::new(3);
        bst.insert(2);

        let expected = TreeNode {
            value: 3,
            left_child: Some(Rc::new(RefCell::new(TreeNode {
                value: 2,
                left_child: None,
                right_child: None,
            }))),
            right_child: None,
        };

        assert_eq!(bst, expected);

        let mut bst = TreeNode::new(50);
        bst.insert(25);
        bst.insert(75);
        bst.insert(10);
        bst.insert(56);
        bst.insert(33);
        bst.insert(89);

        let expected = TreeNode {
            value: 50,
            left_child: Some(Rc::new(RefCell::new(TreeNode {
                value: 25,
                left_child: Some(Rc::new(RefCell::new(TreeNode {
                    value: 10,
                    left_child: None,
                    right_child: None,
                }))),
                right_child: Some(Rc::new(RefCell::new(TreeNode {
                    value: 33,
                    left_child: None,
                    right_child: None,
                }))),
            }))),
            right_child: Some(Rc::new(RefCell::new(TreeNode {
                value: 75,
                left_child: Some(Rc::new(RefCell::new(TreeNode {
                    value: 56,
                    left_child: None,
                    right_child: None,
                }))),
                right_child: Some(Rc::new(RefCell::new(TreeNode {
                    value: 89,
                    left_child: None,
                    right_child: None,
                }))),
            }))),
        };

        assert_eq!(bst, expected);
    }

    #[test]
    fn invert_inverts_a_bst() {
        let expected = TreeNode {
            value: 4,
            left_child: Some(Rc::new(RefCell::new(TreeNode {
                value: 7,
                left_child: Some(Rc::new(RefCell::new(TreeNode {
                    value: 9,
                    left_child: None,
                    right_child: None,
                }))),
                right_child: Some(Rc::new(RefCell::new(TreeNode {
                    value: 6,
                    left_child: None,
                    right_child: None,
                }))),
            }))),
            right_child: Some(Rc::new(RefCell::new(TreeNode {
                value: 2,
                left_child: Some(Rc::new(RefCell::new(TreeNode {
                    value: 3,
                    left_child: None,
                    right_child: None,
                }))),
                right_child: Some(Rc::new(RefCell::new(TreeNode {
                    value: 1,
                    left_child: None,
                    right_child: None,
                }))),
            }))),
        };
        let mut bst = TreeNode::new(4);
        bst.insert(2);
        bst.insert(1);
        bst.insert(3);
        bst.insert(7);
        bst.insert(6);
        bst.insert(9);
        bst = crate::binary_search_tree::invert(bst)
            .unwrap()
            .borrow()
            .clone();

        assert_eq!(bst, expected);
    }

    #[test]
    fn pre_order_traversal_returns_a_vector_containing_the_bst_values_pre_ordered() {
        let expected = vec![3, 2, 7, 5, 9];
        let mut bst = TreeNode::new(3);
        bst.insert(2);
        bst.insert(7);
        bst.insert(5);
        bst.insert(9);

        let result = crate::binary_search_tree::pre_order_traversal(bst);
        assert_eq!(result, expected);
    }

    #[test]
    fn in_order_traversal_returns_a_vector_containing_the_bst_values_in_ordered() {
        let expected = vec![2, 3, 5, 7, 9];
        let mut bst = TreeNode::new(3);
        bst.insert(2);
        bst.insert(7);
        bst.insert(5);
        bst.insert(9);

        let result = crate::binary_search_tree::in_order_traversal(bst);
        assert_eq!(result, expected);
    }

    #[test]
    fn post_order_traversal_returns_a_vector_containing_the_bst_values_post_ordered() {
        let expected = vec![2, 5, 9, 7, 3];
        let mut bst = TreeNode::new(3);
        bst.insert(2);
        bst.insert(7);
        bst.insert(5);
        bst.insert(9);

        let result = crate::binary_search_tree::post_order_traversal(bst);
        assert_eq!(result, expected);
    }

    #[test]
    fn level_order_traversal_returns_a_vector_of_vector_containing_the_bst_values_in_level_ordered()
    {
        let expected = vec![vec![3], vec![2, 7], vec![5, 9]];
        let mut bst = TreeNode::new(3);
        bst.insert(2);
        bst.insert(7);
        bst.insert(5);
        bst.insert(9);

        let result = crate::binary_search_tree::level_order_traversal(bst);
        assert_eq!(result, expected);
    }
}
