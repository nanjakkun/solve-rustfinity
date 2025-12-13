/*
Before starting to solve the challenge, try to run the code and see what error you'll get. This will help you understand the problem better.

The compile error tells us that we can't borrow s as mutable because it is also borrowed as immutable. If we borrow it as mutable, the value will change and the compiler can not guarantee that the immutable reference hasn't changed.

You are given a function calculate_and_modify that violates Rust's ownership rules. Your task is to fix this function by moving exactly one line of code to a different position.

Don't just try to make the tests pass - focus on understanding which single line needs to be moved and why!
*/

pub fn calculate_and_modify() -> (String, usize) {
    let mut s = String::from("hello");
    let length = s.len();

    s.push_str(", world");
    let s2 = &s;

    println!("{}", s2);

    (s, length)
}
