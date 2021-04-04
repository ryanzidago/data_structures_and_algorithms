fn main() {}

fn bubble_sort(array: &mut Vec<i32>) {
    if array.len() <= 1 {
        return;
    }

    for i in 0..array.len() {
        for j in (i + 1)..array.len() {
            if array[i] > array[j] {
                array.swap(i, j);
            }
        }
    }
}

fn bubble_sort_bis(array: &mut Vec<i32>) {
    if array.len() <= 1 {
        return;
    }

    let mut unsorted_until_index = (array.len() - 1) as usize;
    let mut sorted = false;

    while !sorted {
        sorted = true;
        for i in 0..unsorted_until_index {
            if array[i as usize] > array[(i + 1) as usize] {
                array.swap(i as usize, (i + 1) as usize);
                sorted = false;
            }
        }
        unsorted_until_index -= 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bubble_sort_test() {
        let mut input: Vec<i32> = vec![3, 1, 4, 2, 6, 5];
        bubble_sort_bis(&mut input);
        assert_eq!(vec![1, 2, 3, 4, 5, 6], input);
    }

    #[test]
    fn bubble_sort_with_only_positive_integer_test() {
        let mut input: Vec<i32> = vec![23, 2, 2, 2, 0, 99];
        bubble_sort(&mut input);
        assert_eq!(vec![0, 2, 2, 2, 23, 99], input);
    }

    #[test]
    fn bubble_sort_with_zero_as_first_element_test() {
        let mut input: Vec<i32> = vec![0, 5, 2, 1, 6, 3];
        bubble_sort(&mut input);
        assert_eq!(vec![0, 1, 2, 3, 5, 6], input);

        let mut input: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
        bubble_sort(&mut input);
        assert_eq!(vec![0, 1, 2, 3, 4, 5], input);
    }

    #[test]
    fn bubble_sort_with_zero_as_last_element_test() {
        let mut input: Vec<i32> = vec![5, 2, 1, 6, 3, 0];
        bubble_sort(&mut input);
        assert_eq!(vec![0, 1, 2, 3, 5, 6], input);
    }

    #[test]
    fn bubble_sort_with_zero_as_second_to_last_element_test() {
        let mut input: Vec<i32> = vec![5, 2, 1, 6, 0, 3];
        bubble_sort(&mut input);
        assert_eq!(vec![0, 1, 2, 3, 5, 6], input);
    }

    #[test]
    fn bubble_sort_with_only_zeroes_test() {
        let mut input: Vec<i32> = vec![0, 0, 0, 0];
        bubble_sort(&mut input);
        assert_eq!(vec![0, 0, 0, 0], input);
    }

    #[test]
    fn bubble_sort_with_only_negative_integers_test() {
        let mut input: Vec<i32> = vec![-1, -2, 1];
        bubble_sort(&mut input);
        assert_eq!(vec![-2, -1, 1], input);
    }

    #[test]
    fn bubble_sort_with_both_positive_and_negative_integers_test() {
        let mut input: Vec<i32> = vec![0, -200, -100, 200, 10, 300];
        bubble_sort(&mut input);
        assert_eq!(vec![-200, -100, 0, 10, 200, 300], input);
    }

    #[test]
    fn bubble_sort_on_empty_vector_test() {
        let mut input: Vec<i32> = vec![];
        let expected: Vec<i32> = vec![];
        bubble_sort(&mut input);
        assert_eq!(expected, input);
    }

    #[test]
    fn bubble_sort_on_vector_with_only_one_element_test() {
        let mut input: Vec<i32> = vec![1];
        bubble_sort(&mut input);
        assert_eq!(vec![1], input);
    }

    #[test]
    fn bubble_sort_with_duplicate_values_test() {
        let mut input: Vec<i32> = vec![0, 0, 1, 5, 4, 2, 2, 1];
        bubble_sort(&mut input);
        assert_eq!(vec![0, 0, 1, 1, 2, 2, 4, 5], input);
    }
}
