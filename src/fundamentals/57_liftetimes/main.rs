/*
Write a function longest that takes two string slices and returns the longest one. If both slices are of the same length, return the first one.
*/

pub fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.chars().count() >= second.chars().count() {
        first
    } else {
        second
    }
}

// Example usage
pub fn main() {
    let s1 = "short";
    let s2 = "longer string";

    let result = longest(s1, s2);
    println!("The longest string is: {}", result);
    assert_eq!(result, "longer string");

    let s3 = "equal";
    let s4 = "equal";
    let result = longest(s3, s4);
    println!("The longest string is: {}", result);
    assert_eq!(result, "equal");
}
