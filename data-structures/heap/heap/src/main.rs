fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Heap {
    data: Vec<i32>,
}

impl Heap {
    fn new() -> Heap {
        Heap { data: vec![] }
    }

    fn left_child_index(&self, index: usize) -> usize {
        (index * 2) + 1
    }

    fn right_child_index(&self, index: usize) -> usize {
        (index * 2) + 2
    }

    fn parent_index(&self, index: usize) -> usize {
        (index - 1) / 2
    }

    pub fn push(&mut self, value: i32) {
        self.data.push(value);

        let mut new_node_index: usize = self.data.len() - 1;
        while new_node_index > 0
            && self.data[new_node_index] > self.data[self.parent_index(new_node_index)]
        {
            let parent_index = self.parent_index(new_node_index);
            self.data.swap(parent_index, new_node_index);
            new_node_index = self.parent_index(new_node_index);
        }
    }

    pub fn pop(&mut self) -> i32 {
        let deleted_node = self.data[0];

        self.data[0] = self.data.pop().unwrap();

        let mut trickle_node_index = 0;

        while self.has_greater_child(trickle_node_index) {
            let larger_child_index = self.calculate_larger_child_index(trickle_node_index);
            self.data.swap(trickle_node_index, larger_child_index);
            trickle_node_index = larger_child_index;
        }

        deleted_node
    }

    fn has_greater_child(&self, index: usize) -> bool {
        let left_child_index: usize = self.left_child_index(index);
        let right_child_index: usize = self.right_child_index(index);

        self.data.get(left_child_index).is_some() && self.data[left_child_index] > self.data[index]
            || self.data.get(right_child_index).is_some()
                && self.data[right_child_index] > self.data[index]
    }

    fn calculate_larger_child_index(&self, index: usize) -> usize {
        let left_child_index: usize = self.left_child_index(index);
        let right_child_index: usize = self.right_child_index(index);

        let left_child: &i32 = self.data.get(left_child_index).unwrap_or(&0);
        let right_child: &i32 = self.data.get(right_child_index).unwrap_or(&0);

        if right_child > left_child {
            return right_child_index;
        } else {
            return left_child_index;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn push_test() {
        let mut heap: Heap = Heap::new();
        heap.push(3);
        heap.push(2);

        assert_eq!(vec![3, 2], heap.data);
    }

    #[test]
    fn root_node_is_always_the_biggest_element_in_heap_after_push_test() {
        let mut heap: Heap = Heap::new();
        heap.push(5);
        heap.push(10);
        heap.push(2);

        assert_eq!(10, heap.data[0]);

        heap.push(3);

        assert_eq!(10, heap.data[0]);

        heap.push(20);

        assert_eq!(20, heap.data[0]);
    }

    #[test]
    fn pop_always_pop_the_root_node() {
        let mut heap: Heap = Heap::new();
        heap.push(10);
        heap.push(4);
        heap.push(7);

        heap.pop();

        assert!(!heap.data.contains(&10));
    }

    #[test]
    fn root_node_is_always_the_biggest_element_in_heap_after_pop_test() {
        let mut heap: Heap = Heap::new();
        heap.push(5);
        heap.push(10);
        heap.push(2);

        heap.pop();

        assert_eq!(5, heap.data[0]);

        heap.pop();

        assert_eq!(2, heap.data[0]);
    }
}
