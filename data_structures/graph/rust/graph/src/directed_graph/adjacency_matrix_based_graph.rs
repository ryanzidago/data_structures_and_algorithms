use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Vertex<T> {
    value: T,
    edges: HashMap<T, HashMap<T, T>>,
}

type Edge<T> = Rc<RefCell<Vertex<T>>>;

impl<T> Vertex<T> {
    pub fn new(value: T) -> Rc<RefCell<Vertex<T>>> {
        Rc::new(RefCell::new(Vertex {
            value,
            edges: ,
        }))
    }

    pub fn add_edge(&mut self, vertex: Rc<RefCell<Vertex<T>>>) {}
}

#[test]
fn new_test() {
    let expected: Rc<RefCell<Vertex<i32>>> = Rc::new(RefCell::new(Vertex { value: 1 }));

    assert_eq!(Vertex::new(1), expected);
}
