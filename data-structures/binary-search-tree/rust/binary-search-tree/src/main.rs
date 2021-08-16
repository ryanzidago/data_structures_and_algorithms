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

    pub fn to_sexprs(root: TreeNode) -> String {
        _to_sexpres(Some(Rc::new(RefCell::new(root))))
    }

    fn _to_sexpres(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        match root {
            None => "()".to_string(),
            Some(root) => format!(
                "({} {} {})",
                root.borrow().value,
                _to_sexpres(root.borrow().left_child.clone()),
                _to_sexpres(root.borrow().right_child.clone())
            ),
        }
    }

    pub fn serialize(root: TreeNode) -> String {
        _serialize(Some(Rc::new(RefCell::new(root))))
    }

    fn _serialize(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        match root {
            None => "#".to_string(),
            Some(root) => format!(
                "{} {} {}",
                root.borrow().value,
                _serialize(root.borrow().left_child.clone()),
                _serialize(root.borrow().right_child.clone())
            ),
        }
    }

    // "10 5 3 # # 7 # # 15 12 # # 18 # #"
    pub fn deserialize(string: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut iter = string.split_whitespace();
        let val = iter.next()?.parse::<i32>().ok()?;
        let mut bst = TreeNode::new(val);
        for value in iter {
            if value != "#" {
                let value = value.parse::<i32>().ok().unwrap();
                bst.insert(value);
            }
        }

        Some(Rc::new(RefCell::new(bst)))
    }

    pub fn invert(root: TreeNode) -> TreeNode {
        _invert(Some(Rc::new(RefCell::new(root))))
            .unwrap()
            .borrow()
            .clone()
    }

    fn _invert(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let node = root?;

        let left_child = node.borrow().left_child.clone();
        let right_child = node.borrow().right_child.clone();

        node.borrow_mut().left_child = _invert(right_child);
        node.borrow_mut().right_child = _invert(left_child);
        return Some(node);
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

    // uses breadth-first search
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
    fn contains_return_false_if_binary_search_tree_does_not_contain_the_searched_value() {
        let bst = TreeNode::new(100);
        assert!(!bst.contains(1));
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
    fn to_sexprs_serizalizes_a_bst_to_a_sexpres() {
        let bst = TreeNode {
            value: 2,
            left_child: Some(Rc::new(RefCell::new(TreeNode {
                value: 1,
                left_child: None,
                right_child: None,
            }))),
            right_child: Some(Rc::new(RefCell::new(TreeNode {
                value: 3,
                left_child: None,
                right_child: None,
            }))),
        };

        let sexprs = crate::binary_search_tree::to_sexprs(bst);
        assert_eq!(sexprs, "(2 (1 () ()) (3 () ()))".to_string());
    }

    #[test]
    fn serialize_converts_a_bst_to_a_string_representation() {
        let mut bst = TreeNode::new(2);
        bst.insert(1);
        bst.insert(3);

        let serialized_bst = crate::binary_search_tree::serialize(bst);
        assert_eq!(serialized_bst, "2 1 # # 3 # #".to_string());

        let mut bst = TreeNode::new(10);
        bst.insert(5);
        bst.insert(15);
        bst.insert(3);
        bst.insert(7);
        bst.insert(12);
        bst.insert(18);

        let serialized_bst = crate::binary_search_tree::serialize(bst);
        assert_eq!(
            serialized_bst,
            "10 5 3 # # 7 # # 15 12 # # 18 # #".to_string()
        );
    }

    #[test]
    fn deserialize_convert_a_string_representing_a_bst_to_a_bst() {
        let mut expected_bst = TreeNode::new(10);
        expected_bst.insert(5);
        expected_bst.insert(15);
        expected_bst.insert(3);
        expected_bst.insert(7);
        expected_bst.insert(12);
        expected_bst.insert(18);

        let deserialized_bst =
            crate::binary_search_tree::deserialize("10 5 3 # # 7 # # 15 12 # # 18 # #".to_string());

        assert_eq!(deserialized_bst, Some(Rc::new(RefCell::new(expected_bst))));
    }

    #[test]
    fn deserialize_on_an_empty_string() {
        assert_eq!(None, crate::binary_search_tree::deserialize("".to_string()));
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

        bst = crate::binary_search_tree::invert(bst);

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
