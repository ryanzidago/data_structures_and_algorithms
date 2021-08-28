use std::cmp::Ordering;

#[allow(dead_code)]
fn binary_search_iterative(vector: &Vec<i32>, target: i32) -> i32 {
    let mut left: i32 = 0;
    let mut right: i32 = (vector.len() - 1) as i32;

    while left <= right {
        let pivot = left + (right - left) / 2;

        match vector[pivot as usize].cmp(&target) {
            Ordering::Less => left = pivot + 1,
            Ordering::Equal => return pivot as i32,
            Ordering::Greater => right = pivot - 1,
        }
    }
    -1
}

#[allow(dead_code)]
fn binary_search_recursive(vector: &Vec<i32>, target: i32) -> i32 {
    let left: i32 = 0;
    let right: i32 = (vector.len() - 1) as i32;
    _binary_search_recursive(vector, target, left, right)
}

fn _binary_search_recursive(vector: &Vec<i32>, target: i32, left: i32, right: i32) -> i32 {
    if left > right {
        return -1;
    }
    let pivot = left + (right - left) / 2;
    match vector[pivot as usize].cmp(&target) {
        Ordering::Less => _binary_search_recursive(vector, target, pivot + 1, right),
        Ordering::Equal => pivot as i32,
        Ordering::Greater => _binary_search_recursive(vector, target, left, pivot - 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn binary_search_iterative_test() {
        let vector = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;

        assert_eq!(binary_search_iterative(&vector, target), 4);
    }

    #[test]
    fn binary_search_recursive_test() {
        let vector = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;

        assert_eq!(binary_search_recursive(&vector, target), 4);

        let vector = vec![1, 2, 3, 4, 5];
        let target = 6;

        assert_eq!(binary_search_recursive(&vector, target), -1);
    }
}
