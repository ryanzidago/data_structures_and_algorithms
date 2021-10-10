// an ok singly-linked stack

#[derive(Debug, PartialEq)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, PartialEq)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current_link = self.head.take();

        while let Some(mut boxed_node) = current_link {
            current_link = boxed_node.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_returs_an_empty_list() {
        let new_list: List<i32> = List::new();
        assert_eq!(new_list, List { head: None });
    }

    #[test]
    fn push_prepend_an_element_to_the_list() {
        let mut list: List<i32> = List::new();
        list.push(10);

        assert_eq!(
            List {
                head: Link::Some(Box::new(Node {
                    elem: 10,
                    next: Link::None,
                }),),
            },
            list
        );

        list.push(20);

        assert_eq!(
            List {
                head: Link::Some(Box::new(Node {
                    elem: 20,
                    next: Link::Some(Box::new(Node {
                        elem: 10,
                        next: Link::None,
                    }),),
                }),),
            },
            list
        )
    }

    #[test]
    fn pop_deletes_the_last_element_from_the_list_and_returns_it_wrapped_in_an_option() {
        let mut list: List<i32> = List::new();

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
        let mut list: List<String> = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push("Alice".to_string());
        list.push("Bob".to_string());
        list.push("Jean".to_string());

        // Check normal removal
        assert_eq!(list.pop(), Some("Jean".to_string()));
        assert_eq!(list.pop(), Some("Bob".to_string()));

        // Push some more just to make sure nothing's corrupted
        list.push("Julie".to_string());
        list.push("Marie".to_string());

        // Check normal removal
        assert_eq!(list.pop(), Some("Marie".to_string()));
        assert_eq!(list.pop(), Some("Julie".to_string()));

        // Check exhaustion
        assert_eq!(list.pop(), Some("Alice".to_string()));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek_returns_a_reference_to_the_head_of_the_list_wrapped_in_an_option() {
        let mut list: List<String> = List::new();

        list.push("Alice".to_string());
        list.push("Bob".to_string());

        assert_eq!(Some(&"Bob".to_string()), list.peek());

        list.pop();

        assert_eq!(Some(&"Alice".to_string()), list.peek());

        list.pop();

        assert_eq!(None, list.peek());
    }

    #[test]
    fn peek_mut_returns_a_mutable_reference_to_the_head_of_the_list_wrapped_in_an_option() {
        let mut list: List<String> = List::new();

        list.push("Alice".to_string());
        list.push("Bob".to_string());

        let alice: &mut String = list.peek_mut().unwrap();
        alice.make_ascii_uppercase();

        assert_eq!(&"BOB".to_string(), list.peek().unwrap());
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();

        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn nth() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();

        assert_eq!(iter.nth(0), Some(3));
        assert_eq!(iter.nth(0), Some(2));
        assert_eq!(iter.nth(0), Some(1));

        assert_eq!(iter.nth(0), None);
    }

    #[test]
    fn fold() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let iter = list.into_iter();
        let sum = iter.fold(0, |acc, x| acc + x);

        assert_eq!(sum, 6);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();

        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
