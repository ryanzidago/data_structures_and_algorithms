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
