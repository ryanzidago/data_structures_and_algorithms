use std::cmp;


fn main() {
    println!("Hello, world!");
}

// Kadane's algorithm help us find the maximum subarray in an array
// time complexity: O(n)
// space complexity: O(1)
fn kadane(nums: Vec<i32>) -> i32 {
    let mut current_sum: i32 = nums[0];
    let mut max_sum: i32 = nums[0];

    for num in &nums[1..] {
        current_sum = cmp::max(*num, current_sum + *num);
        println!("num = {} current_sum {}", num, current_sum);
        max_sum = cmp::max(max_sum, current_sum);
    }

    return max_sum;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn kadane_test() {
        assert_eq!(6, kadane(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]));
        assert_eq!(1, kadane(vec![1]));
        assert_eq!(23, kadane(vec![5, 4, -1, 7, 8]));
        assert_eq!(1, kadane(vec![-2, 1]));
        assert_eq!(3, kadane(vec![1, 2, -5, 2]));
    }
}
