fn main() {
    println!("Hello, world!");
}

fn insertion_sort<T: Ord>(slice: &mut [T]) {
    for i in 1..slice.len() {
        let mut j = i;
        while j > 0 && slice[j - 1] > slice[j] {
            slice.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut things = vec![4, 2, 5, 3, 1];
        insertion_sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4, 5]);
    }
}
