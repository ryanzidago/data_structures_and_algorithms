pub mod quicksort;

enum LomutoPartitionScheme {}
enum HoarePartitionSchemeLeftmostPivot {}
enum HoarePartitionSchemeRightmostPivot {}

trait Quicksort {
    fn quicksort(numbers: &mut Vec<i32>);
}

impl Quicksort for LomutoPartitionScheme {
    fn quicksort(numbers: &mut Vec<i32>) {
        quicksort::lomuto_partition_scheme::quicksort(numbers);
    }
}

impl Quicksort for HoarePartitionSchemeLeftmostPivot {
    fn quicksort(numbers: &mut Vec<i32>) {
        quicksort::hoare_partition_scheme::leftmost_element_as_pivot::quicksort(numbers);
    }
}

impl Quicksort for HoarePartitionSchemeRightmostPivot {
    fn quicksort(numbers: &mut Vec<i32>) {
        quicksort::hoare_partition_scheme::rightmost_element_as_pivot::quicksort(numbers);
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quicksort_with_only_positive_integer_test() {
        _quicksort_with_only_positive_integer_test::<LomutoPartitionScheme>();
        _quicksort_with_only_positive_integer_test::<HoarePartitionSchemeLeftmostPivot>();
        _quicksort_with_only_positive_integer_test::<HoarePartitionSchemeRightmostPivot>();
    }

    fn _quicksort_with_only_positive_integer_test<T: Quicksort>() {
        let mut input: Vec<i32> = vec![23, 2, 2, 2, 0, 99];
        T::quicksort(&mut input);
        assert_eq!(vec![0, 2, 2, 2, 23, 99], input);
    }

    #[test]
    fn quicksort_with_zero_as_first_element_test() {
        _quicksort_with_zero_as_first_element_test::<LomutoPartitionScheme>();
        _quicksort_with_zero_as_first_element_test::<HoarePartitionSchemeLeftmostPivot>();
        _quicksort_with_zero_as_first_element_test::<HoarePartitionSchemeRightmostPivot>();
    }

    fn _quicksort_with_zero_as_first_element_test<T: Quicksort>() {
        let mut input: Vec<i32> = vec![0, 5, 2, 1, 6, 3];
        T::quicksort(&mut input);
        assert_eq!(vec![0, 1, 2, 3, 5, 6], input);

        let mut input: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
        T::quicksort(&mut input);
        assert_eq!(vec![0, 1, 2, 3, 4, 5], input);
    }

    #[test]
    fn quicksort_with_zero_as_last_element_test() {
        _quicksort_with_zero_as_last_element_test::<LomutoPartitionScheme>();
        _quicksort_with_zero_as_last_element_test::<HoarePartitionSchemeLeftmostPivot>();
        _quicksort_with_zero_as_last_element_test::<HoarePartitionSchemeRightmostPivot>();
    }

    fn _quicksort_with_zero_as_last_element_test<T: Quicksort>() {
        let mut input: Vec<i32> = vec![5, 2, 1, 6, 3, 0];
        T::quicksort(&mut input);
        assert_eq!(vec![0, 1, 2, 3, 5, 6], input);
    }

    #[test]
    fn quicksort_with_zero_as_second_to_last_element_test() {
        _quicksort_with_zero_as_second_to_last_element_test::<LomutoPartitionScheme>();
        _quicksort_with_zero_as_second_to_last_element_test::<HoarePartitionSchemeLeftmostPivot>();
        _quicksort_with_zero_as_second_to_last_element_test::<HoarePartitionSchemeRightmostPivot>();
    }

    fn _quicksort_with_zero_as_second_to_last_element_test<T: Quicksort>() {
        let mut input: Vec<i32> = vec![5, 2, 1, 6, 0, 3];
        T::quicksort(&mut input);
        assert_eq!(vec![0, 1, 2, 3, 5, 6], input);
    }

    #[test]
    fn quicksort_with_only_zeroes_test() {
        _quicksort_with_only_zeroes_test::<LomutoPartitionScheme>();
        _quicksort_with_only_zeroes_test::<HoarePartitionSchemeLeftmostPivot>();
        _quicksort_with_only_zeroes_test::<HoarePartitionSchemeRightmostPivot>();
    }

    fn _quicksort_with_only_zeroes_test<T: Quicksort>() {
        let mut input: Vec<i32> = vec![0, 0, 0, 0];
        T::quicksort(&mut input);
        assert_eq!(vec![0, 0, 0, 0], input);
    }

    #[test]
    fn quicksort_with_only_negative_integers_test() {
        _quicksort_with_only_negative_integers_test::<LomutoPartitionScheme>();
        _quicksort_with_only_negative_integers_test::<HoarePartitionSchemeLeftmostPivot>();
        _quicksort_with_only_negative_integers_test::<HoarePartitionSchemeRightmostPivot>();
    }

    fn _quicksort_with_only_negative_integers_test<T: Quicksort>() {
        let mut input: Vec<i32> = vec![-1, -2, 1];
        T::quicksort(&mut input);
        assert_eq!(vec![-2, -1, 1], input);
    }

    #[test]
    fn quicksort_with_both_positive_and_negative_integers_test() {
        _quicksort_with_both_positive_and_negative_integers_test::<LomutoPartitionScheme>();
        _quicksort_with_both_positive_and_negative_integers_test::<HoarePartitionSchemeLeftmostPivot>(
        );
        _quicksort_with_both_positive_and_negative_integers_test::<
            HoarePartitionSchemeRightmostPivot,
        >();
    }

    fn _quicksort_with_both_positive_and_negative_integers_test<T: Quicksort>() {
        let mut input: Vec<i32> = vec![0, -200, -100, 200, 10, 300];
        T::quicksort(&mut input);
        assert_eq!(vec![-200, -100, 0, 10, 200, 300], input);
    }

    #[test]
    fn quicksort_on_empty_vector_test() {
        _quicksort_on_empty_vector_test::<LomutoPartitionScheme>();
        _quicksort_on_empty_vector_test::<HoarePartitionSchemeLeftmostPivot>();
        _quicksort_on_empty_vector_test::<HoarePartitionSchemeRightmostPivot>();
    }

    fn _quicksort_on_empty_vector_test<T: Quicksort>() {
        let mut input: Vec<i32> = vec![];
        let expected: Vec<i32> = vec![];
        T::quicksort(&mut input);
        assert_eq!(expected, input);
    }

    #[test]
    fn quicksort_on_vector_with_only_one_element_test() {
        _quicksort_on_vector_with_only_one_element_test::<LomutoPartitionScheme>();
        _quicksort_on_vector_with_only_one_element_test::<HoarePartitionSchemeLeftmostPivot>();
        _quicksort_on_vector_with_only_one_element_test::<HoarePartitionSchemeRightmostPivot>();
    }

    fn _quicksort_on_vector_with_only_one_element_test<T: Quicksort>() {
        let mut input: Vec<i32> = vec![1];
        T::quicksort(&mut input);
        assert_eq!(vec![1], input);
    }

    #[test]
    fn quicksort_with_duplicate_values_test() {
        _quicksort_with_duplicate_values_test::<LomutoPartitionScheme>();
        _quicksort_with_duplicate_values_test::<HoarePartitionSchemeLeftmostPivot>();
        _quicksort_with_duplicate_values_test::<HoarePartitionSchemeRightmostPivot>();
    }

    fn _quicksort_with_duplicate_values_test<T: Quicksort>() {
        let mut input: Vec<i32> = vec![0, 0, 1, 5, 4, 2, 2, 1];
        T::quicksort(&mut input);
        assert_eq!(vec![0, 0, 1, 1, 2, 2, 4, 5], input);

        let mut input: Vec<i32> = vec![-3, -3, 2, 10, 5, 1, 1];
        T::quicksort(&mut input);
        assert_eq!(vec![-3, -3, 1, 1, 2, 5, 10], input);

        let mut input: Vec<i32> = vec![0, 0, 0, 0, 0, 0];
        T::quicksort(&mut input);
        assert_eq!(vec![0, 0, 0, 0, 0, 0], input);
    }

    #[test]
    fn quicksort_on_already_sorted_values_test() {
        _quicksort_on_already_sorted_values_test::<LomutoPartitionScheme>();
        _quicksort_on_already_sorted_values_test::<HoarePartitionSchemeLeftmostPivot>();
        _quicksort_on_already_sorted_values_test::<HoarePartitionSchemeRightmostPivot>();
    }

    fn _quicksort_on_already_sorted_values_test<T: Quicksort>() {
        let mut input: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        T::quicksort(&mut input);
        assert_eq!(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], input);
    }
}
