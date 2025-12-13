/*
    Use a while loop to implement the countdown.
    Ensure the function handles the case where n is 0 correctly.
    The function should return a vector with each number in the countdown sequence.
*/

pub fn countdown(n: u32) -> Vec<u32> {
    let mut result = vec![n];
    let mut i = n;

    while i > 0 {
        i -= 1;
        result.push(i);
    }
    result
}
