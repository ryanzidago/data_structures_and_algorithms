#[derive(Debug, PartialEq)]
pub struct Queue<T> {
    data: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { data: vec![] }
    }

    pub fn enqueue(&mut self, element: T) {
        self.data.push(element);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.data.remove(0))
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }
}

#[test]
fn new_test() {
    let expected: Queue<i32> = Queue { data: vec![] };

    assert_eq!(Queue::new(), expected);
}

#[test]
fn enqueue_test() {
    let mut queue: Queue<i32> = Queue::new();
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);

    assert_eq!(queue.data, vec![1, 2, 3]);
}

#[test]
fn dequeue_test() {
    let mut queue: Queue<i32> = Queue::new();
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);

    assert_eq!(Some(1), queue.dequeue());
    assert_eq!(Some(2), queue.dequeue());
    assert_eq!(Some(3), queue.dequeue());
    assert_eq!(None, queue.dequeue());
}

#[test]
fn peak_test() {
    let mut queue: Queue<i32> = Queue::new();
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);

    assert_eq!(queue.peek(), Some(&1));
}
