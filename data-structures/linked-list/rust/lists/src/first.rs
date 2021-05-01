// a bad singly-linked stack
use std::mem;

// layout A:
//  pub enum List {
//      Empty,
//      Elem(i32, Box<List>),
//  }

// layout B:
#[derive(Debug, PartialEq)]
pub struct List {
    head: Link,
}

#[derive(Debug, PartialEq)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug, PartialEq)]
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut current_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = current_link {
            current_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_returs_an_empty_list() {
        assert_eq!(List::new(), List { head: Link::Empty });
    }

    #[test]
    fn push_prepend_an_element_to_the_list() {
        let mut list: List = List::new();
        list.push(10);

        assert_eq!(
            List {
                head: Link::More(Box::new(Node {
                    elem: 10,
                    next: Link::Empty,
                }),),
            },
            list
        );

        list.push(20);

        assert_eq!(
            List {
                head: Link::More(Box::new(Node {
                    elem: 20,
                    next: Link::More(Box::new(Node {
                        elem: 10,
                        next: Link::Empty,
                    }),),
                }),),
            },
            list
        )
    }

    #[test]
    fn pop_deletes_the_last_element_from_the_list_and_returns_it_wrapped_in_an_option() {
        let mut list: List = List::new();

        assert_eq!(None, list.pop());

        list.push(5);
        list.push(10);
        list.push(15);

        assert_eq!(Some(15), list.pop());
        assert_eq!(Some(10), list.pop());
        assert_eq!(Some(5), list.pop());
        assert_eq!(None, list.pop());
    }

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
