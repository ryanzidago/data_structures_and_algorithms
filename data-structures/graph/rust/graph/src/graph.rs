use std::cell::RefCell;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::rc::Rc;

// Graph with adjacency list
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

struct Graph {}

impl Graph {
    // Algorithm:
    // 1. From the starting vertex within the graph
    // 2. Add the current vertex to the HashSet, to mark it as being visited
    // 3. Iterate through the current vertex's adjacent vertices
    // 4. for each adjacent vertex, ignore the adjacent vertex if it has already been visited
    // otherwise, recursively perform depth-first search on the adjacent vertex

    // Time Complexity:
    // O(V + E)
    pub fn dfs_traverse(vertex: Rc<RefCell<Vertex>>) -> Vec<String> {
        let mut visited_vertices_set: HashSet<String> = HashSet::new();
        let mut visited_vertices_values: Vec<String> = Vec::new();
        Graph::_dfs_traverse(
            vertex,
            &mut visited_vertices_set,
            &mut visited_vertices_values,
        );

        visited_vertices_values
    }

    fn _dfs_traverse(
        vertex: Rc<RefCell<Vertex>>,
        visited_vertices_set: &mut HashSet<String>,
        visited_vertices_values: &mut Vec<String>,
    ) {
        let value = vertex.borrow().value.clone();

        visited_vertices_values.push(value.clone());
        visited_vertices_set.insert(value);

        for adjacent_vertex in &vertex.borrow().adjacent_vertices {
            let value = adjacent_vertex.borrow().value.clone();
            if !visited_vertices_set.contains(&value) {
                Graph::_dfs_traverse(
                    adjacent_vertex.to_owned(),
                    visited_vertices_set,
                    visited_vertices_values,
                );
            }
        }
    }

    pub fn dfs(vertex: Rc<RefCell<Vertex>>, searched_value: String) -> Option<Rc<RefCell<Vertex>>> {
        let mut visited_vertices_set: HashSet<String> = HashSet::new();
        Graph::_dfs(Some(vertex), searched_value, &mut visited_vertices_set)
    }

    fn _dfs(
        vertex: Option<Rc<RefCell<Vertex>>>,
        searched_value: String,
        visited_vertices_set: &mut HashSet<String>,
    ) -> Option<Rc<RefCell<Vertex>>> {
        if let Some(vertex) = vertex {
            if vertex.borrow().value == searched_value {
                return Some(vertex);
            }

            let value = vertex.borrow().value.clone();
            visited_vertices_set.insert(value.clone());

            for adjacent_vertex in &vertex.borrow().adjacent_vertices {
                if !visited_vertices_set.contains(&value) {
                    if adjacent_vertex.borrow().value == searched_value {
                        return Some(adjacent_vertex.to_owned());
                    } else {
                        return Graph::_dfs(
                            Some(adjacent_vertex.to_owned()),
                            searched_value,
                            visited_vertices_set,
                        );
                    }
                }
            }
        }

        return None;
    }

    // Algorithm:
    // 1. Start from a given vertex within the graph
    // 2. Add the vertex to the HashSet to mark it has being visited
    // 3. add the vertex to a queue
    // 4. for as long as the queue is not empty
    // 5. remove the first vertex from the queue (this vertex becomes the current vertex)
    // 6. iterate over the current vertex's adjacent vertices
    // 7. ignore the adjacent vertex that has already been visited
    // 8. if the adjacent vertex has not been visited, mark it as visited by putting it in the HashSet and add it to the queue
    // 9. repeat the loop from step 4 until the queue is empty

    // Time Complexity:
    // O(V + E)
    pub fn bfs_traverse(starting_vertex: Rc<RefCell<Vertex>>) -> Vec<String> {
        let mut queue: VecDeque<Rc<RefCell<Vertex>>> = VecDeque::new();
        let mut visited_vertices_set: HashSet<String> = HashSet::new();
        let mut visited_vertices_values: Vec<String> = Vec::new();

        visited_vertices_set.insert(starting_vertex.borrow().value.clone());
        visited_vertices_values.push(starting_vertex.borrow().value.clone());
        queue.push_back(Rc::clone(&starting_vertex));

        while !queue.is_empty() {
            if let Some(current_vertex) = queue.pop_front() {
                for adjacent_vertex in current_vertex.borrow().adjacent_vertices.clone() {
                    let adjacent_vertex_value = adjacent_vertex.borrow().value.clone();
                    if !visited_vertices_set.contains(&adjacent_vertex_value) {
                        visited_vertices_set.insert(adjacent_vertex_value.clone());
                        visited_vertices_values.push(adjacent_vertex_value.clone());
                        queue.push_back(Rc::clone(&adjacent_vertex));
                    }
                }
            }
        }

        visited_vertices_values
    }

    pub fn bfs(
        starting_vertex: Rc<RefCell<Vertex>>,
        searched_value: String,
    ) -> Option<Rc<RefCell<Vertex>>> {
        let mut visited_vertices_set: HashSet<String> = HashSet::new();
        let mut queue: VecDeque<Rc<RefCell<Vertex>>> = VecDeque::new();

        visited_vertices_set.insert(starting_vertex.borrow().value.clone());
        queue.push_back(starting_vertex);

        while !queue.is_empty() {
            if let Some(current_vertex) = queue.pop_front() {
                if current_vertex.borrow().value == searched_value {
                    return Some(current_vertex);
                } else {
                    for adjacent_vertex in current_vertex.borrow().adjacent_vertices.clone() {
                        let adjacent_vertex_value = adjacent_vertex.borrow().value.clone();
                        if !visited_vertices_set.contains(&adjacent_vertex_value) {
                            visited_vertices_set.insert(adjacent_vertex_value.clone());
                            queue.push_back(adjacent_vertex);
                        }
                    }
                }
            }
        }

        return None;
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

        let result = Graph::dfs_traverse(alice);

        let expected = vec![
            "Alice", "Bob", "Fred", "Helen", "Candy", "Derek", "Gina", "Irena", "Elaine",
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn dfs_searches_for_a_vertex_by_its_value_and_returns_an_option() {
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

        if let Some(found) = Graph::dfs(alice, "Irena".to_string()) {
            assert_eq!(found, irena);
        }

        assert!(Graph::dfs(bob, "Martin".to_string()).is_none());
    }

    #[test]
    fn bfs_traverse_traverses_a_graph_using_breadth_first_search_and_returns_a_vector_of_the_vertices_values(
    ) {
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

        let result = Graph::bfs_traverse(alice);
        let expected = vec![
            "Alice".to_string(),
            "Bob".to_string(),
            "Candy".to_string(),
            "Derek".to_string(),
            "Elaine".to_string(),
            "Fred".to_string(),
            "Helen".to_string(),
            "Gina".to_string(),
            "Irena".to_string(),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn bfs_searches_for_a_vertex_by_its_value_and_returns_an_option() {
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

        if let Some(found) = Graph::dfs(alice, "Irena".to_string()) {
            assert_eq!(found, irena);
        }

        assert!(Graph::dfs(bob, "Martin".to_string()).is_none());
    }
}
