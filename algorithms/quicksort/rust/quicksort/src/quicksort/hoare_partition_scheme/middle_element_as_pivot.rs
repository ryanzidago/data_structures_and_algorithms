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
    _quicksort(numbers, left_index, pivot_index);
    _quicksort(numbers, pivot_index + 1, right_index);
}

fn partition(numbers: &mut Vec<i32>, mut left_index: isize, mut right_index: isize) -> isize {
    // You should not compute the middle point as (left + right)/2.
    // If the array is very large (>2G)
    // then the result of "left + right" may overflow and become negative.
    // The proper way of choosing the midpoint pivot is: "left + (right-left)/2"
    // which is mathematically equivalent but immune to overflow as "right > left" is an invariant that always holds
    // and if "right" is representable then, "left" is also representable the result will never overflow
    // as it will be less than "right".
    // cf here: https://www.youtube.com/watch?v=SLauY6PpjW4
    let middle_index: usize = ((left_index + right_index) / 2) as usize;
    numbers.swap(right_index as usize, middle_index);

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
            numbers.swap(left_index as usize, pivot_index as usize);
            return left_index;
        } else {
            numbers.swap(left_index as usize, right_index as usize);
            left_index += 1;
        }
    }
}
