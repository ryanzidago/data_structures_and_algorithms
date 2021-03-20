pub fn quicksort(numbers: &mut Vec<i32>) {
    if numbers.len() <= 1 {
        return;
    }

    let left_index: isize = 0;
    let right_index: isize = (numbers.len() - 1) as isize;

    _quicksort(numbers, left_index, right_index);
}

fn _quicksort(numbers: &mut Vec<i32>, left_index: isize, right_index: isize) {
    if left_index >= right_index {
        return;
    }

    let pivot_index: isize = partition(numbers, left_index, right_index);
    _quicksort(numbers, left_index, pivot_index - 1);
    _quicksort(numbers, pivot_index + 1, right_index);
}

fn partition(numbers: &mut Vec<i32>, mut left_index: isize, mut right_index: isize) -> isize {
    let pivot_index: isize = right_index;
    right_index -= 1;

    loop {
        while numbers[left_index as usize] < numbers[pivot_index as usize] {
            left_index += 1
        }

        while numbers[right_index as usize] > numbers[pivot_index as usize] && right_index > 0 {
            right_index -= 1
        }

        if left_index >= right_index {
            break;
        } else {
            numbers.swap(left_index as usize, right_index as usize);
            left_index += 1;
        }
    }

    numbers.swap(left_index as usize, pivot_index as usize);
    return left_index;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn partition_test() {
        let mut input: Vec<i32> = vec![0, 5, 2, 1, 6, 3];
        let left_index: isize = 0;
        let right_index: isize = (input.len() - 1) as isize;
        let output: isize = partition(&mut input, left_index, right_index);

        assert_eq!(3, output);
    }

    #[test]
    fn quicksort_with_only_positive_integer_test() {
        let mut input: Vec<i32> = vec![23, 2, 2, 2, 0, 99];
        quicksort(&mut input);
        assert_eq!(vec![0, 2, 2, 2, 23, 99], input);
    }

    #[test]
    fn quicksort_with_zero_as_first_element_test() {
        let mut input: Vec<i32> = vec![0, 5, 2, 1, 6, 3];
        quicksort(&mut input);
        assert_eq!(vec![0, 1, 2, 3, 5, 6], input);

        let mut input: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
        quicksort(&mut input);
        assert_eq!(vec![0, 1, 2, 3, 4, 5], input);
    }

    #[test]
    fn quicksort_with_zero_as_last_element_test() {
        let mut input: Vec<i32> = vec![5, 2, 1, 6, 3, 0];
        quicksort(&mut input);
        assert_eq!(vec![0, 1, 2, 3, 5, 6], input);
    }

    #[test]
    fn quicksort_with_zero_as_second_to_last_element_test() {
        let mut input: Vec<i32> = vec![5, 2, 1, 6, 0, 3];
        quicksort(&mut input);
        assert_eq!(vec![0, 1, 2, 3, 5, 6], input);
    }

    #[test]
    fn quicksort_with_only_zeroes_test() {
        let mut input: Vec<i32> = vec![0, 0, 0, 0];
        quicksort(&mut input);
        assert_eq!(vec![0, 0, 0, 0], input);
    }

    #[test]
    fn quicksort_with_only_negative_integers_test() {
        let mut input: Vec<i32> = vec![-1, -2, 1];
        quicksort(&mut input);
        assert_eq!(vec![-2, -1, 1], input);
    }

    #[test]
    fn quicksort_with_both_positive_and_negative_integers_test() {
        let mut input: Vec<i32> = vec![0, -200, -100, 200, 10, 300];
        quicksort(&mut input);
        assert_eq!(vec![-200, -100, 0, 10, 200, 300], input);
    }

    #[test]
    fn quicksort_on_empty_vector_test() {
        let mut input: Vec<i32> = vec![];
        let expected: Vec<i32> = vec![];
        quicksort(&mut input);
        assert_eq!(expected, input);
    }

    #[test]
    fn quicksort_on_vector_with_only_one_element_test() {
        let mut input: Vec<i32> = vec![1];
        quicksort(&mut input);
        assert_eq!(vec![1], input);
    }

    #[test]
    fn quicksort_with_duplicate_values_test() {
        let mut input: Vec<i32> = vec![0, 0, 1, 5, 4, 2, 2, 1];
        quicksort(&mut input);
        assert_eq!(vec![0, 0, 1, 1, 2, 2, 4, 5], input);
    }

    #[test]
    fn quicksort_on_already_sorted_values_test() {
        let mut input: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        quicksort(&mut input);
        assert_eq!(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], input);
    }
}
