pub fn factorial(n: u32) -> u128 {
    // Implement your code here
    (1..=n).fold(1, |acc, x| acc * x as u128)
}
