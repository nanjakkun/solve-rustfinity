/*
Your task is to implement a function, is_prime(n: u32) -> bool, that takes a number as input and returns true if the number is prime, and false otherwise.
Requirements

Using Eratosthenes sieve
*/
pub fn is_prime(n: u32) -> bool {
    // Implement your code here
    if n <= 1 {
        return false;
    }
    let mut sieve = vec![true; n as usize + 1];
    let limit = (n as f64).sqrt() as u32;
    for i in 2..=limit {
        if sieve[i as usize] {
            for j in (i * i..=n).step_by(i as usize) {
                sieve[j as usize] = false;
            }
        }
    }
    sieve[n as usize]
}
