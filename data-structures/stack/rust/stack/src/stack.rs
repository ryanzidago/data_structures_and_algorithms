#[derive(Debug, PartialEq)]
pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { data: vec![] }
    }

    pub fn push(&mut self, element: T) {
        self.data.push(element)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.last()
    }
}

#[test]
fn new_test() {
    let expected: Stack<i32> = Stack { data: vec![] };

    assert_eq!(Stack::new(), expected);
}

#[test]
fn push_test() {
    let mut stack: Stack<i32> = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    assert_eq!(stack.data, vec![1, 2, 3]);
}

#[test]
fn pop_test() {
    let mut stack: Stack<i32> = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None);
}

#[test]
fn peek_test() {
    let mut stack: Stack<i32> = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    assert_eq!(stack.peek(), Some(&3));
}
