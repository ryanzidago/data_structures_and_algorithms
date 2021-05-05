fn main() {
    let mut array = vec![4, 2, 7, 1, 3];
    selection_sort(&mut array);
    println!("{:?}", array);
}

fn selection_sort(array: &mut Vec<i32>) {
    for i in 0..array.len() - 1 {
        let mut lowest_number_index = i;
        for j in (i + 1)..array.len() {
            if array[j] < array[lowest_number_index] {
                lowest_number_index = j;
            }
        }

        if lowest_number_index != i {
            array.swap(i, lowest_number_index);
        }
    }
}
