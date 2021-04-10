fn main() {
    println!("Hello, world!");
}

use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub struct TreeNode {
    pub value: Option<i32>,
    pub left_child: Option<Box<TreeNode>>,
    pub right_child: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(value: i32) -> TreeNode {
        TreeNode {
            value: Some(value),
            left_child: None,
            right_child: None,
        }
    }

    pub fn contains(&self, searched_value: i32) -> bool {
        match &self.value {
            Some(value) => match value.cmp(&searched_value) {
                Ordering::Equal => true,
                Ordering::Greater => match &self.left_child {
                    Some(left_child) => left_child.contains(searched_value),
                    None => false,
                },
                Ordering::Less => match &self.right_child {
                    Some(right_child) => right_child.contains(searched_value),
                    None => false,
                },
            },
            None => false,
        }
    }

    pub fn insert(&mut self, value_to_be_inserted: i32) {
        match self.value {
            Some(value) => match value_to_be_inserted.cmp(&value) {
                Ordering::Less => match &mut self.left_child {
                    Some(left_child) => left_child.insert(value_to_be_inserted),
                    None => self.left_child = Some(Box::new(TreeNode::new(value_to_be_inserted))),
                },
                Ordering::Greater => match &mut self.right_child {
                    Some(right_child) => right_child.insert(value_to_be_inserted),
                    None => self.right_child = Some(Box::new(TreeNode::new(value_to_be_inserted))),
                },
                Ordering::Equal => (),
            },
            None => self.value = Some(value_to_be_inserted),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_returns_a_tree_node_with_the_created_value_wrapped_in_the_option_enum() {
        let bst = TreeNode::new(100);
        let expected = TreeNode {
            value: Some(100),
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
            value: Some(3),
            left_child: Some(Box::new(TreeNode {
                value: Some(2),
                left_child: None,
                right_child: None,
            })),
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
            value: Some(50),
            left_child: Some(Box::new(TreeNode {
                value: Some(25),
                left_child: Some(Box::new(TreeNode {
                    value: Some(10),
                    left_child: None,
                    right_child: None,
                })),
                right_child: Some(Box::new(TreeNode {
                    value: Some(33),
                    left_child: None,
                    right_child: None,
                })),
            })),
            right_child: Some(Box::new(TreeNode {
                value: Some(75),
                left_child: Some(Box::new(TreeNode {
                    value: Some(56),
                    left_child: None,
                    right_child: None,
                })),
                right_child: Some(Box::new(TreeNode {
                    value: Some(89),
                    left_child: None,
                    right_child: None,
                })),
            })),
        };

        assert_eq!(bst, expected);
    }
}
