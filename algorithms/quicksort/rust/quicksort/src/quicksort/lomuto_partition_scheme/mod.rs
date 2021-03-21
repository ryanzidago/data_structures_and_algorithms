pub fn quicksort(numbers: &mut Vec<i32>) {
    if numbers.len() <= 1 {
        return;
    }

    let low: isize = 0;
    let high: isize = (numbers.len() - 1) as isize;
    _quicksort(numbers, low, high);
}

// Note that in Lomuto's partition scheme,
// the pivot is not included
fn _quicksort(numbers: &mut Vec<i32>, low: isize, high: isize) {
    if low < high {
        let pivot = partition(numbers, low, high);
        _quicksort(numbers, low, pivot - 1);
        _quicksort(numbers, pivot + 1, high);
    }
}

fn partition(numbers: &mut Vec<i32>, mut low: isize, high: isize) -> isize {
    let pivot: i32 = numbers[high as usize];

    for j in low..high {
        if numbers[j as usize] < pivot {
            numbers.swap(low as usize, j as usize);
            low += 1;
        }
    }

    numbers.swap(low as usize, high as usize);
    return low;
}
