/*
In this challenge, you will implement a function find_first_even that takes a list of integers and returns the first even number in the list wrapped in Some. If no even number is present, the function should return None.

For example:
    If the input list is [1, 3, 5, 8, 10], the function should return Some(8).
    If the input list is [1, 3, 5], the function should return None.

Your task is to implement the function so it correctly handles any list of integers.
*/

pub fn find_first_even(numbers: &[i32]) -> Option<i32> {
    numbers.iter().copied().find(|n| n % 2 == 0)
}

// Example usage
pub fn main() {
    let nums1 = vec![1, 3, 5, 8];
    let nums2 = vec![1, 3, 5];

    println!("{:?}", find_first_even(&nums1)); // Output: Some(8)
    println!("{:?}", find_first_even(&nums2)); // Output: None
}
