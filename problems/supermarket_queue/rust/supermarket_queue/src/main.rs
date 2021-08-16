use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    println!("Hello, world!");
}

fn queue_time(customers: Vec<i32>, n: i32) -> i32 {
    let mut checkouts: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    for _ in 0..n {
        checkouts.push(Reverse(0));
    }

    for customer in customers {
        let Reverse(time) = checkouts.pop().unwrap();
        checkouts.push(Reverse(time + customer));
    }

    let checkouts = checkouts.into_sorted_vec();
    let Reverse(max_time_in_checkout) = checkouts.first().unwrap();
    *max_time_in_checkout
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn queue_time_test() {
        assert_eq!(0, queue_time(vec![], 1));
        assert_eq!(5, queue_time(vec![5], 1));
        assert_eq!(2, queue_time(vec![2], 5));
        assert_eq!(15, queue_time(vec![1, 2, 3, 4, 5], 1));
        assert_eq!(5, queue_time(vec![1, 2, 3, 4, 5], 100));
        assert_eq!(9, queue_time(vec![2, 2, 3, 3, 4, 4], 2));
        assert_eq!(
            106,
            queue_time(
                vec![45, 25, 2, 5, 46, 4, 50, 27, 34, 22, 12, 9, 29, 7, 1, 8, 49, 37, 22, 18],
                5
            )
        );
    }
}
