use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Vertex {
    value: String,
    adjacent_vertices: Vec<Rc<RefCell<Vertex>>>,
}

impl Vertex {
    pub fn new(value: String) -> Rc<RefCell<Vertex>> {
        Rc::new(RefCell::new(Vertex {
            value,
            adjacent_vertices: Vec::new(),
        }))
    }

    pub fn add_adjacent_vertex(&mut self, vertex: Rc<RefCell<Vertex>>) {
        if self.adjacent_vertices.contains(&vertex) {
            return;
        }

        self.adjacent_vertices.push(Rc::clone(&vertex));
    }
}

pub fn dfs_traverse(vertex: Rc<RefCell<Vertex>>) -> Vec<String> {
    let mut visited_verticies_set: HashSet<String> = HashSet::new();
    let mut visited_vertices_values: Vec<String> = Vec::new();
    _dfs_traverse(
        vertex,
        &mut visited_verticies_set,
        &mut visited_vertices_values,
    );

    visited_vertices_values
}

fn _dfs_traverse(
    vertex: Rc<RefCell<Vertex>>,
    visited_verticies_set: &mut HashSet<String>,
    visited_vertices_values: &mut Vec<String>,
) {
    let value = vertex.borrow().value.clone();

    visited_vertices_values.push(value.clone());
    visited_verticies_set.insert(value);

    for adjacent_vertex in &vertex.borrow().adjacent_vertices {
        let value = adjacent_vertex.borrow().value.clone();
        if !visited_verticies_set.contains(&value) {
            _dfs_traverse(
                adjacent_vertex.to_owned(),
                visited_verticies_set,
                visited_vertices_values,
            );
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_creates_a_new_vertex() {
        let new_vertex = Vertex::new("Alice".to_string());
        let expected = Rc::new(RefCell::new(Vertex {
            value: "Alice".to_string(),
            adjacent_vertices: vec![],
        }));

        assert_eq!(new_vertex, expected);
    }

    #[test]
    fn add_adjacent_vertex_creates_an_edge_from_a_vertex_towards_another_vertex() {
        let alice = Vertex::new("Alice".to_string());
        let cynthia = Vertex::new("Cynthia".to_string());
        let bob = Vertex::new("Bob".to_string());

        alice.borrow_mut().add_adjacent_vertex(bob.clone());
        alice.borrow_mut().add_adjacent_vertex(cynthia.clone());

        let bob_as_alice_adjacent_vertex = alice.borrow().adjacent_vertices[0].clone();
        let cynthia_as_alice_adjacent_vertex = alice.borrow().adjacent_vertices[1].clone();

        assert_eq!(bob, bob_as_alice_adjacent_vertex);
        assert_eq!(cynthia, cynthia_as_alice_adjacent_vertex);
    }

    #[test]
    fn add_adjacent_vertex_does_not_allow_inserting_more_than_once_the_same_vertex() {
        let alice = Vertex::new("Alice".to_string());
        let bob = Vertex::new("Bob".to_string());

        alice.borrow_mut().add_adjacent_vertex(bob.clone());
        alice.borrow_mut().add_adjacent_vertex(bob.clone());

        assert_eq!(1, alice.borrow().adjacent_vertices.len());
    }

    #[test]
    fn dfs_traverse_traverses_a_graph_using_depth_first_search() {
        let alice = Vertex::new("Alice".to_string());
        let bob = Vertex::new("Bob".to_string());
        let fred = Vertex::new("Fred".to_string());
        let helen = Vertex::new("Helen".to_string());
        let candy = Vertex::new("Candy".to_string());
        let derek = Vertex::new("Derek".to_string());
        let elaine = Vertex::new("Elaine".to_string());
        let gina = Vertex::new("Gina".to_string());
        let irena = Vertex::new("Irena".to_string());

        alice.borrow_mut().add_adjacent_vertex(bob.clone());
        alice.borrow_mut().add_adjacent_vertex(candy.clone());
        alice.borrow_mut().add_adjacent_vertex(derek.clone());
        alice.borrow_mut().add_adjacent_vertex(elaine.clone());

        bob.borrow_mut().add_adjacent_vertex(alice.clone());
        bob.borrow_mut().add_adjacent_vertex(fred.clone());

        fred.borrow_mut().add_adjacent_vertex(bob.clone());
        fred.borrow_mut().add_adjacent_vertex(helen.clone());

        helen.borrow_mut().add_adjacent_vertex(fred.clone());
        helen.borrow_mut().add_adjacent_vertex(candy.clone());

        candy.borrow_mut().add_adjacent_vertex(helen.clone());
        candy.borrow_mut().add_adjacent_vertex(alice.clone());

        derek.borrow_mut().add_adjacent_vertex(alice.clone());
        derek.borrow_mut().add_adjacent_vertex(gina.clone());
        derek.borrow_mut().add_adjacent_vertex(elaine.clone());

        gina.borrow_mut().add_adjacent_vertex(derek.clone());
        gina.borrow_mut().add_adjacent_vertex(irena.clone());

        irena.borrow_mut().add_adjacent_vertex(irena.clone());

        elaine.borrow_mut().add_adjacent_vertex(alice.clone());
        elaine.borrow_mut().add_adjacent_vertex(derek.clone());

        let result = dfs_traverse(alice);

        let expected = vec![
            "Alice", "Bob", "Fred", "Helen", "Candy", "Derek", "Gina", "Irena", "Elaine",
        ];

        assert_eq!(result, expected);
    }
}
