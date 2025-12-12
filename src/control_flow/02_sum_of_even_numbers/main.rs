/*
Your task is to implement a function sum_of_evens that takes two arguments, start and end, both of which are i32. The function should return the sum of all even numbers within the range [start, end] inclusive. If start is greater than end, the function should return 0.
*/

pub fn sum_of_evens(start: i32, end: i32) -> i32 {
    if start > end {
        return 0;
    }
    (start..=end).filter(|x| x % 2 == 0).sum()
}
