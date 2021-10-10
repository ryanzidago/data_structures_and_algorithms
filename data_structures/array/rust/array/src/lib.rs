#[allow(dead_code)]
fn rotate_left_build_in_function(array: &mut Vec<i32>, k: i32) {
    let k = (k as usize) % array.len();
    array.rotate_left(k);
}

#[allow(dead_code)]
fn rotate_right_build_in_function(array: &mut Vec<i32>, k: i32) {
    let k = (k as usize) % array.len();
    array.rotate_right(k);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_left_build_in_function_test() {
        let mut array = vec![1, 2, 3];
        let k = 1;

        rotate_left_build_in_function(&mut array, k);
        assert_eq!(array, [2, 3, 1]);
    }

    #[test]
    fn rotate_right_build_in_function_test() {
        let mut array = vec![1, 2, 3];
        let k = 1;

        rotate_right_build_in_function(&mut array, k);
        assert_eq!(array, [3, 1, 2]);
    }
}
