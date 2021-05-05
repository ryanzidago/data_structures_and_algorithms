fn main() {
    println!("Hello, world!");
}

mod binary_heap {
    #[derive(Debug)]
    pub struct MaxHeap<T> {
        pub data: Vec<T>,
    }

    impl<T> MaxHeap<T>
    where
        T: PartialOrd,
    {
        pub fn new() -> MaxHeap<T> {
            MaxHeap { data: vec![] }
        }

        // Binary heap insertion

        // Algorithm:
        // 1. Insert the value at the end of the heap tree (next available rightmost spot in the bottom level of the heap tree)
        // 2. Compare the inserted node with its parent node
        // 3. If the inserted node is greater than its parent, swap the inserted node with the parent
        // 4. Repeat step 2 and 3 until the newly inserted node has no parent whose value is greater than it

        // Time complexity:
        // O(log N)
        // At most we would need to go through all the heap tree's row.
        // A binary tree of size N has roughly O(log N) rows
        // So deleting a node from a heap is at most O(log N)

        pub fn push(&mut self, value: T) {
            self.data.push(value);
            let new_node_index: usize = self.data.len() - 1;
            self.sift_up(new_node_index);
        }

        // Binary heap deletion

        // Algorithm:
        // 1. Remove the root node (i.e. the node with the highest value in a max heap)
        // 2. Replace the root node with the heap's last node (which will be called the sifted down node)
        // 3. Compare the sifted down node with the larger of the sifted down node's two children
        // 4. If the sifted down node is smaller than the larger of the two child nodes, swap the sifted node with that larger child
        // 5. Repeat steps 3 and 4 until the sifted down node has no children who are greater than it

        // Time complexity:
        // O(log N)
        // At most we would need to go through all the heap tree's row.
        // A binary tree of size N has roughly O(log N) rows
        // So deleting a node from a heap is at most O(log N)
        pub fn pop(&mut self) -> Option<T> {
            match self.data.len() {
                0 => None,
                _ => {
                    let deleted_node = self.data.swap_remove(0);
                    self.sift_down();
                    Some(deleted_node)
                }
            }
        }

        fn sift_up(&mut self, mut new_node_index: usize) {
            while !self.is_root(new_node_index) && self.is_greater_than_parent(new_node_index) {
                let parent_index = self.parent_index(new_node_index);
                self.data.swap(parent_index, new_node_index);
                new_node_index = self.parent_index(new_node_index);
            }
        }

        fn is_root(&self, node_index: usize) -> bool {
            node_index == 0
        }

        fn is_greater_than_parent(&self, node_index: usize) -> bool {
            let parent_index = self.parent_index(node_index);
            self.data[node_index] > self.data[parent_index]
        }

        fn sift_down(&mut self) {
            let mut sifted_down_node_index: usize = 0;

            while self.has_greater_child(sifted_down_node_index) {
                let larger_child_index = self.calculate_larger_child_index(sifted_down_node_index);
                self.data.swap(sifted_down_node_index, larger_child_index);
                sifted_down_node_index = larger_child_index;
            }
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

        fn has_greater_child(&self, index: usize) -> bool {
            let left_child_index: usize = self.left_child_index(index);
            let right_child_index: usize = self.right_child_index(index);

            self.data.get(left_child_index).is_some()
                && self.data[left_child_index] > self.data[index]
                || self.data.get(right_child_index).is_some()
                    && self.data[right_child_index] > self.data[index]
        }

        fn calculate_larger_child_index(&self, index: usize) -> usize {
            let left_child_index: usize = self.left_child_index(index);
            let right_child_index: usize = self.right_child_index(index);

            let left_child = self.data.get(left_child_index);
            let right_child = self.data.get(right_child_index);

            if ((right_child.is_some() && left_child.is_some()) && right_child > left_child)
                || left_child.is_none()
            {
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
            let mut heap: MaxHeap<i32> = MaxHeap::new();
            heap.push(3);
            heap.push(2);

            assert_eq!(vec![3, 2], heap.data);
        }

        #[test]
        fn root_node_is_always_the_biggest_element_in_heap_after_push_test() {
            let mut heap: MaxHeap<i32> = MaxHeap::new();
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
            let mut heap: MaxHeap<i32> = MaxHeap::new();
            heap.push(10);
            heap.push(4);
            heap.push(7);

            heap.pop();

            assert!(!heap.data.contains(&10));
        }

        #[test]
        fn pop_returns_a_variant_of_the_option_enum() {
            let mut heap: MaxHeap<i32> = MaxHeap::new();
            heap.push(10);

            assert_eq!(Some(10), heap.pop());
            assert_eq!(None, heap.pop());
        }

        #[test]
        fn root_node_is_always_the_biggest_element_in_heap_after_pop_test() {
            let mut heap: MaxHeap<i32> = MaxHeap::new();
            heap.push(5);
            heap.push(10);
            heap.push(2);

            heap.pop();

            assert_eq!(5, heap.data[0]);

            heap.pop();

            assert_eq!(2, heap.data[0]);
        }

        #[test]
        fn heap_is_generic_over_some_type_t() {
            let mut heap: MaxHeap<(i32, String)> = MaxHeap::new();
            heap.push((2, String::from("Zanzibar")));
            heap.push((10, String::from("Porto")));
            heap.push((5, String::from("Beijing")));

            let element = heap.pop();

            assert_eq!(Some((10, String::from("Porto"))), element);
            assert_eq!((5, String::from("Beijing")), heap.data[0]);
        }
    }
}
